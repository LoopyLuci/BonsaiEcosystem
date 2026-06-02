# Adaptive Transformer Training: Implementation Guide

**Complete guide for implementing the 5-phase training pipeline for Bonsai Adaptive Transformer.**

---

## Quick Start

### Run Phase 0 (Base Model)

```bash
cd bonsai-workspace/runtimes/bonsai-trainer

# Single GPU
python train_adaptive_transformer.py \
    --config configs/adaptive_phase_0.json \
    --phase 0 \
    --output-dir ./checkpoints/phase_0 \
    --seed 42

# Multiple GPUs (DDP)
torchrun --nproc_per_node=8 train_adaptive_transformer.py \
    --config configs/adaptive_phase_0.json \
    --phase 0 \
    --output-dir ./checkpoints/phase_0 \
    --seed 42
```

### Run Phase 1 (Depth)

```bash
python train_adaptive_transformer.py \
    --config configs/adaptive_phase_1.json \
    --phase 1 \
    --output-dir ./checkpoints/phase_1 \
    --seed 42
```

### Full Pipeline

```bash
#!/bin/bash
set -e

for phase in 0 1 2 3 4 5; do
    echo "Starting Phase $phase..."
    python train_adaptive_transformer.py \
        --config configs/adaptive_phase_${phase}.json \
        --phase $phase \
        --output-dir ./checkpoints/phase_${phase} \
        --seed 42
done
```

---

## System Architecture

### Model Checkpoint Hierarchy

```
checkpoints/
├── phase_0/                          # Base model (100M)
│   ├── best.pt                       # Best checkpoint
│   ├── step_1000.pt
│   ├── step_5000.pt
│   └── metrics.jsonl
│
├── phase_1/                          # Depth (4→100L)
│   ├── layer_4_ppl_20.5.pt
│   ├── layer_8_ppl_19.2.pt
│   ├── layer_50_ppl_16.1.pt
│   ├── layer_100_ppl_15.8.pt         # Best for depth
│   └── metrics.jsonl
│
├── phase_2/                          # Width (256→1024D)
│   ├── width_512_ppl_14.2.pt
│   ├── width_768_ppl_13.5.pt
│   ├── width_1024_ppl_12.8.pt        # Best for width
│   └── metrics.jsonl
│
├── phase_3/                          # Experts (1→1024E)
│   ├── experts_1_ppl_12.5.pt
│   ├── experts_2_ppl_12.4.pt
│   ├── experts_1024_ppl_11.2.pt      # Final model
│   └── metrics.jsonl
│
├── phase_4/                          # LoRA adapters
│   ├── adapter_000/
│   │   ├── weights.pt
│   │   └── config.json
│   ├── adapter_050/
│   └── adapter_099/
│
└── phase_5/                          # Co-adaptation
    ├── step_10000_ppl_11.5.pt
    ├── step_50000_ppl_11.2.pt
    └── metrics.jsonl
```

### Integration with Bonsai Registry

```python
# After training completes, package as Crystal image
from bonsai_model_registry.crystal import CrystalStore

crystal_store = CrystalStore()

# Phase 0 checkpoint
crystal_store.save_checkpoint(
    checkpoint_path="checkpoints/phase_0/best.pt",
    model_name="bonsai-adaptive-base",
    version="1.0.0",
    metadata={
        "phase": 0,
        "params": 100e6,
        "val_ppl": 20.5,
        "architecture": "transformer_4L_256D",
    }
)

# Phase 5 final model
crystal_store.save_checkpoint(
    checkpoint_path="checkpoints/phase_5/best.pt",
    model_name="bonsai-adaptive-full",
    version="1.0.0",
    metadata={
        "phase": 5,
        "params": 10e9,
        "val_ppl": 11.2,
        "architecture": "transformer_100L_1024D_1024E",
        "adapters": 100,
    }
)
```

---

## Phase Descriptions & Expectations

### Phase 0: Base Model (3-4 days, ~100 GPU-days)

**Goal:** Train a minimal but functional transformer.

**Configuration:**
- Layers: 4
- Hidden dim: 256
- Heads: 4
- FFN dim: 1024
- Total params: ~100M

**Training:**
- Data: 1B tokens from Pile
- Batch size: 256
- Learning rate: 3e-4
- Duration: ~100k steps

**Expected Outcomes:**
- Validation perplexity: 20-30
- Training should be stable
- No vanishing/exploding gradients

