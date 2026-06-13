# Bonsai Adaptive Transformer: Quick Reference & Checklists

**Version:** 1.0  
**Date:** 2026-06-01  
**Target:** Rapid implementation and debugging

---

## Quick Start: One-Page Summary

### The Core Idea

Standard transformer forward pass:
```
input → embedding → layer1 → layer2 → ... → layerN → output
```

Adaptive transformer:
```
input → embedding → [masked layers, width scaling, expert routing, LoRA] → output
```

**Three key innovations:**
1. **Layer masking** — skip inactive layers without breaking residuals
2. **Width scaling** — use 25%, 50%, or 200% of hidden dimensions
3. **LoRA composition** — stack adapters without copying weights

### Pseudo-code

```python
def forward(x, layer_masks, width_factor, expert_masks, adapters):
    for i in range(num_layers):
        if not layer_masks[i]:
            continue  # Skip inactive layer
        
        # Compute with active dimensions only
        active_dim = int(hidden_dim * width_factor)
        x = attention(x[:, :active_dim])  # Use first N heads
        x = ffn(x[:, :active_dim])        # Use first N neurons
        
        # Route through active experts
        x = moe(x, expert_masks)
        
        # Stack adapters
        for adapter in adapters:
            x = x + adapter(x)
        
        # Residual + norm
        x = x + residual
        x = norm(x)
    
    return x
```

---

## Design Patterns

### Pattern 1: Masking (Skip Inactive Layers)

**Problem:** If we skip layers, how do we preserve gradients and prevent training collapse?

**Solution:** Use **residual connections** for inactive layers.

```python
if layer_mask[i]:
    x = transformer_layer(x)
    x = x + residual
    x = norm(x)
else:
    x = residual  # Identity pass-through
```

### Pattern 2: Width Scaling (Dimension Truncation)

**Problem:** How do we scale from 4096-dim to 2048-dim without retraining?

**Solution:** Use **column slicing** from weight matrices.

```python
# Base weight: [4096, 4096]
# Active weight (50%): [4096, 2048]
W_active = W[:, :2048]
output = x @ W_active  # [batch, seq, 2048]
```

**Key:** No weight copying. Use views/indices on original W.

### Pattern 3: Expert Routing (Selective MoE)

**Problem:** How do we disable experts without recompiling?

**Solution:** **Mask router logits** to -∞ for disabled experts.

```python
router_logits = x @ W_router  # [batch, seq, 1024]

# Mask: set disabled expert logits to -inf
for i in range(1024):
    if not expert_mask[i]:
        router_logits[:, :, i] = -1e9

# Top-K selection now ignores masked experts
top_k = select_top_k(router_logits, k=2)
```

### Pattern 4: LoRA Composition (Adapter Stacking)

**Problem:** How do we stack multiple adapters without quadratic overhead?

**Solution:** Use **efficient low-rank projection**: (x @ A) @ B^T

```python
# Inefficient: compute full delta W = B @ A^T [d x d]
# delta = x @ (B @ A^T)  — O(d^2) memory

# Efficient: batch multiply [r << d]
# x @ A → [batch, r]
# @ B^T → [batch, d]
x_proj = x @ A  # [batch, seq, rank]
delta = x_proj @ B.T  # [batch, seq, hidden_dim]

output = x + delta
```

**Complexity:** O(batch * seq * d * r) instead of O(d^2)

---

## Configuration Presets

### Preset 1: Speed (50% Width, 50% Layers)

For fast inference (4x speedup):

```python
config = AdaptiveConfig(
    layer_masks=[True, False] * 16,  # Alternate active/inactive
    width_factor=0.5,                 # Half the dimensions
    expert_masks=expert_mask[::2],    # Every other expert
    top_k_experts=1,                  # Single expert routing
)
```

**Performance:** ~4-5x faster, ~30% quality loss

### Preset 2: Balanced (75% Width, 75% Layers)

For good trade-off:

```python
config = AdaptiveConfig(
    layer_masks=[True] * 24 + [False] * 8,  # 24/32 layers active
    width_factor=0.75,
    expert_masks=[True] * 512 + [False] * 512,  # Half experts
    top_k_experts=2,
)
```

