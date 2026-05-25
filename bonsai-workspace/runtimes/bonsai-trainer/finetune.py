#!/usr/bin/env python3
"""
BonsAI-Core LoRA fine-tune.

Device priority: CUDA -> DirectML fp32 (AMD/Intel Windows) -> MPS -> CPU.

--gguf: accepts a local GGUF path.  GGUF is a quantized inference format —
        gradient-based LoRA training requires float weights.  This flag reads
        the GGUF header to identify the source architecture and maps it to a
        locally-cached HuggingFace model.  No download is attempted; if the
        matching model is not in the HF cache the run aborts with a clear error.

Usage:
    py finetune.py --gguf D:/Models/general/Bonsai-1.7B-Q2_K/Bonsai-1.7B-Q2_K.gguf \
                   --data data/bonsai_core/bonsai_core_train_v2.jsonl \
                   --output ~/.bonsai/adapters/bonsai-core-v3 \
                   --epochs 3

    py finetune.py --base-model Qwen/Qwen2.5-0.5B-Instruct \
                   --backend directml --precision fp32 \
                   --data data/bonsai_core/bonsai_core_train_v2.jsonl \
                   --output ~/.bonsai/adapters/bonsai-core-v3
"""
import argparse, json, os, shutil, struct, time
from pathlib import Path

import torch

# ── Progress helpers ──────────────────────────────────────────────────────────

_TRAIN_START: float = 0.0

def emit(tag: str, **kw):
    """Print a structured progress line that the UI can parse."""
    print(f"[{tag}] " + " ".join(f"{k}={v}" for k, v in kw.items()), flush=True)

# ── GGUF header parser ────────────────────────────────────────────────────────

GGUF_MAGIC = b"GGUF"

# GGUF key names that carry the model family/name
_ARCH_KEYS = ("general.architecture", "general.name")

# Map GGUF architecture strings → HuggingFace model IDs (local cache only)
_ARCH_TO_HF: dict[str, list[str]] = {
    "qwen2":  ["Qwen/Qwen2.5-1.5B-Instruct", "Qwen/Qwen2.5-0.5B-Instruct"],
    "qwen3":  ["Qwen/Qwen2.5-1.5B-Instruct", "Qwen/Qwen2.5-0.5B-Instruct"],
    "llama":  ["meta-llama/Llama-3.2-1B-Instruct"],
    "mistral":["mistralai/Mistral-7B-Instruct-v0.3"],
}


def _read_gguf_string(f) -> str:
    length = struct.unpack("<Q", f.read(8))[0]
    return f.read(length).decode("utf-8", errors="replace")


def _parse_gguf_arch(gguf_path: str) -> str | None:
    """Return the architecture string from a GGUF header, or None."""
    try:
        with open(gguf_path, "rb") as f:
            if f.read(4) != GGUF_MAGIC:
                return None
            _version = struct.unpack("<I", f.read(4))[0]
            n_tensors = struct.unpack("<Q", f.read(8))[0]
            n_kv = struct.unpack("<Q", f.read(8))[0]
            for _ in range(min(n_kv, 256)):
                key = _read_gguf_string(f)
                vtype = struct.unpack("<I", f.read(4))[0]
                if vtype == 8:  # GGUF_TYPE_STRING
                    val = _read_gguf_string(f)
                    if key in _ARCH_KEYS:
                        return val.lower().split("-")[0]
                    # skip other types by reading fixed sizes
                elif vtype == 0:  f.read(1)
                elif vtype == 1:  f.read(1)
                elif vtype == 2:  f.read(2)
                elif vtype == 3:  f.read(2)
                elif vtype == 4:  f.read(4)
                elif vtype == 5:  f.read(4)
                elif vtype == 6:  f.read(4)
                elif vtype == 7:  f.read(8)
                elif vtype == 9:  f.read(8)
                elif vtype == 10: f.read(1)
                elif vtype == 11: f.read(8)
                elif vtype == 12:  # array — skip
                    atype = struct.unpack("<I", f.read(4))[0]
                    alen  = struct.unpack("<Q", f.read(8))[0]
                    break  # arrays are complex; stop scanning
                else:
                    break
    except Exception as exc:
        print(f"[gguf] warning: could not parse header: {exc}")
    return None


def _hf_cache_path(model_id: str) -> str | None:
    """Return the local snapshot path if model_id is in the HF cache."""
    from huggingface_hub import try_to_load_from_cache, scan_cache_dir
    try:
        cache = scan_cache_dir()
        for repo in cache.repos:
            if repo.repo_id == model_id:
                # Use the most recent complete revision
                for rev in sorted(repo.revisions, key=lambda r: r.last_modified, reverse=True):
                    snap = Path(rev.snapshot_path)
                    if (snap / "config.json").exists():
                        return str(snap)
    except Exception:
        pass
    return None