**Quality Gate:**
- [ ] Val perplexity < 25
- [ ] Training stable throughout
- [ ] Checkpoint saved with metrics

### Phase 1: Progressive Depth (2-3 days, ~200 GPU-days)

**Goal:** Grow from 4 to 100 layers without harming quality.

**Strategy:**
1. Load Phase 0 checkpoint (4 layers)
2. Initialize full 100-layer model
3. Copy trained weights to layers 0-3
4. For each new layer 4..99:
   - Freeze layers 0..i-1
   - Train only layer i (and output head) for 5 epochs
   - Validate and checkpoint

**Configuration:**
- Start: 4 layers, 256 hidden dim
- End: 100 layers, 256 hidden dim
- New layer LR: 1e-3 (high)
- Head LR: 3e-4 (low)

**Expected Outcomes:**
- Layer 4: ~20.2 ppl (close to base)
- Layer 16: ~19.0 ppl
- Layer 50: ~17.5 ppl
- Layer 100: ~15.8 ppl (no cliff drops)

**Quality Gate:**
- [ ] Monotonic or flat perplexity curve
- [ ] No regression > 10% from previous layer
- [ ] All 96 new layers successfully added

**Monitoring:**
```
Layer  4: ppl = 20.2 ✓
Layer  8: ppl = 19.8 ✓
Layer 16: ppl = 19.0 ✓
Layer 32: ppl = 18.2 ✓
Layer 64: ppl = 16.5 ✓
Layer 100: ppl = 15.8 ✓
```

### Phase 2: Progressive Width (2 days, ~100 GPU-days)

**Goal:** Expand hidden dimension without regression.

**Strategy:**
1. Load Phase 1 checkpoint (100 layers, 256D)
2. For each width expansion (256→512→768→1024):
   - Expand all projection matrices
   - Initialize new dimensions with identity-scale init
   - Train for 10 epochs
   - Validate

**Width Progression:**
```
256D → 512D → 768D → 1024D
(4x expansion total)
```

**Initialization Strategy (Recommended):**
```python
# Old weight matrix: (D, D)
# New weight matrix: (D', D') where D' = 2*D

new_weight = zeros(D', D')
new_weight[:D, :D] = old_weight  # Copy old
for i in range(D, D'):
    new_weight[i, i] = 0.1  # Scaled identity for new dims
```

**Expected Outcomes:**
- 256D: baseline ~15.8 ppl
- 512D: ~14.2 ppl (10% improvement)
- 768D: ~13.5 ppl
- 1024D: ~12.8 ppl

**Quality Gate:**
- [ ] No regression upon expansion
- [ ] New dimensions activate (not dead)
- [ ] Final width stable at target

### Phase 3: Expert Pool Training (3-4 days, ~150 GPU-days)

**Goal:** Grow expert pool from 1 to 1024 shared experts.

**Expert Architecture:**
```python
class ExpertPool(nn.Module):
    def __init__(self, num_experts, hidden_dim, ffn_dim):
        self.experts = nn.ModuleList([
            FFNExpert(hidden_dim, ffn_dim) for _ in range(num_experts)
        ])
    
    def forward(self, x, routing_weights):
        # Route tokens to selected experts
        # Return weighted sum of expert outputs
```

**Expansion Schedule:**
```
1 expert   (baseline, single FFN per layer)
↓ (train 5 epochs)
2 experts  (now use routing layer)
↓ (train 5 epochs)
4 experts
↓ (train 5 epochs)
8, 16, 32, 64, 128, 256, 512, 1024
↓ (train 10 epochs at full)
Final: 1024 experts
```

**Routing Details:**
- Top-k routing: k=2 (select 2 experts per token)
- Load-balancing loss: 0.01 × aux_loss
- Encourage balanced utilization across tokens

**Expected Outcomes:**
- 1 expert: ~12.8 ppl (baseline)
- 2 experts: ~12.7 ppl
- 4 experts: ~12.5 ppl
- 1024 experts: ~11.2 ppl

**Quality Gate:**
- [ ] Expert load balanced (within 20%)
- [ ] No quality cliff at any expansion
- [ ] Final perplexity < 11.5

### Phase 4: LoRA Adapters (2-3 days, ~100 GPU-days)

**Goal:** Train 100 independent LoRA adapters sequentially.