**Performance:** ~2x faster, ~10% quality loss

### Preset 3: Quality (100% Width, 100% Layers)

Standard transformer (baseline):

```python
config = AdaptiveConfig(
    layer_masks=[True] * 32,
    width_factor=1.0,
    expert_masks=[True] * 1024,
    top_k_experts=2,
)
```

**Performance:** 1x (baseline), 0% quality loss

### Preset 4: Mobile (25% Width, 25% Layers)

For edge devices:

```python
config = AdaptiveConfig(
    layer_masks=[True] * 8,  # 8/32 layers
    width_factor=0.25,
    expert_masks=[True] * 128,  # 128/1024 experts
    top_k_experts=1,
)
```

**Performance:** ~15-20x faster, ~40-50% quality loss

---

## Numerical Stability Checklist

When implementing, watch for these issues:

### Issue 1: Attention Softmax on Masked Experts

**Problem:** If all experts are masked, router logits are all -∞, softmax → NaN.

**Solution:** Ensure at least 1 expert is always enabled.

```python
num_enabled = expert_mask.sum()
assert num_enabled >= 1, "At least 1 expert must be enabled"
```

### Issue 2: LayerNorm on Skipped Layers

**Problem:** If all layers are skipped, input never gets normalized.

**Solution:** Apply norm before first layer if many layers are inactive.

```python
if layer_masks.sum() < num_layers // 2:
    # More than half the layers are skipped
    x = layer_norm(x)  # Normalize input
```

### Issue 3: Width Scaling Beyond 1.0

**Problem:** If width_factor = 2.0, we're expanding dimensions. What goes in the new dims?

**Solution:** Use learned projection or zero-padding (with interpolation for PE).

```python
if width_factor > 1.0:
    # Expand via learned projection
    x = x @ W_expand  # [batch, seq, hidden_dim * 2]
else:
    # Shrink via column slicing
    active_dim = int(hidden_dim * width_factor)
    x = x[:, :active_dim]
```

### Issue 4: Accumulating Errors in Deep Networks

**Problem:** With many masked layers, residual connections might accumulate numerical errors.

**Solution:** Periodically normalize or use gradient clipping.

```python
# Clip activations if they get too large
max_norm = 100.0
if x.norm() > max_norm:
    x = x / x.norm() * max_norm
```

---

## Performance Tuning Guide

### Bottleneck 1: Conditional Branches (GPU)

**Problem:** CUDA kernels don't like `if` statements.

**Solution:** Use vectorized masking.

```python
# Bad: conditional in loop
for i in range(num_layers):
    if layer_mask[i]:
        x = layer(x)

# Good: vectorized
x = layer(x)
x = torch.where(layer_mask, x, x_residual)
```

### Bottleneck 2: Expert Routing (Scatter/Gather)

**Problem:** Selecting different experts per token requires gather operations (slow).

**Solution:** Use custom CUDA kernels or batch experts by activation pattern.

```cpp
// Custom kernel for expert routing
__global__ void expert_route_kernel(
    const float* x,
    const int* expert_indices,  // which expert per token
    const float* experts,
    float* output
) {
    int token_idx = blockIdx.x;
    int expert_idx = expert_indices[token_idx];
    
    // Fetch and apply expert[expert_idx] to token_idx
    // ...
}
```

### Bottleneck 3: KV Cache Memory

**Problem:** KV cache can be 1-2 GB for large models.

**Solution:** Use int8/fp16 quantization or dynamic cache allocation.

```python
# Quantized KV cache
K_cached = quantize_to_int8(K)  # [batch, heads, seq, head_dim] → 1/2 memory
V_cached = quantize_to_int8(V)

# Reconstruct during attention
K_float = dequantize(K_cached)
V_float = dequantize(V_cached)
```

### Bottleneck 4: LoRA Composition

**Problem:** Multiple adapters mean multiple matmuls (slow).

**Solution:** Fuse adapters or batch compute.

```python
# Bad: sequential
for adapter in adapters:
    x = x + adapter(x)

# Good: batch compute all deltas then add
deltas = torch.zeros_like(x)
for adapter in adapters:
    deltas = deltas + adapter(x)
x = x + deltas
```

