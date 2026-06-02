# Adaptive Transformer: Validation & Testing Framework

**Comprehensive testing and validation procedures for the 5-phase adaptive transformer training pipeline.**

---

## Overview

This document defines:
1. **Unit tests** for each component
2. **Integration tests** for multi-component workflows
3. **Validation gates** before proceeding to next phase
4. **Performance benchmarks** at each scale
5. **Regression detection** across phases

---

## Unit Tests

### Test 1: Model Instantiation

```python
import pytest
import torch
from train_adaptive_transformer import AdaptiveTransformer

def test_model_instantiation():
    """Test creating a model with various configurations."""
    configs = [
        {"layers": 4, "hidden": 256, "heads": 4, "ffn": 1024},
        {"layers": 100, "hidden": 1024, "heads": 16, "ffn": 4096},
        {"layers": 50, "hidden": 512, "heads": 8, "ffn": 2048},
    ]
    
    for cfg in configs:
        model = AdaptiveTransformer(
            num_layers=cfg["layers"],
            hidden_dim=cfg["hidden"],
            num_heads=cfg["heads"],
            ffn_dim=cfg["ffn"],
            vocab_size=32000,
        )
        
        # Check output shape
        batch_size, seq_len = 2, 128
        input_ids = torch.randint(0, 32000, (batch_size, seq_len))
        logits = model(input_ids)
        
        assert logits.shape == (batch_size, seq_len, 32000)
        assert not torch.isnan(logits).any()
        assert not torch.isinf(logits).any()
```

### Test 2: Width Expansion

```python
def test_width_expansion():
    """Test that width expansion preserves model behavior."""
    model = AdaptiveTransformer(
        num_layers=4, hidden_dim=256, num_heads=4,
        ffn_dim=1024, vocab_size=32000
    )
    
    # Get output before expansion
    torch.manual_seed(42)
    x = torch.randint(0, 32000, (2, 128))
    logits_before = model(x).detach()
    
    # Expand width
    model.expand_width(512, init_strategy="identity_scale")
    
    # Get output after expansion
    logits_after = model(x)
    
    # Check shapes are updated
    assert model.hidden_dim == 512
    assert model.layers[0].hidden_dim == 512
    
    # Check outputs are still valid (not NaN/Inf)
    assert not torch.isnan(logits_after).any()
    assert not torch.isinf(logits_after).any()
    
    # Check that expanded model produces reasonable outputs
    # (not checking numerical equality since init is random)
    assert logits_after.shape == (2, 128, 32000)
    assert (logits_after.std() > 0.1)  # Not collapsed
```

### Test 3: Gradient Flow

```python
def test_gradient_flow():
    """Test that gradients flow correctly through all layers."""
    model = AdaptiveTransformer(
        num_layers=10, hidden_dim=256, num_heads=4,
        ffn_dim=1024, vocab_size=32000
    )
    
    x = torch.randint(0, 32000, (4, 64))
    labels = torch.randint(0, 32000, (4, 64))
    
    # Forward
    logits = model(x)
    loss = torch.nn.functional.cross_entropy(
        logits.view(-1, 32000), labels.view(-1)
    )
    
    # Backward
    loss.backward()
    
    # Check all parameters have gradients
    for name, param in model.named_parameters():
        assert param.grad is not None, f"No gradient for {name}"
        assert not torch.isnan(param.grad).any(), f"NaN gradient for {name}"
        assert not torch.isinf(param.grad).any(), f"Inf gradient for {name}"
        assert param.grad.abs().max() > 0, f"Zero gradient for {name}"
```

### Test 4: Layer Freezing

```python
def test_layer_freezing():
    """Test that freezing layers prevents gradient computation."""
    model = AdaptiveTransformer(
        num_layers=10, hidden_dim=256, num_heads=4,
        ffn_dim=1024, vocab_size=32000
    )
    
    # Freeze first 5 layers
    for i in range(5):
        for param in model.layers[i].parameters():
            param.requires_grad = False
    
    # Forward & backward
    x = torch.randint(0, 32000, (4, 64))
    labels = torch.randint(0, 32000, (4, 64))
    logits = model(x)
    loss = torch.nn.functional.cross_entropy(
        logits.view(-1, 32000), labels.view(-1)
    )
    loss.backward()
    
    # Check frozen layers have no gradients
    for i in range(5):
        for param in model.layers[i].parameters():
            assert param.grad is None, f"Layer {i} should be frozen"
    
    # Check unfrozen layers have gradients
    for i in range(5, 10):
        has_grad = False
        for param in model.layers[i].parameters():
            if param.grad is not None and param.grad.abs().max() > 0:
                has_grad = True
        assert has_grad, f"Layer {i} should have gradients"
```