**LoRA Configuration:**
```python
class LoRAAdapter(nn.Module):
    def __init__(self, hidden_dim, rank=32):
        self.A = nn.Linear(hidden_dim, rank, bias=False)
        self.B = nn.Linear(rank, hidden_dim, bias=False)
        # Initialize A with small random, B with zeros
    
    def forward(self, x):
        return self.B(self.A(x))  # Returns delta: ΔW @ x
```

**Training Strategy:**
1. Freeze entire base model (100 layers + 1024 experts)
2. For each adapter ID in 0..99:
   - Initialize new adapter
   - Train only this adapter for 5 epochs
   - Freeze it
   - Move to next

**Per-Adapter Details:**
- Rank: 32 (balance capacity vs. params)
- Parameters per adapter: 32 × (1024 + 1024) × 100 layers ≈ 6.4M
- Total adapters: 100
- Cumulative: 640M parameters (optional, sparse)

**Expected Outcomes:**
- Adapter 0: +0.1 ppl improvement
- Adapter 50: cumulative improvement > 0.5 ppl
- Adapter 99: final best perplexity ~11.0 ppl

**Quality Gate:**
- [ ] Each adapter trains successfully
- [ ] Cumulative improvement monotonic
- [ ] Final perplexity < 11.2 with all adapters

### Phase 5: Co-adaptation (1-2 days, ~50 GPU-days)

**Goal:** Align all scales so subsets work well together.

**Co-adaptation Strategy:**
```python
# At each training step, sample random scale config:
config = {
    "num_layers": random.choice([4, 8, 16, 32, 64, 100]),
    "width_factor": random.choice([1, 2, 3, 4]),
    "num_experts": random.choice([1, 2, 4, ..., 1024]),
    "num_adapters": random.choice([0, 10, 20, ..., 100]),
}

# Forward with full model and subset model
logits_full = model(batch, all_scales)
logits_subset = model(batch, **config)

# Loss: CE + diversity
loss = ce_loss(logits_full) + 0.1 * kl_div(logits_full, logits_subset)
```

**Diversity Loss:**
```python
def kl_divergence(logits_full, logits_subset, temperature=4.0):
    p = F.softmax(logits_full / temperature, dim=-1)
    q = F.log_softmax(logits_subset / temperature, dim=-1)
    return F.kl_div(q, p, reduction="batchmean")
```

**Expected Outcomes:**
- All scales produce bounded divergence (KL < 0.5 nats)
- Final validation perplexity: ~11.0-11.2
- No quality regression across any scale

**Quality Gate:**
- [ ] KL divergence < 0.5 nats for all scales
- [ ] No regression from Phase 4 best
- [ ] All scales validated per-step

---

## Monitoring & Metrics

### Per-Phase Metrics File

Each phase produces a `metrics.jsonl` with one JSON object per line:

```json
{
  "timestamp": "2026-06-01T12:34:56.789",
  "phase": 0,
  "step": 1000,
  "loss": 2.854,
  "val_loss": 2.912,
  "val_ppl": 18.4,
  "learning_rate": 3e-4,
  "gradient_norm": 0.52
}
```

### Real-time Monitoring

```bash
# Watch metrics as they're written
tail -f checkpoints/phase_0/metrics.jsonl | jq '.val_ppl'

# Plot training curves
python plot_training_curves.py checkpoints/phase_0/metrics.jsonl
```

### Scale-Specific Validation (Phase 5)

```json
{
  "timestamp": "2026-06-01T12:34:56.789",
  "phase": 5,
  "step": 10000,
  "scales": {
    "layers_4_width_1": 24.2,
    "layers_4_width_4": 18.5,
    "layers_100_width_1": 15.8,
    "layers_100_width_4": 12.8,
    "experts_1": 12.8,
    "experts_1024": 11.2,
    "adapters_0": 11.2,
    "adapters_100": 10.9
  },
  "kl_divergence": {
    "mean": 0.23,
    "max": 0.48,
    "threshold": 0.5
  }
}
```

---

## Common Issues & Troubleshooting

### Issue: Perplexity jumps when adding a new layer (Phase 1)

**Symptom:** Layer N trains fine, but layer N+1 causes 5%+ regression.

**Causes:**
1. Learning rate too high for new layer
2. New layer initialization too large
3. Frozen base layers not adapted to new capacity