def resolve_base_model(gguf_path: str) -> str:
    """
    Given a GGUF path, find the best locally-cached HF model to use as base.
    Raises SystemExit if nothing is cached (no downloads).
    """
    arch = _parse_gguf_arch(gguf_path)
    gguf_name = Path(gguf_path).stem.lower()
    emit("gguf", path=gguf_path, detected_arch=arch or "unknown")

    candidates: list[str] = []
    if arch:
        for key, ids in _ARCH_TO_HF.items():
            if arch.startswith(key):
                candidates.extend(ids)
                break
    if not candidates:
        # fallback: try all known small models
        candidates = ["Qwen/Qwen2.5-1.5B-Instruct", "Qwen/Qwen2.5-0.5B-Instruct"]

    for model_id in candidates:
        snap = _hf_cache_path(model_id)
        if snap:
            emit("gguf", resolved=model_id, cache=snap)
            return snap  # return local path — no download
        # Try the simple hf-hub cache dir naming
        slug = model_id.replace("/", "--")
        hf_home = Path(os.environ.get("HF_HOME", Path.home() / ".cache" / "huggingface"))
        snap_dir = hf_home / "hub" / f"models--{slug}"
        if snap_dir.exists():
            snaps = sorted(snap_dir.glob("snapshots/*/config.json"))
            if snaps:
                p = str(snaps[-1].parent)
                emit("gguf", resolved=model_id, cache=p)
                return p

    raise SystemExit(
        f"[gguf] ERROR: no locally-cached HF model found for arch='{arch}'.\n"
        f"  GGUF format is quantized-inference-only; LoRA training requires float weights.\n"
        f"  Candidates checked: {candidates}\n"
        f"  Either cache one of those models or use --base-model with a local HF path."
    )


# ── Device selection ──────────────────────────────────────────────────────────

def get_device(backend: str, precision: str):
    dtype = torch.float16 if precision == "fp16" else torch.float32

    if backend == "cuda" or (backend == "auto" and torch.cuda.is_available()):
        emit("device", using="cuda", dtype=str(dtype))
        return torch.device("cuda"), dtype

    if backend in ("directml", "auto"):
        try:
            import torch_directml
            dev = torch_directml.device()
            # Verify a real op works before committing
            _ = (torch.ones(2, 2, dtype=torch.float32).to(dev) @
                 torch.ones(2, 2, dtype=torch.float32).to(dev))
            emit("device", using="directml:privateuseone:0", dtype="float32")
            return dev, torch.float32   # always fp32 on DirectML
        except Exception as e:
            if backend == "directml":
                raise SystemExit(f"[device] DirectML requested but failed: {e}")
            emit("device", directml_unavailable=str(e))

    if backend in ("mps", "auto") and torch.backends.mps.is_available():
        emit("device", using="mps", dtype=str(dtype))
        return torch.device("mps"), torch.float32

    emit("device", using="cpu", dtype="float32")
    return torch.device("cpu"), torch.float32


# ── Training callback for live progress ──────────────────────────────────────

try:
    from transformers import TrainerCallback

    class LiveProgressCallback(TrainerCallback):
        def on_log(self, args, state, control, logs=None, **kw):
            if not logs:
                return
            elapsed = time.time() - _TRAIN_START
            loss   = logs.get("loss", "?")
            step   = state.global_step
            total  = state.max_steps
            epoch  = state.epoch or 0
            pct    = (step / total * 100) if total else 0
            eta    = (elapsed / step * (total - step)) if step > 0 else 0
            emit("progress",
                 step=step, total=total, epoch=f"{epoch:.2f}",
                 loss=f"{loss:.4f}" if isinstance(loss, float) else loss,
                 pct=f"{pct:.1f}", elapsed=f"{elapsed:.0f}s",
                 eta=f"{eta:.0f}s")