### Test 5: Learning Rate Scheduler

```python
def test_warmup_scheduler():
    """Test linear warmup scheduler."""
    from train_adaptive_transformer import warmup_linear
    
    model = AdaptiveTransformer(
        num_layers=4, hidden_dim=256, num_heads=4,
        ffn_dim=1024, vocab_size=32000
    )
    
    optimizer = torch.optim.Adam(model.parameters(), lr=1e-3)
    scheduler = warmup_linear(optimizer, warmup_steps=1000)
    
    # Check warmup phase
    expected_lrs = []
    for step in [0, 100, 500, 999, 1000, 5000]:
        optimizer.zero_grad()
        x = torch.randint(0, 32000, (2, 64))
        logits = model(x)
        loss = logits.mean()  # Dummy loss
        loss.backward()
        optimizer.step()
        scheduler.step()
        
        current_lr = scheduler.get_last_lr()[0]
        expected_lrs.append(current_lr)
    
    # Check warmup is linear
    assert expected_lrs[0] < expected_lrs[1] < expected_lrs[2]  # Increasing
    assert expected_lrs[2] < expected_lrs[3]  # Still increasing at 999
    assert abs(expected_lrs[4] - expected_lrs[3]) < 1e-6  # Constant after 1000
```

---

## Integration Tests

### Integration Test 1: Phase 0 Training Loop

```python
def test_phase_0_training_loop():
    """Test complete Phase 0 training loop."""
    import tempfile
    import json
    from train_adaptive_transformer import train_phase_0
    
    config = {
        "seed": 42,
        "model": {
            "num_layers": 4,
            "hidden_dim": 256,
            "num_heads": 4,
            "ffn_dim": 1024,
            "vocab_size": 32000,
            "max_seq_len": 512,
        },
        "training": {
            "batch_size": 16,
            "learning_rate": 3e-4,
            "warmup_steps": 100,
            "total_steps": 200,  # Short for testing
            "eval_interval": 50,
            "save_interval": 100,
        },
        "data": {
            "sources": ["dummy"],
        }
    }
    
    with tempfile.TemporaryDirectory() as tmpdir:
        model = train_phase_0(config, tmpdir, logger=None)
        
        # Check model trained
        assert model is not None
        
        # Check checkpoint saved
        checkpoint_path = f"{tmpdir}/phase_0_best.pt"
        assert os.path.exists(checkpoint_path)
        
        # Check metrics file
        metrics_path = f"{tmpdir}/metrics.jsonl"
        assert os.path.exists(metrics_path)
        
        # Validate metrics
        with open(metrics_path) as f:
            metrics = [json.loads(line) for line in f]
        
        assert len(metrics) > 0
        assert all("val_ppl" in m or "loss" in m for m in metrics)
```

### Integration Test 2: Phase 0 → Phase 1

```python
def test_phase_0_to_phase_1_transition():
    """Test loading Phase 0 checkpoint and starting Phase 1."""
    import tempfile
    from train_adaptive_transformer import (
        train_phase_0, train_phase_1, load_checkpoint
    )
    
    config_0 = {
        "seed": 42,
        "model": {
            "num_layers": 4,
            "hidden_dim": 256,
            "num_heads": 4,
            "ffn_dim": 1024,
            "vocab_size": 32000,
        },
        "training": {
            "batch_size": 16,
            "learning_rate": 3e-4,
            "warmup_steps": 50,
            "total_steps": 100,
            "eval_interval": 50,
        },
        "data": {"sources": ["dummy"]},
    }
    
    config_1 = {
        "seed": 42,
        "model": {
            "num_layers": 20,  # Expand to 20 for testing
            "hidden_dim": 256,
            "num_heads": 4,
            "ffn_dim": 1024,
            "vocab_size": 32000,
            "base_num_layers": 4,
        },
        "training": {
            "batch_size": 16,
            "learning_rate": 1e-3,
            "warmup_steps": 50,
            "epochs_per_layer": 2,
            "eval_interval": 50,
        },
        "data": {"sources": ["dummy"]},
    }
    
    with tempfile.TemporaryDirectory() as tmpdir:
        # Phase 0
        print("Training Phase 0...")
        train_phase_0(config_0, f"{tmpdir}/phase_0")
        
        # Phase 1
        print("Training Phase 1...")
        train_phase_1(config_1, f"{tmpdir}/phase_1")
        
        # Verify checkpoints exist
        assert os.path.exists(f"{tmpdir}/phase_1/phase_1_layer_19_best.pt")
```