**Solution:**
```python
# Reduce new layer learning rate
optimizer = Adam([
    {"params": model.layers[new_id].parameters(), "lr": 1e-3},  # Try 5e-4
    {"params": model.lm_head.parameters(), "lr": 3e-4},
])

# Reduce initialization scale
nn.init.xavier_uniform_(layer.ffn.weight, gain=0.05)  # Was 0.1

# Train more epochs before moving to next
config["epochs_per_layer"] = 10  # Was 5
```

### Issue: New width dimensions remain dead (Phase 2)

**Symptom:** After expansion, new dimensions have near-zero activations.

**Causes:**
1. Initialization too small (no signal)
2. Gradient flow blocked
3. Layer norm suppressing signals

**Solution:**
```python
# Check activation statistics during training
for name, param in model.named_parameters():
    if "expanded" in name:  # New parameters
        print(f"{name}: mean={param.grad.mean()}, std={param.grad.std()}")

# Increase init scale slightly
for new_dim in range(old_dim, new_dim):
    new_weight[new_dim, new_dim] = 0.5  # Was 0.1

# Reduce learning rate to prevent divergence
optimizer = Adam(model.parameters(), lr=1e-4)  # Was 3e-4
```

### Issue: Expert load imbalanced (Phase 3)

**Symptom:** Some experts get 90% of tokens, others get < 1%.

**Causes:**
1. Load-balancing loss weight too low
2. Router initialized poorly
3. Expert capacity bottleneck

**Solution:**
```python
# Increase auxiliary loss weight
config["routing"]["load_balance_loss_weight"] = 0.1  # Was 0.01

# Initialize router with near-uniform distribution
nn.init.constant_(router.gate.weight, 0.0)  # All experts equally likely

# Add entropy regularization
entropy_loss = -torch.sum(router_probs * torch.log(router_probs + 1e-8))
total_loss = ce_loss + 0.1 * aux_loss + 0.01 * entropy_loss
```

### Issue: LoRA adapters training too slowly (Phase 4)

**Symptom:** Individual adapter improvements < 0.01 ppl.

**Causes:**
1. Base model already near-optimal
2. Adapter rank too small
3. Learning rate too conservative

**Solution:**
```python
# Increase LoRA rank
config["lora"]["rank"] = 64  # Was 32

# Increase learning rate for adapters
optimizer = Adam(adapter_params, lr=5e-3)  # Was 1e-3

# Reduce epochs per adapter if previous adapters done
config["epochs_per_adapter"] = 3  # Was 5 (if early adapters show saturation)
```

### Issue: Co-adaptation loss doesn't converge (Phase 5)

**Symptom:** Diversity loss stays high, doesn't improve.

**Causes:**
1. Temperature too low (softmax too sharp)
2. Subset scales too divergent
3. Learning rate too low

**Solution:**
```python
# Increase temperature (soften softmax)
config["co_adaptation"]["temperature"] = 8.0  # Was 4.0

# Start with closer subsets (fewer scales per batch)
config["co_adaptation"]["num_random_scales"] = 3  # Was 10

# Increase learning rate
config["training"]["learning_rate"] = 5e-4  # Was 1e-4

# Gradually reduce scale diversity
for epoch in range(num_epochs):
    if epoch < 10:
        scale_diversity = "low"  # Only similar scales
    elif epoch < 20:
        scale_diversity = "medium"
    else:
        scale_diversity = "high"  # Full diversity
```

---

## Validation Checklist

### Before Starting Training

- [ ] Data downloaded and accessible (Pile subset)
- [ ] GPU memory sufficient (estimate for each phase)
- [ ] Config JSON files validated (`python -m json.tool config.json`)
- [ ] Output directories created and writable
- [ ] Random seed set for reproducibility
- [ ] Logging configured and tested

### During Training

- [ ] Loss decreases for first few thousand steps
- [ ] Gradient norms reasonable (not NaN/Inf)
- [ ] Learning rate schedule applied correctly
- [ ] Validation metrics improving overall
- [ ] Checkpoints saving successfully
- [ ] No OOM errors

### After Each Phase

- [ ] Best checkpoint identified
- [ ] Metrics file complete and valid
- [ ] Perplexity within expected range
- [ ] No quality regressions
- [ ] Checkpoint can be loaded for next phase

---

## Resource Requirements

### GPU Memory per Phase