---

## Testing & Validation

### Test 1: Output Shape

```python
def test_output_shape():
    model = AdaptiveTransformer(...)
    x = torch.randn(2, 8, 4096)  # [batch, seq_len, hidden_dim]
    
    y = model(x, layer_masks=[True]*32, width_factor=1.0)
    
    assert y.shape == (2, 8, 4096), f"Expected (2, 8, 4096), got {y.shape}"
```

### Test 2: No NaN/Inf

```python
def test_numerical_stability():
    model = AdaptiveTransformer(...)
    x = torch.randn(2, 8, 4096)
    
    y = model(x, layer_masks=[True, False]*16, width_factor=0.5)
    
    assert not torch.isnan(y).any(), "Output contains NaN"
    assert not torch.isinf(y).any(), "Output contains Inf"
```

### Test 3: Gradient Flow

```python
def test_gradient_flow():
    model = AdaptiveTransformer(...)
    x = torch.randn(2, 8, 4096, requires_grad=True)
    
    y = model(x, layer_masks=[True]*32)
    loss = y.sum()
    loss.backward()
    
    assert x.grad is not None, "No gradient for input"
    assert model.embedding.weight.grad is not None, "No gradient for embedding"
```

### Test 4: Equivalence (Masked == Unmasked for Active Layers)

```python
def test_equivalence():
    model = AdaptiveTransformer(...)
    x = torch.randn(1, 8, 4096)
    
    # Forward with all layers active
    y_full = model(x, layer_masks=[True]*32)
    
    # Forward with first 16 layers masked (but should give same result if disabled)
    y_partial = model(x, layer_masks=[True]*16 + [False]*16)
    
    # These won't be exactly equal (due to layer norm differences),
    # but should be correlated
    correlation = torch.nn.functional.cosine_similarity(y_full.flatten(), y_partial.flatten(), dim=0)
    assert correlation > 0.7, f"Correlation too low: {correlation:.2f}"
```

### Test 5: Memory Usage

```python
def test_memory_usage():
    import psutil
    
    model = AdaptiveTransformer(...)
    
    # Measure memory for full model
    torch.cuda.reset_peak_memory_stats()
    x = torch.randn(8, 512, 4096).cuda()
    y = model(x, layer_masks=[True]*32, width_factor=1.0)
    full_memory = torch.cuda.max_memory_allocated() / 1e9  # GB
    
    # Measure memory for scaled model
    torch.cuda.reset_peak_memory_stats()
    y = model(x, layer_masks=[True]*16, width_factor=0.5)
    scaled_memory = torch.cuda.max_memory_allocated() / 1e9
    
    print(f"Full: {full_memory:.2f} GB, Scaled: {scaled_memory:.2f} GB")
    assert scaled_memory < full_memory * 0.6, "Scaled model not significantly smaller"
```

---

## Common Pitfalls & Fixes

| Pitfall | Symptom | Fix |
|---------|---------|-----|
| **No residuals for skipped layers** | Gradients die, training collapses | Always: `x = x + residual` for inactive layers |
| **Layer norm on masked input** | NaN softmax | Skip norm for fully masked layers or use RMSNorm |
| **All experts masked** | Router softmax → NaN | Always enable ≥1 expert |
| **Weight sharing between widths** | Memory leak, incorrect routing | Use view/slice, not copy |
| **Positional encoding mismatch** | Attention breaks with width scaling | Truncate/interpolate PE for active dims only |
| **KV cache not invalidated** | Cached values stale on mask change | Call `cache.invalidate_on_mask_change()` |
| **LoRA applied to inactive layers** | Unused computation | Check `adapter.applies_to_layer()` before forward |
| **Gradient flow blocked** | Model doesn't learn | Ensure `requires_grad=True` for trainable params |

---

## Reference: Activation Patterns

### Activation Map: Which Components Compute?