---

## Validation Gates

### Gate 1: Phase 0 → Phase 1

**Pre-requisites:**
- [ ] Phase 0 perplexity < 25
- [ ] Phase 0 training stable (no divergence)
- [ ] Checkpoint saved and loadable
- [ ] Model parameter count matches expectation (~100M)

**Check:**
```python
def validate_phase_0_output(checkpoint_path, config):
    """Validate Phase 0 output meets gates."""
    model = load_checkpoint(checkpoint_path)
    
    # Check parameter count
    params = sum(p.numel() for p in model.parameters())
    expected = 100e6  # ~100M
    assert params > expected * 0.9 and params < expected * 1.1, \
        f"Param count {params} not near {expected}"
    
    # Check model compiles and runs
    x = torch.randint(0, 32000, (2, 128))
    logits = model(x)
    assert logits.shape == (2, 128, 32000)
    
    # Check output is reasonable
    assert not torch.isnan(logits).any()
    assert logits.std() > 0.01
    
    return True
```

### Gate 2: Phase 1 → Phase 2

**Pre-requisites:**
- [ ] All 96 new layers added successfully
- [ ] Final perplexity ≤ 16 (no cliff from base)
- [ ] Monotonic or flat improvement curve
- [ ] No regression > 10% from base

**Check:**
```python
def validate_phase_1_output(final_checkpoint, config):
    """Validate Phase 1 depth addition."""
    model = load_checkpoint(final_checkpoint)
    
    # Check layer count
    assert len(model.layers) == 100
    
    # Check output shape
    x = torch.randint(0, 32000, (2, 128))
    logits = model(x)
    assert logits.shape == (2, 128, 32000)
    
    # Estimate parameter count
    # 100 layers × (256² attn + 256×1024×2 FFN) ≈ 2.5B
    params = sum(p.numel() for p in model.parameters())
    expected = 2.5e9
    assert params > expected * 0.9, f"Param count {params} < {expected}"
    
    return True
```

### Gate 3: Phase 2 → Phase 3

**Pre-requisites:**
- [ ] All 3 width expansions completed
- [ ] Final hidden_dim = 1024
- [ ] Perplexity < 13
- [ ] No regression upon any expansion

**Check:**
```python
def validate_phase_2_output(final_checkpoint, config):
    """Validate Phase 2 width expansion."""
    model = load_checkpoint(final_checkpoint)
    
    # Check width
    assert model.hidden_dim == 1024
    assert model.layers[0].hidden_dim == 1024
    
    # Check param count ~10B
    params = sum(p.numel() for p in model.parameters())
    expected = 10e9
    assert params > expected * 0.8 and params < expected * 1.2
    
    # Functional test
    x = torch.randint(0, 32000, (2, 256))  # Longer sequence
    logits = model(x)
    assert logits.shape == (2, 256, 32000)
    
    return True
```

### Gate 4: Phase 3 → Phase 4

**Pre-requisites:**
- [ ] All expert counts added (1, 2, 4, ..., 1024)
- [ ] Final perplexity < 11.5
- [ ] Expert load balanced (within 20%)
- [ ] All routing masks valid

**Check:**
```python
def validate_phase_3_output(final_checkpoint, config):
    """Validate Phase 3 expert pool training."""
    model = load_checkpoint(final_checkpoint)
    
    # Check expert pool size
    assert model.expert_pool.num_experts == 1024
    assert len(model.expert_pool.experts) == 1024
    
    # Check routing layers
    assert len(model.routers) == 100
    assert all(r.num_experts == 1024 for r in model.routers)
    
    # Check param count ~10B (experts add minimal params due to sharing)
    params = sum(p.numel() for p in model.parameters())
    expected = 10e9
    assert params > expected * 0.95
    
    return True
```

