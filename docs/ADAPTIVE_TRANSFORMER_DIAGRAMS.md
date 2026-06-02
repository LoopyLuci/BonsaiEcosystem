# Bonsai Adaptive Transformer: Visual Diagrams & Architecture

**Version:** 1.0  
**Date:** 2026-06-01  

---

## 1. High-Level System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Bonsai Inference Engine                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │         Adaptive Transformer Forward Pass                │  │
│  ├──────────────────────────────────────────────────────────┤  │
│  │                                                          │  │
│  │  Input Tokens → Embedding + Position Encoding           │  │
│  │                          ↓                              │  │
│  │       ╔═══════════════════════════════════════╗         │  │
│  │       ║   Adaptive Layer Stack (32 layers)   ║         │  │
│  │       ╠═══════════════════════════════════════╣         │  │
│  │       ║  FOR i = 0 TO 31:                    ║         │  │
│  │       ║    IF layer_mask[i]:                 ║         │  │
│  │       ║      • Adaptive Attention ────────┐  ║         │  │
│  │       ║      • Adaptive FFN / MoE ────────┼─ ║         │  │
│  │       ║      • LoRA Adapters ─────────────┤  ║         │  │
│  │       ║      • Residual + LayerNorm ──────┘  ║         │  │
│  │       ║    ELSE:                             ║         │  │
│  │       ║      identity_pass_through()         ║         │  │
│  │       ╚═══════════════════════════════════════╝         │  │
│  │                          ↓                              │  │
│  │                Output Projection                        │  │
│  │                          ↓                              │  │
│  │                     LM Head                             │  │
│  │                          ↓                              │  │
│  │                     Logits                              │  │
│  │                                                          │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
│  ┌──────────────┬──────────────┬──────────────┐                │
│  │ LoRA Registry│ KV Cache     │ Model Params │                │
│  │ • Adapter-A  │ • Layer 0 K/V│ • Base W     │                │
│  │ • Adapter-B  │ • Layer 1 K/V│ • Expert vec │                │
│  │ • ...        │ • ...        │ • ...        │                │
│  └──────────────┴──────────────┴──────────────┘                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Adaptive Layer Stack Detail

```
┌──────────────────────────────────────────────────────────────┐
│                    Single Adaptive Layer i                   │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Input x [batch, seq_len, hidden_dim]                       │
│            ↓                                                │
│  x_residual = x.clone()                                     │
│            ↓                                                │
│  ╔═══════════════════════════════════════════════════════╗  │
│  ║ IF layer_mask[i] == False: SKIP THIS ENTIRE BLOCK     ║  │
│  ║ (just do: x = x_residual, continue)                   ║  │
│  ╚═══════════════════════════════════════════════════════╝  │
│            ↓                                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Adaptive Attention                                  │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │ • Project Q, K, V (width_factor aware)             │    │
│  │ • Use first N heads instead of all 32              │    │
│  │ • Attention scores = (Q @ K^T) / sqrt(head_dim)   │    │
│  │ • Output = softmax(scores) @ V                     │    │
│  │ • Width: active_heads = 32 * width_factor          │    │
│  └─────────────────────────────────────────────────────┘    │
│            ↓                                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Adaptive FFN OR MoE Routing                         │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │ FFN:                                                │    │
│  │ • x_proj = x @ W_up[:, :ffn_dim*width]            │    │
│  │ • x_proj = gelu(x_proj)                            │    │
│  │ • output = x_proj @ W_down[:, :hidden*width]      │    │
│  │                                                     │    │
│  │ MoE (if MoE layer):                                │    │
│  │ • router_logits = x @ W_router                     │    │
│  │ • Apply expert_mask → disabled = -∞                │    │
│  │ • top_k = select_top_k(softmax(logits))            │    │
│  │ • output = sum(gate[i] * expert[i](x))            │    │
│  └─────────────────────────────────────────────────────┘    │
│            ↓                                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ LoRA Adapter Composition                            │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │ FOR adapter in active_adapters:                     │    │
│  │   IF adapter.applies_to_layer(i):                  │    │
│  │     x_proj = x @ A[i]  [batch, seq, rank]         │    │
│  │     delta = x_proj @ B[i]^T  [batch, seq, hidden] │    │
│  │     x = x + delta * (alpha / rank)                 │    │
│  └─────────────────────────────────────────────────────┘    │
│            ↓                                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Residual Connection + LayerNorm                     │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │ x = x + x_residual                                 │    │
│  │ x = LayerNorm(x)                                   │    │
│  └─────────────────────────────────────────────────────┘    │
│            ↓                                                │
│  Output [batch, seq_len, hidden_dim]                        │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

---

## 3. Width Scaling Visualization

```
Base Model (100% width)
────────────────────────────────────────────────────
Hidden Dim: 4096
Attention Heads: 32 (128 dims each)
FFN Dim: 16384

       Q ──→ [32 heads] ──→ output
       K ──→ [32 heads] ──→
       V ──→ [32 heads] ──→