```
Layer Mask = True/False
Width = 25% / 50% / 100% / 200%
Expert Mask = 50 / 256 / 512 / 1024 enabled

Example: Layer 5, Width 50%, Experts 512

    Embedding ──────────────┐
                            ↓
    Layer 1 (active)   [2048 dims]
    ├─ Attention      active
    ├─ FFN            active
    └─ LoRA           if in layer_indices
                            ↓
    Layer 2-4          [skip]
                            ↓
    Layer 5 (active)   [2048 dims]
    ├─ Attention      active
    ├─ MoE Router     512/1024 experts
    └─ LoRA           if in layer_indices
                            ↓
    Layer 6-32         [skip or active]
                            ↓
    Output Proj        [2048 → 4096]
                            ↓
    LM Head            [4096 → vocab]
```

---

## Integration Checklist

### Pre-Implementation

- [ ] Read full design document (ADAPTIVE_TRANSFORMER_FORWARD_PASS.md)
- [ ] Review PyTorch reference implementation
- [ ] Set up test infrastructure
- [ ] Define success criteria (speed/quality trade-offs)

### Implementation

- [ ] Implement `AdaptiveConfig` struct
- [ ] Implement position encoding (RoPE)
- [ ] Implement `AdaptiveAttention` layer
- [ ] Implement `AdaptiveFFN` layer
- [ ] Implement `AdaptiveMoE` routing
- [ ] Integrate KV cache
- [ ] Add LoRA composition
- [ ] Write unit tests (10+ test cases)
- [ ] Benchmark performance
- [ ] Profile memory usage
- [ ] Optimize critical paths

### Integration with bonsai-inference

- [ ] Add module to `crates/bonsai-inference/src/`
- [ ] Update `InferenceEngine` to use adaptive forward
- [ ] Create Rust FFI for llama.cpp integration
- [ ] Extend llama.cpp with adaptive layer support
- [ ] Test end-to-end inference pipeline
- [ ] Document API for users

### Production Hardening

- [ ] Add error handling (all Result<T>)
- [ ] Validate inputs (mask shapes, dimensions)
- [ ] Log all major decisions (for debugging)
- [ ] Write integration tests
- [ ] Performance regression tests
- [ ] Documentation (API docs, examples)

---

## Debugging Tips

### Problem: Output is all zeros

**Causes:**
1. All layers masked
2. Width scaling caused dimension mismatch
3. LayerNorm failed

**Debug:**
```python
# Check layer masks
print("Active layers:", config.layer_masks.count(True))

# Check dimensions
print(f"Input shape: {x.shape}")
print(f"Output shape: {output.shape}")
print(f"Active hidden dim: {int(hidden_dim * width_factor)}")

# Check for zero activations
print(f"Output min/max/mean: {output.min()}/{output.max()}/{output.mean()}")
```

### Problem: NaN in gradients during training

**Causes:**
1. Softmax on all -inf (masked experts)
2. Layer norm on zero-variance input
3. Numerical instability in LoRA

**Debug:**
```python
# Enable anomaly detection
torch.autograd.set_detect_anomaly(True)

# Track intermediate values
for i, layer in enumerate(model.layers):
    x_before = x.clone()
    x = layer(x)
    print(f"Layer {i}: {x.min():.4f} to {x.max():.4f}, NaN: {torch.isnan(x).any()}")
```

### Problem: Out of memory during inference

**Causes:**
1. KV cache too large
2. Weight matrices duplicated
3. Batch size too large

**Debug:**
```python
import torch
print(f"Allocated: {torch.cuda.memory_allocated() / 1e9:.2f} GB")
print(f"Reserved: {torch.cuda.memory_reserved() / 1e9:.2f} GB")

# Clear cache
torch.cuda.empty_cache()

# Use smaller batch or reduce seq_len
```

---

## Performance Targets

| Configuration | Speed | Quality | Memory |
|---------------|-------|---------|--------|
| Full (100%, all layers) | 1.0x | 100% | 1.0x |
| Balanced (75%, 75%) | 1.5x | 95% | 0.65x |
| Fast (50%, 50%) | 3-4x | 80% | 0.40x |
| Mobile (25%, 25%) | 10-15x | 60% | 0.15x |

**Notes:**
- Speed assumes no custom kernels
- Quality measured via downstream task accuracy
- Memory includes KV cache for seq_len=2048

---

**Document End**