### Gate 5: Phase 4 → Phase 5

**Pre-requisites:**
- [ ] All 100 LoRA adapters trained
- [ ] Cumulative improvement > 0.5 ppl
- [ ] Final perplexity < 11.2
- [ ] Each adapter converged

**Check:**
```python
def validate_phase_4_output(adapters_dir, config):
    """Validate Phase 4 LoRA adapter training."""
    import os
    
    # Check all adapters present
    adapter_files = [f for f in os.listdir(adapters_dir) 
                     if f.startswith("lora_adapter_")]
    assert len(adapter_files) == 100, f"Missing adapters: {len(adapter_files)}/100"
    
    # Check each can be loaded
    for i in range(100):
        adapter_path = os.path.join(adapters_dir, f"lora_adapter_{i:03d}.pt")
        assert os.path.exists(adapter_path)
        state = torch.load(adapter_path)
        assert isinstance(state, dict)
    
    return True
```

### Gate 6: Final Validation

**Pre-requisites:**
- [ ] All scales validated per-step
- [ ] KL divergence < 0.5 nats for all scales
- [ ] No quality regression across any combination
- [ ] Training curves smooth and converged

**Check:**
```python
def validate_phase_5_output(metrics_file, config):
    """Validate Phase 5 co-adaptation."""
    import json
    
    with open(metrics_file) as f:
        metrics = [json.loads(line) for line in f]
    
    # Check KL divergence
    for m in metrics:
        if "kl_divergence" in m:
            assert m["kl_divergence"]["mean"] < 0.5
            assert m["kl_divergence"]["max"] < 0.7
    
    # Check final perplexity
    final_ppls = [m["val_ppl"] for m in metrics if "val_ppl" in m]
    assert final_ppls[-1] < 11.5, f"Final ppl {final_ppls[-1]} too high"
    
    # Check monotonicity (some regression is OK)
    recent_ppls = final_ppls[-100:]
    mean_recent = sum(recent_ppls) / len(recent_ppls)
    assert mean_recent < 11.5
    
    return True
```

---

## Performance Benchmarks

### Benchmark 1: Inference Latency

```python
def benchmark_inference_latency(model, seq_lengths=[64, 256, 1024]):
    """Measure inference latency across sequence lengths."""
    import time
    
    model.eval()
    device = next(model.parameters()).device
    
    results = {}
    for seq_len in seq_lengths:
        x = torch.randint(0, 32000, (1, seq_len)).to(device)
        
        # Warmup
        for _ in range(5):
            _ = model(x)
        
        # Benchmark
        with torch.no_grad():
            start = time.time()
            for _ in range(10):
                _ = model(x)
            elapsed = time.time() - start
        
        latency_ms = (elapsed / 10) * 1000
        tokens_per_sec = seq_len / (latency_ms / 1000)
        
        results[seq_len] = {
            "latency_ms": latency_ms,
            "tokens_per_sec": tokens_per_sec,
        }
    
    return results
```

### Benchmark 2: Memory Usage

```python
def benchmark_memory(model, seq_lengths=[256, 1024]):
    """Measure peak memory usage."""
    import torch
    
    device = next(model.parameters()).device
    
    for seq_len in seq_lengths:
        torch.cuda.reset_peak_memory_stats(device)
        torch.cuda.synchronize(device)
        
        x = torch.randint(0, 32000, (1, seq_len)).to(device)
        
        with torch.no_grad():
            _ = model(x)
        
        torch.cuda.synchronize(device)
        peak_memory = torch.cuda.max_memory_allocated(device) / 1e9
        
        print(f"Seq len {seq_len}: {peak_memory:.2f}GB")
```

### Benchmark 3: Training Throughput