50% Width Scaling (width_factor=0.5)
────────────────────────────────────────────────────
Hidden Dim: 2048 (use first 2048 dims only)
Attention Heads: 16 (128 dims each)
FFN Dim: 8192

       Q ──→ [16 heads] ──→ output
       K ──→ [16 heads] ──→
       V ──→ [16 heads] ──→
       
       ↑ Only compute first 2048 dimensions


25% Width Scaling (width_factor=0.25)
────────────────────────────────────────────────────
Hidden Dim: 1024
Attention Heads: 8 (128 dims each)
FFN Dim: 4096

       Q ──→ [8 heads] ──→ output
       K ──→ [8 heads] ──→
       V ──→ [8 heads] ──→


200% Width Scaling (width_factor=2.0, rare)
────────────────────────────────────────────────────
Hidden Dim: 8192 (expand via interpolation)
Attention Heads: 64 (128 dims each)
FFN Dim: 32768

       Q ──→ [64 heads] ──→ output
       K ──→ [64 heads] ──→
       V ──→ [64 heads] ──→
```

---

## 4. Layer Masking Pattern Examples

```
Pattern A: Uniform Skip (every other)
──────────────────────────────────────
Layer:   0  1  2  3  4  5  6  7 ... 30 31
Mask:    1  0  1  0  1  0  1  0 ... 1  0
         █  ░  █  ░  █  ░  █  ░ ... █  ░

Active Layers: 16/32 = 50%
Speedup: ~2x
Quality: ~90-95%


Pattern B: Front-Heavy (first N active)
──────────────────────────────────────
Layer:   0  1  2  3  4  5  6  7 ... 30 31
Mask:    1  1  1  1  1  1  1  1 ... 0  0
         █  █  █  █  █  █  █  █ ... ░  ░

Active Layers: 24/32 = 75%
Speedup: ~1.3x
Quality: ~95%


Pattern C: Aggressive Skip (mostly off)
──────────────────────────────────────
Layer:   0  1  2  3  4  5  6  7 ... 30 31
Mask:    1  0  0  0  0  0  1  0 ... 0  1
         █  ░  ░  ░  ░  ░  █  ░ ... ░  █

Active Layers: 3/32 = 9%
Speedup: ~10x
Quality: ~60-70%


Pattern D: Learning-Based (trained)
──────────────────────────────────────
Layer:   0  1  2  3  4  5  6  7 ... 30 31
Mask:    1  1  0  1  0  0  1  1 ... 0  1
         █  █  ░  █  ░  ░  █  █ ... ░  █

Active Layers: varies
Speedup: variable
Quality: optimized per task
```

---

## 5. Expert Routing with Masking

```
Standard Expert Routing (all experts available)
───────────────────────────────────────────────────────────

Input x [batch=2, seq_len=4]
           ↓
    Router Network
    [output: 1024 logits per token]
           ↓
    ┌─ Token 1 logits: [0.5, 0.2, 0.1, ..., -0.3] (1024 values)
    ├─ Token 2 logits: [0.1, 0.8, 0.3, ..., 0.2]
    ├─ Token 3 logits: [0.9, -0.1, 0.0, ..., 0.5]
    └─ Token 4 logits: [0.3, 0.4, 0.6, ..., 0.1]
           ↓
    Top-K=2 selection
    ┌─ Token 1: experts [0, 1]
    ├─ Token 2: experts [1, 2]
    ├─ Token 3: experts [0, 4]
    └─ Token 4: experts [2, 3]
           ↓
    Gate weights (softmax)
    Apply selected experts
           ↓
    Output [batch=2, seq_len=4, hidden_dim]


With Expert Masking (only 512 experts enabled)
───────────────────────────────────────────────────────────

expert_masks = [True]*512 + [False]*512  ← last 512 disabled