| Phase | Model Size | Batch Size | Grad. Accum. | Approx. GPU Memory |
|-------|-----------|-----------|--------------|-------------------|
| 0     | 100M      | 256       | 1            | 12GB (8xA100)     |
| 1     | 2.5B      | 256       | 1            | 32GB (8xA100)     |
| 2     | 10B       | 256       | 2            | 40GB (8xA100)     |
| 3     | 10B+E     | 256       | 2            | 48GB (8xA100)     |
| 4     | 10B+E     | 256       | 1            | 24GB (4xA100)     |
| 5     | 10B+E     | 256       | 2            | 48GB (8xA100)     |

### Estimated Timeline (256-GPU cluster)

| Phase | Duration | GPU-Days |
|-------|----------|----------|
| 0     | 3-4d     | ~100     |
| 1     | 2-3d     | ~200     |
| 2     | 2d       | ~100     |
| 3     | 3-4d     | ~150     |
| 4     | 2-3d     | ~100     |
| 5     | 1-2d     | ~50      |
| **Total** | **13-18d** | **~700** |

---

## Reproducibility

### Seed Strategy

```python
# Set all random sources
config["seed"] = 42

# In training script:
torch.manual_seed(config["seed"])
torch.cuda.manual_seed_all(config["seed"])
np.random.seed(config["seed"])
random.seed(config["seed"])

# Deterministic behavior (slower)
torch.backends.cudnn.deterministic = True
torch.backends.cudnn.benchmark = False
```

### Version Control

```bash
# After training completes, commit:
git add docs/ADAPTIVE_TRANSFORMER_TRAINING_PIPELINE.md
git add bonsai-workspace/runtimes/bonsai-trainer/train_adaptive_transformer.py
git add bonsai-workspace/runtimes/bonsai-trainer/configs/
git commit -m "feat: complete adaptive transformer training pipeline (phases 0-5)"

# Tag release
git tag -a v1.0.0-adaptive-pipeline -m "Initial adaptive transformer training pipeline"
git push origin main v1.0.0-adaptive-pipeline
```

---

## Integration with Universe

### Eternal Training Loop Integration

```python
# In eternal_training_loop.rs or continuous_training.rs
impl EternalTrainingLoop {
    pub async fn train_adaptive_transformer(&self) -> Result<()> {
        let phases = vec![0, 1, 2, 3, 4, 5];
        
        for phase in phases {
            let config = format!("configs/adaptive_phase_{}.json", phase);
            
            let output = self.run_training_phase(
                "train_adaptive_transformer.py",
                &config,
                phase,
            ).await?;
            
            // Log results to Universe
            self.universe_bus.emit(SystemEvent {
                event_type: "TrainingPhaseComplete".to_string(),
                phase,
                metrics: output.metrics,
            }).await?;
            
            // Checkpoint to Crystal
            self.crystal_store.save_checkpoint(
                output.checkpoint_path,
                format!("adaptive-phase-{}", phase),
            ).await?;
        }
        
        Ok(())
    }
}
```

### Model Registry Integration

```python
# After Phase 5 training
from bonsai_model_registry import ModelRegistry

registry = ModelRegistry::new()

# Register each phase checkpoint
for phase in 0..=5:
    registry.register_model(
        name=f"bonsai-adaptive-phase-{phase}",
        version="1.0.0",
        checkpoint_path=f"checkpoints/phase_{phase}/best.pt",
        manifest={
            "architecture": "adaptive_transformer",
            "phase": phase,
            "num_layers": layer_counts[phase],
            "hidden_dim": hidden_dims[phase],
            "num_experts": expert_counts[phase],
            "num_adapters": adapter_counts[phase],
            "val_ppl": val_ppls[phase],
        },
    )
```

---

## Next Steps

1. **Data Preparation:** Download and process Pile subset
2. **Hardware Setup:** Configure 256-GPU cluster or reduced cluster
3. **Run Phase 0:** Validate base training works
4. **Progressive Rollout:** Run phases 1-5 sequentially
5. **Validation:** Verify all scale metrics at end
6. **Integration:** Package to Crystal, register in model registry
7. **Deployment:** Serve via inference engine

---

## References

- [Transformers: Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- [Mixture of Experts: Scaling Transformer Models](https://arxiv.org/abs/2101.03961)
- [LoRA: Low-Rank Adaptation](https://arxiv.org/abs/2106.09685)
- [Co-training and Self-training for Semi-supervised Learning](https://arxiv.org/abs/0908.4213)