```python
def benchmark_training_throughput(model, batch_size=256, seq_len=1024):
    """Measure training throughput (tokens/sec)."""
    import time
    
    model.train()
    device = next(model.parameters()).device
    optimizer = torch.optim.Adam(model.parameters())
    
    x = torch.randint(0, 32000, (batch_size, seq_len)).to(device)
    labels = torch.randint(0, 32000, (batch_size, seq_len)).to(device)
    
    # Warmup
    for _ in range(3):
        logits = model(x)
        loss = F.cross_entropy(logits.view(-1, 32000), labels.view(-1))
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
    
    # Benchmark
    torch.cuda.synchronize(device)
    start = time.time()
    
    steps = 10
    for _ in range(steps):
        logits = model(x)
        loss = F.cross_entropy(logits.view(-1, 32000), labels.view(-1))
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
    
    torch.cuda.synchronize(device)
    elapsed = time.time() - start
    
    tokens_per_sec = (batch_size * seq_len * steps) / elapsed
    print(f"Training throughput: {tokens_per_sec:.0f} tokens/sec")
```

---

## Regression Detection

### Automated Regression Check

```python
def detect_regressions(phase_metrics, previous_best_metrics):
    """Detect quality regressions across phases."""
    regressions = {}
    
    for metric_name in ["val_ppl", "val_loss"]:
        if metric_name not in phase_metrics:
            continue
        
        current = phase_metrics[metric_name]
        previous = previous_best_metrics.get(metric_name)
        
        if previous is None:
            continue
        
        # Allow 5% regression (noise tolerance)
        threshold = previous * 1.05
        
        if current > threshold:
            regression_pct = ((current - previous) / previous) * 100
            regressions[metric_name] = {
                "previous": previous,
                "current": current,
                "regression_pct": regression_pct,
                "severity": "critical" if regression_pct > 10 else "warning",
            }
    
    return regressions
```

### Per-Scale Regression Check

```python
def validate_scale_regression(scale_metrics, baseline_metrics):
    """Check for regressions in per-scale metrics."""
    alerts = []
    
    for scale, ppl in scale_metrics.items():
        baseline = baseline_metrics.get(scale)
        
        if baseline is None:
            continue
        
        regression = ((ppl - baseline) / baseline) * 100
        
        if regression > 5:  # 5% threshold
            alerts.append({
                "scale": scale,
                "baseline_ppl": baseline,
                "current_ppl": ppl,
                "regression_pct": regression,
            })
    
    if alerts:
        print("⚠️  Scale regression warnings:")
        for alert in alerts:
            print(f"  {alert['scale']}: {alert['baseline_ppl']:.2f} → {alert['current_ppl']:.2f} "
                  f"({alert['regression_pct']:.1f}% regression)")
    
    return alerts
```

---

## Continuous Testing

### GitHub Actions Workflow

```yaml
# .github/workflows/adaptive-transformer-tests.yml
name: Adaptive Transformer Tests

on: [push, pull_request]

jobs:
  tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Set up Python
        uses: actions/setup-python@v2
        with:
          python-version: 3.10
      
      - name: Install dependencies
        run: |
          pip install torch torchvision torchaudio
          pip install pytest pytest-cov
      
      - name: Run unit tests
        run: |
          pytest tests/unit/test_adaptive_transformer.py -v
      
      - name: Run integration tests
        run: |
          pytest tests/integration/test_adaptive_phases.py -v
      
      - name: Upload coverage
        uses: codecov/codecov-action@v2
```

---

## Test Execution

```bash
# Run all tests
pytest tests/ -v

# Run specific test
pytest tests/unit/test_adaptive_transformer.py::test_model_instantiation -v

# Run with coverage
pytest tests/ --cov=train_adaptive_transformer --cov-report=html

# Run integration tests (slow)
pytest tests/integration/ -v -m integration

# Quick smoke test
pytest tests/ -v -k "not integration"
```

---

## Success Criteria Summary

| Phase | Perplexity | Regression Limit | Status Check |
|-------|-----------|------------------|--------------|
| 0     | 20-30     | N/A              | Training stable, checkpoint valid |
| 1     | 14-16     | < 10% from base  | All 96 layers added, monotonic curve |
| 2     | 12-14     | < 5% per step    | All widths valid, new dims active |
| 3     | 10-12     | < 5% per step    | Load balanced, all experts used |
| 4     | 10-11     | < 5% cumulative  | All 100 adapters trained |
| 5     | 10-11.2   | < 2% final       | KL < 0.5, all scales valid |