Router logits:
  ┌─ Token 1: [0.5, ..., -0.3, ░░░░░░░░ -∞ ░░░░░░░░]
  │           (first 512 ↑)   (last 512 all set to -∞)
  ├─ Token 2: [0.1, ..., 0.2, ░░░░░░░░ -∞ ░░░░░░░░]
  ├─ Token 3: [0.9, ..., 0.5, ░░░░░░░░ -∞ ░░░░░░░░]
  └─ Token 4: [0.3, ..., 0.1, ░░░░░░░░ -∞ ░░░░░░░░]
           ↓
Top-K=2 from enabled experts only:
  ┌─ Token 1: only selected from [0..511]
  ├─ Token 2: only selected from [0..511]
  ├─ Token 3: only selected from [0..511]
  └─ Token 4: only selected from [0..511]
           ↓
Result: Never uses experts 512-1023
Memory per expert doubles (fewer experts for same tokens)
```

---

## 6. LoRA Adapter Composition

```
Single Adapter
──────────────────────────────────────────
Base Model Weight: W [hidden_dim, hidden_dim] = [4096, 4096]
Trainable LoRA:
  • A [hidden_dim, rank] = [4096, 16]
  • B [hidden_dim, rank] = [4096, 16]

Forward:
  x [batch, seq, hidden]
    ├─ base_out = x @ W^T  [batch, seq, hidden]
    ├─ x_proj = x @ A  [batch, seq, rank]  ← cheap!
    ├─ lora_delta = x_proj @ B^T  [batch, seq, hidden]  ← cheap!
    └─ output = base_out + lora_delta * (alpha / rank)


Stacked Adapters (3 adapters)
──────────────────────────────────────────
Adapter A: rank=16, alpha=16
Adapter B: rank=32, alpha=32
Adapter C: rank=8, alpha=8

Forward:
  x [batch, seq, hidden]
    ├─ base_out = x @ W^T
    ├─ delta_A = (x @ A) @ B^T * (16/16)
    ├─ delta_B = (x @ B) @ B^T * (32/32)
    ├─ delta_C = (x @ C) @ B^T * (8/8)
    └─ output = base_out + delta_A + delta_B + delta_C

Total matmuls: 1 + 2*3 = 7 (vs 1 without LoRA)
Cost: O(hidden * rank) instead of O(hidden^2)
```

---

## 7. Position Encoding: RoPE Dimension-Aware

```
Standard RoPE (full dimension)
────────────────────────────────────────────
hidden_dim = 4096, num_heads = 32, head_dim = 128

Frequencies:
  θ_i = 10000^(-2i/128) for i in [0, 64]
  
Position encoding matrix [seq_len, 128]:
  ┌ [sin(pos*θ₀), cos(pos*θ₀), sin(pos*θ₁), cos(pos*θ₁), ...]
  │ [sin(pos*θ₀), cos(pos*θ₀), sin(pos*θ₁), cos(pos*θ₁), ...]
  └ ...

Apply to Q, K via rotation:
  Q_rotated = rotate(Q, PE)
  K_rotated = rotate(K, PE)


With Width Scaling (width_factor=0.5)
────────────────────────────────────────────
hidden_dim = 2048 (only first 2048 dims active)
active_head_dim = 64 (half the heads)

Frequencies:
  θ_i = 10000^(-2i/64) for i in [0, 32]
  ↑ Different! More aggressive frequency decay
  
Position encoding matrix [seq_len, 64]:
  ┌ [sin(pos*θ₀), cos(pos*θ₀), sin(pos*θ₁), cos(pos*θ₁), ...]
  │ [sin(pos*θ₀), cos(pos*θ₀), sin(pos*θ₁), cos(pos*θ₁), ...]
  └ ...

Key: Adjust frequency to match actual active dimensions
This ensures positional info is consistent across widths
```

---

## 8. KV Cache State Machine

```
Prefill Phase (first forward)
─────────────────────────────
Input: [batch, seq_len=512, hidden]
       ↓
FOR layer 0..31:
  IF layer_mask[i]:
    Compute Q, K, V for entire sequence
    K_cache[i] = K [batch, num_heads, 512, head_dim]
    V_cache[i] = V [batch, num_heads, 512, head_dim]

State: caches populated