except ImportError:
    LiveProgressCallback = None


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    global _TRAIN_START

    parser = argparse.ArgumentParser()
    parser.add_argument("--gguf", type=str, default=None,
        help="Path to local GGUF model — resolves to cached HF weights by arch (no download)")
    parser.add_argument("--base-model", "--base_model",
        default="Qwen/Qwen2.5-1.5B-Instruct",
        dest="base_model",
        help="HuggingFace model ID or local path (must be in cache; no download)")
    parser.add_argument("--data",       required=True)
    parser.add_argument("--output",     required=True)
    parser.add_argument("--epochs",     type=int,   default=3)
    parser.add_argument("--batch-size", "--batch_size", type=int, default=1, dest="batch_size")
    parser.add_argument("--grad-accum", "--grad_accum", type=int, default=8, dest="grad_accum")
    parser.add_argument("--lr",         type=float, default=2e-4)
    parser.add_argument("--backend",    choices=["auto","cuda","directml","mps","cpu"],
        default="auto", help="Training backend")
    parser.add_argument("--precision",  choices=["fp32","fp16"], default="fp32")
    parser.add_argument("--extra-data", "--extra_data", type=str, default=None,
        dest="extra_data",
        help="Additional JSONL of curated live examples to merge before training")
    args = parser.parse_args()

    # ── Resolve base model (local only) ──────────────────────────────────────
    if args.gguf:
        base_model_path = resolve_base_model(args.gguf)
    else:
        # Check if it's a plain HF model ID — verify cache exists, no download
        from huggingface_hub import constants as hf_const
        hf_home = Path(os.environ.get("HF_HOME",
            Path.home() / ".cache" / "huggingface"))
        slug = args.base_model.replace("/", "--")
        snap_dir = hf_home / "hub" / f"models--{slug}"
        snaps = sorted(snap_dir.glob("snapshots/*/config.json")) if snap_dir.exists() else []
        if snaps:
            base_model_path = str(snaps[-1].parent)
            emit("model", resolved="local_cache", path=base_model_path)
        else:
            # Accept as-is (local dir path or already-downloaded model)
            base_model_path = args.base_model
            emit("model", resolved="direct", path=base_model_path)

    # ── Device ───────────────────────────────────────────────────────────────
    dev, dtype = get_device(args.backend, args.precision)
    dev_str = str(dev)

    # ── Load model + tokenizer ───────────────────────────────────────────────
    from transformers import AutoModelForCausalLM, AutoTokenizer, TrainingArguments, Trainer
    from peft import LoraConfig, get_peft_model

    emit("load", status="loading_model", base=base_model_path)
    model = AutoModelForCausalLM.from_pretrained(
        base_model_path,
        torch_dtype=dtype,
        local_files_only=True,
    )
    # DirectML: load to CPU first, then move (avoids from_pretrained device_map issues)
    if dev_str not in ("cpu",):
        try:
            model = model.to(dev)
            emit("load", status="model_on_device", device=dev_str)
        except Exception as e:
            emit("load", warning=f"device_move_failed:{e}", fallback="cpu")
            dev = torch.device("cpu")
            dev_str = "cpu"

    tokenizer = AutoTokenizer.from_pretrained(base_model_path, local_files_only=True)
    tokenizer.pad_token = tokenizer.eos_token

    lora_cfg = LoraConfig(
        r=16, lora_alpha=32,
        target_modules=["q_proj", "k_proj", "v_proj", "o_proj"],
        lora_dropout=0.05, bias="none", task_type="CAUSAL_LM",
    )
    model = get_peft_model(model, lora_cfg)
    model.print_trainable_parameters()

    # ── Load + merge data ─────────────────────────────────────────────────────
    with open(args.data, encoding="utf-8") as f:
        examples = [json.loads(line) for line in f if line.strip()]
    emit("data", synthetic=len(examples))

    curated_count = 0
    if args.extra_data and Path(args.extra_data).exists():
        with open(args.extra_data, encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    examples.append(json.loads(line))
                    curated_count += 1
                except Exception:
                    pass
        emit("data", curated=curated_count, total=len(examples))
    else:
        emit("data", curated=0, note="no_curated_file")

    texts = [ex["text"] if "text" in ex else json.dumps(ex["messages"])
             for ex in examples]

    # ── Tokenise ──────────────────────────────────────────────────────────────
    MAX_LEN = 512
    from datasets import Dataset

    def tokenize(batch):
        enc = tokenizer(batch["text"], truncation=True,
                        max_length=MAX_LEN, padding="max_length")
        enc["labels"] = enc["input_ids"].copy()
        return enc

    raw     = Dataset.from_list([{"text": t} for t in texts])
    dataset = raw.map(tokenize, batched=True, remove_columns=["text"])
    dataset.set_format("torch")
    emit("data", tokenized=len(dataset))

    # ── Train ─────────────────────────────────────────────────────────────────
    use_fp16 = (dtype == torch.float16) and dev_str not in ("cpu", "privateuseone:0")
    training_args = TrainingArguments(
        output_dir=args.output,
        num_train_epochs=args.epochs,
        per_device_train_batch_size=args.batch_size,
        gradient_accumulation_steps=args.grad_accum,
        learning_rate=args.lr,
        fp16=use_fp16,
        bf16=False,
        logging_steps=5,
        save_strategy="epoch",
        report_to="none",
        remove_unused_columns=False,
        no_cuda=(dev_str == "cpu"),
        use_cpu=(dev_str == "cpu"),
    )

    callbacks = [LiveProgressCallback()] if LiveProgressCallback else []
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=dataset,
        tokenizer=tokenizer,
        callbacks=callbacks,
    )

    emit("train", status="starting", examples=len(dataset),
         epochs=args.epochs, backend=dev_str, dtype=str(dtype))
    _TRAIN_START = time.time()
    result = trainer.train()
    elapsed = time.time() - _TRAIN_START

    final_loss = result.training_loss
    emit("train", status="complete", loss=f"{final_loss:.4f}",
         elapsed=f"{elapsed:.0f}s", steps=result.global_step)

    # ── Save ──────────────────────────────────────────────────────────────────
    out = Path(args.output)
    out.mkdir(parents=True, exist_ok=True)
    model.save_pretrained(str(out))
    tokenizer.save_pretrained(str(out))

    tmpl = Path(__file__).parent / "prompt_template.txt"
    if tmpl.exists():
        shutil.copy(tmpl, out / "prompt_template.txt")

    adapter_size = sum(f.stat().st_size for f in out.glob("*.safetensors"))
    emit("save", path=str(out), adapter_mb=f"{adapter_size/1024/1024:.1f}",
         curated_merged=curated_count, synthetic=len(dataset) - curated_count)


if __name__ == "__main__":
    main()