Decode Phase (token by token generation)
─────────────────────────────────────────
FOR step in range(max_tokens):
  Input: [batch, 1, hidden]  ← single token!
         ↓
  FOR layer 0..31:
    IF layer_mask[i]:
      Compute Q, K_new, V_new for single token
      Append to cache:
        K_cache[i][:, :, position+1, :] = K_new
        V_cache[i][:, :, position+1, :] = V_new
      Use full K_cache, V_cache in attention
      ↓
    ELSE:
      Skip (don't append to cache)
  
  Output logits → sample next token


Mask Change Mid-Generation
──────────────────────────
Old mask: [1,1,0,1,0,...]
New mask: [1,0,1,1,0,...]
             ↑ changed!

Cache invalidation:
  FOR i where old_mask[i] != new_mask[i]:
    K_cache[i].clear()
    V_cache[i].clear()
    
Continue generation with updated mask
```

---

## 9. Batch Processing: Mixed Scales

```
Strategy 1: Separate Batches (Simple)
──────────────────────────────────────
Input batch: 8 requests
  ├─ Req 1: width=1.0, layers=[1]*32
  ├─ Req 2: width=0.5, layers=[1]*32
  ├─ Req 3: width=1.0, layers=[1]*32
  ├─ Req 4: width=0.5, layers=[1,0]*16
  ├─ Req 5: width=0.5, layers=[1]*32
  ├─ Req 6: width=0.5, layers=[1,0]*16
  ├─ Req 7: width=1.0, layers=[1]*32
  └─ Req 8: width=0.5, layers=[1]*32

Group by config:
  ┌─ Group A: {width=1.0, layers=[1]*32} → [1,3,7]
  ├─ Group B: {width=0.5, layers=[1]*32} → [2,5,8]
  └─ Group C: {width=0.5, layers=[1,0]*16} → [4,6]

Process separately:
  forward([1,3,7], config_A) → batch=3
  forward([2,5,8], config_B) → batch=3
  forward([4,6], config_C) → batch=2

Merge results
  Pros: No overhead, simple logic
  Cons: Reduced parallelism if configs diverse


Strategy 2: Unified Batch (Complex)
──────────────────────────────────────
Keep all 8 requests together

┌─────────────────────────────────────┐
│ FOR layer_idx in 0..31:             │
│   FOR request_idx in 0..7:          │
│     IF layer_masks[request][layer]: │
│       compute(request)              │
│     ELSE:                           │
│       skip(request)                 │
│   Merge results                     │
└─────────────────────────────────────┘

Use torch.where() for conditional execution:
  output = torch.where(active, computed, identity)

  Pros: Better GPU utilization, parallelism
  Cons: More complex, overhead from torch.where
```

---

## 10. Training vs Inference

```
INFERENCE MODE
──────────────────────────────────────────────
layer_masks: bool [True/False]
             ↓
Hard selection: skip entire layer or not

        ┌─ Layer Active ──→ compute ──→ output
        │
input ──┤
        │
        └─ Layer Inactive ──→ identity ──→ output

No gradients needed
Fast, deterministic


TRAINING MODE
──────────────────────────────────────────────
layer_masks: float [0.0 to 1.0]
             ↓
Soft gating: scale layer contribution

        ┌─ Layer ──→ compute ──→ x * mask[i] ──→
        │                                        
input ──┤                                        ├─→ output
        │                                        
        └─ Identity ──→ x * (1 - mask[i]) ──→

Gradient flows through both paths:
  ∂output/∂mask[i] = (x_layer - x_identity)
  
Enables learning which layers are important
Model can learn to skip unnecessary layers
```

---

## 11. Memory Layout: KV Cache

```
KV Cache Shape
──────────────────────────────────────────────
Single Layer KV Cache:
  K: [batch=8, num_heads=32, seq_len=2048, head_dim=128]
  V: same

Memory per layer (fp16):
  K: 8 * 32 * 2048 * 128 * 2 bytes = 134 MB
  V: 134 MB
  Total per layer: 268 MB

32 layers:
  Total KV cache: 32 * 268 MB = 8.5 GB


With Width Scaling (50%)
──────────────────────────────────────────────
Active heads: 16 (not 32)

K: [batch=8, num_heads=16, seq_len=2048, head_dim=128]
Memory per layer: 134 MB (half!)
32 layers: 4.2 GB (half!)


Allocation Strategy
──────────────────────────────────────────────
Pre-allocate full size at startup:
  max_seq_len = 4096
  max_batch = 16
  K: [16, 32, 4096, 128] fp16 = 1.0 GB

At inference:
  batch=8, seq_len=2048 → use K[:8, :, :2048, :]
  
Or dynamic allocation:
  Allocate based on actual config
  Save memory, but slower
```

---

## 12. Comparative Performance

```
Model Size vs Speed Tradeoff
────────────────────────────────────────────
Full Model
  Hidden: 4096, Heads: 32, Layers: 32
  Speed: 1.0x (baseline)
  Quality: 100%
  Memory: 1.0x
       ▓▓▓▓▓▓▓▓▓▓

75% Width + All Layers
  Hidden: 3072, Heads: 24, Layers: 32
  Speed: 1.4x
  Quality: 98%
  Memory: 0.75x
       ▓▓▓▓▓▓▓

50% Width + All Layers
  Hidden: 2048, Heads: 16, Layers: 32
  Speed: 3.0x
  Quality: 90%
  Memory: 0.5x
       ▓▓▓▓

50% Width + 50% Layers
  Hidden: 2048, Heads: 16, Layers: 16
  Speed: 6.0x
  Quality: 80%
  Memory: 0.25x
       ▓▓

25% Width + 25% Layers
  Hidden: 1024, Heads: 8, Layers: 8
  Speed: 15.0x
  Quality: 60%
  Memory: 0.1x
       ▓
```

---

## 13. Data Flow: Single Token Generation

```
Generation Step (single token)
──────────────────────────────────────────────
Cache Length: 256 tokens (previous context)

Input: token_id [1024]
  ├─ Embedding: [hidden_dim=4096]
  ├─ Add Position Encoding (pos=256): [4096]
  ├─ LayerNorm: [4096]
  │
  └─ Layer Stack:
      ├─ Layer 0 (active):
      │  ├─ Attention:
      │  │  ├─ Q projection: [4096] → [4096]
      │  │  ├─ K projection: [4096] → [4096]
      │  │  │  ├─ Cache K[:, :, 0:256, :] from prefill
      │  │  │  └─ Append new K[:, :, 256, :] to cache
      │  │  ├─ V projection: [4096] → [4096]
      │  │  │  ├─ Cache V[:, :, 0:256, :] from prefill
      │  │  │  └─ Append new V[:, :, 256, :] to cache
      │  │  ├─ Attention scores: Q @ K_full^T [1, 257]
      │  │  │  (256 cached + 1 new)
      │  │  └─ Output: [4096]
      │  │
      │  └─ FFN:
      │     ├─ x_proj: [4096] → [16384]
      │     ├─ GELU: [16384]
      │     └─ Linear: [16384] → [4096]
      │
      ├─ Layer 1 (inactive):
      │  └─ Skip (identity pass)
      │
      └─ ... (layers 2-31)
      │
      └─ Output: [4096]
  │
  ├─ LayerNorm: [4096]
  ├─ Output Projection: [4096]
  └─ LM Head: [4096] → [vocab_size=128256]
       │
       └─ Logits (pick top token or sample)

Memory accessed:
  • Input embedding: [hidden_dim]
  • KV cache: [num_layers, 2, num_heads, 256, head_dim]
  • Weights: [num_layers, hidden_dim, hidden_dim]
           (loaded once, not per token)

Latency bottleneck: Memory bandwidth (KV cache reads)
```

---

## 14. Error Cases & Handling

```
┌─ All experts masked
│  └─→ softmax([-∞, -∞, ...]) = undefined
│       FIX: Ensure min 1 expert enabled
│
├─ All layers masked
│  └─→ Output = embedding (no transformation)
│       FIX: Not really an error, just use embedding as-is
│
├─ Width factor mismatch
│  └─→ Dimension mismatch in matmul
│       FIX: Validate width_factor in [0.1, 2.0]
│
├─ LoRA on masked layer
│  └─→ Adapter computes delta on masked input
│       FIX: Check layer_mask before applying adapter
│
├─ KV cache not cleared on mask change
│  └─→ Cached values stale, wrong output
│       FIX: Call cache.invalidate_on_mask_change()
│
├─ Layer norm on zero-variance input
│  └─→ Division by zero in normalization
│       FIX: Add eps term, skip norm if all zeros
│
└─ Extremely sparse masks (1% layers active)
   └─→ Numerical instability in residual accumulation
        FIX: Periodic normalization, gradient clipping
```

---

**Document End**

All diagrams illustrate design from ADAPTIVE_TRANSFORMER_FORWARD_PASS.md.

