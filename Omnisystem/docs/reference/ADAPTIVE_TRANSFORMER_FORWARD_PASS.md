# Bonsai Adaptive Transformer: Core Forward Pass Design

**Status:** Design Phase  
**Target:** Integration with bonsai-inference engine  
**Version:** 1.0  
**Author:** Claude Code  
**Date:** 2026-06-01

---

## Executive Summary

This document specifies the core transformer forward pass architecture for Bonsai's adaptive scaling system. The forward pass must support:

1. **Variable Layer Masks** — Skip/activate layers dynamically
2. **Width Scaling** — Use only first N attention heads and hidden dimensions
3. **Expert Routing** — Selective mixture-of-experts with dynamic expert pools
4. **LoRA Adapters** — Stack multiple low-rank updates without weight modification
5. **Zero-Copy Masking** — No weight copying, only routing/masking decisions

All components operate **without recompilation** and **without modifying base weights**. The design targets inference at <10% overhead vs. non-adaptive forward pass.

---

## 1. Adaptive Forward Pass Architecture

### 1.1 High-Level Flow

```
Input Tokens (sequence_length, hidden_dim)
    ↓
Token Embedding + Position Encoding (respecting width_factor)
    ↓
FOR i = 0 TO num_layers-1:
    IF layer_mask[i] == True:
        Adaptive Attention (with width_factor)
        ↓
        Adaptive FFN (with width_factor)
        ↓
        LoRA Adapter Stack (if adapters[i] exists)
        ↓
        Residual Connection + LayerNorm
    ELSE:
        Skip Layer (apply skip residual connection)
    ↓
    MAYBE invalidate KV-cache for this position if mask changed
    ↓
Output (sequence_length, active_hidden_dim)
    ↓
Output Projection (active_hidden_dim → full_hidden_dim)
    ↓
LM Head
    ↓
Logits (vocabulary_size)
```

### 1.2 Pseudo-code

```python
def adaptive_forward_pass(
    input_ids: Tensor[batch, seq_len],
    layer_masks: Tensor[num_layers],           # bool or float
    width_factor: float,                       # 0.5 to 2.0
    expert_masks: Tensor[num_experts],        # bool
    active_lora_adapters: List[str],           # ["adapter-A", "adapter-B"]
    kv_cache_store: KVCacheStore,              # for generation
    generation_mode: bool = False,              # prefill vs decode
) -> Tensor[batch, seq_len, vocab_size]:
    """
    Core adaptive forward pass for Bonsai transformers.
    
    Args:
        input_ids: tokenized input
        layer_masks: which layers to activate (shape: [num_layers])
        width_factor: fraction of hidden_dim to use (0.5 = half width)
        expert_masks: which experts are available (shape: [num_experts])
        active_lora_adapters: list of adapter names to compose
        kv_cache_store: KVCache for generation
        generation_mode: True during decoding, False during prefill
    
    Returns:
        logits for next token prediction
    """
    
    # ========== EMBEDDING LAYER ==========
    x = embedding(input_ids)  # [batch, seq_len, hidden_dim]
    
    # Apply position encoding aware of width_factor
    pos_enc = position_encoding(
        seq_len=input_ids.shape[1],
        hidden_dim=int(hidden_dim * width_factor),
    )
    x = x[:, :, :pos_enc.shape[-1]] + pos_enc
    
    # Track which dimensions are active
    active_hidden_dim = int(hidden_dim * width_factor)
    
    # ========== LAYER STACK ==========
    for layer_idx in range(num_layers):
        x_residual = x.clone()
        
        # Layer masking: skip inactive layers
        if not layer_masks[layer_idx]:
            # Inactive layer: apply skip connection
            x = x_residual
            # Invalidate KV cache for this layer
            kv_cache_store.invalidate(layer_idx, generation_mode)
            continue
        
        # ===== ADAPTIVE ATTENTION =====
        x = adaptive_attention(
            x=x,
            layer_idx=layer_idx,
            width_factor=width_factor,
            active_hidden_dim=active_hidden_dim,
            kv_cache=kv_cache_store.get(layer_idx) if generation_mode else None,
        )
        
        # ===== ADAPTIVE FFN =====
        x = adaptive_ffn(
            x=x,
            layer_idx=layer_idx,
            width_factor=width_factor,
            active_hidden_dim=active_hidden_dim,
        )
        
        # ===== EXPERT ROUTING (if MoE layer) =====
        if is_moe_layer[layer_idx]:
            x = selective_moe(
                x=x,
                layer_idx=layer_idx,
                expert_masks=expert_masks,
                top_k=default_top_k,
            )
        
        # ===== LORA ADAPTER COMPOSITION =====
        for adapter_name in active_lora_adapters:
            adapter = adapter_registry[adapter_name]
            if adapter.applies_to_layer(layer_idx):
                delta = adapter.forward(x, layer_idx)
                x = x + delta
        
        # ===== RESIDUAL + LAYERNORM =====
        x = x + x_residual
        x = layer_norm(x)
    
    # ========== OUTPUT PROJECTION ==========
    # Map from active_hidden_dim back to full hidden_dim if scaled
    if width_factor != 1.0:
        x = output_projection(x, width_factor)
    
    # ========== LM HEAD ==========
    logits = lm_head(x)  # [batch, seq_len, vocab_size]
    
    return logits
```

### 1.3 Masking Strategy

We support **two types of masks**:

#### Binary Masks (Boolean)
```
layer_mask[i] ∈ {True, False}
- True: compute full layer
- False: skip layer (residual only)
```

#### Soft Masks (Float [0,1])
```
layer_mask[i] ∈ [0.0, 1.0]
- Scale layer output: output = layer(x) * mask[i] + x * (1 - mask[i])
- Enables gradient-based layer selection during training
```

For inference, we recommend **binary masks** (no extra computation).

---

## 2. Layer Mask Application

### 2.1 Skip Connection Design

When a layer is masked out (inactive), we must preserve the residual connection:

```
# INACTIVE LAYER (layer_mask[i] = False)
x_out = x_in  # Identity pass-through

# ACTIVE LAYER (layer_mask[i] = True)
x_out = LayerNorm(x_in + TransformerBlock(x_in))
```

**Important:** The LayerNorm is applied **only for active layers**. For inactive layers, we skip normalization to avoid accumulating normalization errors.

### 2.2 Handling Variable Layer Count in Generation

During generation, when using KV-cache:

```python
def generation_forward(
    token_id: int,
    layer_masks: Tensor[num_layers],
    kv_cache_store: KVCacheStore,
) -> int:
    """
    Decode a single token given the KV cache.
    
    KV cache shape: [num_layers, num_heads, cache_len, head_dim]
    """
    
    x = embedding(token_id)  # [1, 1, hidden_dim]
    
    # Position encoding for current position
    position = kv_cache_store.current_position()
    pos_enc = position_encoding(position, hidden_dim)
    x = x + pos_enc
    
    for layer_idx in range(num_layers):
        if not layer_masks[layer_idx]:
            # No KV cache for this layer; just skip
            continue
        
        # Use cached KV from previous prefill
        kv = kv_cache_store.get(layer_idx)
        
        # Attention with cached KV
        x = attention(x, kv.K, kv.V, mask=None)
        
        # FFN
        x = ffn(x)
        
        # Residual + LayerNorm
        x = x + x_residual
        x = layer_norm(x)
        
        # Append new KV to cache
        kv_cache_store.append(layer_idx, new_K, new_V)
    
    return lm_head(x).argmax()
```

**Key Design Decision:** We track position globally across all layers. Position encoding is applied at embedding time, not per-layer.

### 2.3 Attention with Variable Layers

Standard attention: `Attention(Q, K, V) = softmax(Q @ K^T / sqrt(d_k)) @ V`

When we skip layers, the attention positions don't change—only the layer activations do.

```python
def adaptive_attention(
    x: Tensor[batch, seq_len, hidden_dim],
    layer_idx: int,
    width_factor: float,
    active_hidden_dim: int,
    kv_cache: Optional[KVCache] = None,
) -> Tensor[batch, seq_len, active_hidden_dim]:
    """
    Attention with dynamic width scaling.
    
    If width_factor = 0.5:
    - Use only first 50% of attention heads
    - Head dimension unchanged
    - Each head operates on full input, but we select subset of heads
    """
    
    # Adaptive head count
    num_heads = base_num_heads * width_factor
    head_dim = hidden_dim // base_num_heads
    
    # Project Q, K, V (but only compute active heads)
    Q = x @ W_q[:, :num_heads * head_dim]  # [batch, seq_len, num_heads * head_dim]
    K = x @ W_k[:, :num_heads * head_dim]
    V = x @ W_v[:, :num_heads * head_dim]
    
    # Reshape for multi-head attention
    Q = Q.reshape(batch, seq_len, num_heads, head_dim)
    K = K.reshape(batch, seq_len, num_heads, head_dim)
    V = V.reshape(batch, seq_len, num_heads, head_dim)
    
    # Cache KV if in generation mode
    if kv_cache is not None:
        # Append current K, V to cache
        K_cached = torch.cat([kv_cache.K, K], dim=2)  # [batch, num_heads, cache_len+1, head_dim]
        V_cached = torch.cat([kv_cache.V, V], dim=2)
    else:
        K_cached, V_cached = K, V
    
    # Compute attention
    scores = (Q @ K_cached.transpose(-2, -1)) / math.sqrt(head_dim)
    attn_weights = softmax(scores, dim=-1)
    output = attn_weights @ V_cached
    
    # Reshape and project
    output = output.reshape(batch, seq_len, num_heads * head_dim)
    output = output @ W_o[:num_heads * head_dim, :active_hidden_dim]
    
    return output
```

---

## 3. Width Scaling in Attention & FFN

### 3.1 Attention Head Scaling

```
Base model:
- hidden_dim = 4096
- num_heads = 32
- head_dim = 128

With width_factor = 2.0:
- active_hidden_dim = 8192 (!)
- active_num_heads = 64
- head_dim still 128

With width_factor = 0.5:
- active_hidden_dim = 2048
- active_num_heads = 16
- head_dim still 128
```

**Key insight:** We keep head_dim constant; we scale the number of heads.

### 3.2 Implementation Strategy

Instead of creating new weight matrices, we **select the first N columns** from the base weights:

```python
def project_with_width_factor(x, W, width_factor):
    """
    Project x using first N columns of W.
    
    W shape: [hidden_dim, hidden_dim]
    width_factor: float
    
    Returns: [batch, seq_len, int(hidden_dim * width_factor)]
    """
    active_dim = int(W.shape[1] * width_factor)
    
    # Only use first active_dim columns
    W_active = W[:, :active_dim]  # [hidden_dim, active_dim]
    
    output = x @ W_active  # [batch, seq_len, active_dim]
    return output
```

**Memory efficiency:** No copying of weights. We use views/indexing on the original W.

### 3.3 FFN with Width Scaling

```python
def adaptive_ffn(
    x: Tensor[batch, seq_len, hidden_dim],
    layer_idx: int,
    width_factor: float,
    active_hidden_dim: int,
) -> Tensor[batch, seq_len, active_hidden_dim]:
    """
    FFN with adaptive intermediate dimension.
    
    Base: hidden → ffn_dim → hidden (e.g., 4096 → 16384 → 4096)
    
    With width_factor = 0.5:
    hidden_dim_active → (ffn_dim * 0.5) → hidden_dim_active
    """
    
    ffn_dim_active = int(ffn_dim * width_factor)
    
    # First linear layer: hidden_dim → ffn_dim_active
    x_hidden = x @ W_up[:, :ffn_dim_active]  # [batch, seq_len, ffn_dim_active]
    
    # Activation (GELU, ReLU, etc.)
    x_hidden = gelu(x_hidden)
    
    # Second linear layer: ffn_dim_active → active_hidden_dim
    x_out = x_hidden @ W_down[:ffn_dim_active, :active_hidden_dim]  # [batch, seq_len, active_hidden_dim]
    
    return x_out
```

### 3.4 Width Projection Back to Full Dimension

If we want to return to full hidden dimension:

```python
def output_projection(x, width_factor):
    """
    Project from active_hidden_dim back to full hidden_dim.
    
    Options:
    1. Zero-pad: x → [x, zeros(...)]
    2. Learned projection: x @ W_proj
    3. Interpolation (for positional embeddings)
    """
    
    active_dim = x.shape[-1]
    full_dim = int(active_dim / width_factor)
    
    if width_factor >= 1.0:
        # Expanding (rare in inference)
        # Learn a projection matrix
        return x @ W_expand  # [batch, seq_len, full_dim]
    else:
        # Shrinking: zero-pad for efficiency
        padding = full_dim - active_dim
        return torch.nn.functional.pad(x, (0, padding))
```

---

## 4. Expert Routing with Expert Masks

### 4.1 Mixture-of-Experts Basics

Standard MoE layer:

```
Router computes scores: logits = x @ W_router  # [batch, seq_len, num_experts]
Top-K selection: top_k_experts = top_k(logits, k=num_selected_experts)
Gating: gate_values = softmax(logits[top_k_experts])
Output: result = sum_i gate_values[i] * expert[top_k_experts[i]](x)
```

### 4.2 Expert Masking

We add a gating mechanism to **disable experts dynamically**:

```python
def selective_moe(
    x: Tensor[batch, seq_len, hidden_dim],
    layer_idx: int,
    expert_masks: Tensor[num_experts],      # bool or float [0,1]
    top_k: int = 2,
) -> Tensor[batch, seq_len, hidden_dim]:
    """
    Mixture-of-Experts with expert masking.
    
    Args:
        expert_masks: which experts to enable (bool) or their scaling (float)
                      shape: [num_experts]
    """
    
    # Router decides which experts to use
    router_logits = x @ W_router  # [batch, seq_len, num_experts]
    
    # Apply expert mask: set disabled expert logits to -inf
    # This prevents them from being selected by top_k
    MASK_VALUE = -1e9
    masked_logits = torch.where(
        expert_masks.unsqueeze(0).unsqueeze(0),  # broadcast to [1, 1, num_experts]
        router_logits,
        torch.full_like(router_logits, MASK_VALUE)
    )
    
    # Select top-K from enabled experts only
    top_k_values, top_k_indices = torch.topk(masked_logits, k=top_k, dim=-1)
    # top_k_values: [batch, seq_len, top_k]
    # top_k_indices: [batch, seq_len, top_k]
    
    # Gating weights
    gate_weights = softmax(top_k_values, dim=-1)  # [batch, seq_len, top_k]
    
    # Route to selected experts
    output = torch.zeros_like(x)
    for i in range(top_k):
        expert_idx = top_k_indices[:, :, i]  # [batch, seq_len]
        weight = gate_weights[:, :, i].unsqueeze(-1)  # [batch, seq_len, 1]
        
        # Apply expert (sparse, only for selected indices)
        expert_output = apply_expert(x, expert_idx, layer_idx)
        output = output + weight * expert_output
    
    return output
```

### 4.3 Load Balancing with Masked Experts

**Challenge:** If many experts are masked, some sequences might have no valid experts (top_k > num_enabled_experts).

**Solution:** Adaptive top_k:

```python
def selective_moe_with_adaptive_topk(
    x: Tensor[batch, seq_len, hidden_dim],
    expert_masks: Tensor[num_experts],
    min_top_k: int = 1,
    default_top_k: int = 2,
) -> Tensor:
    """
    Select top-K experts from enabled experts.
    If fewer than default_top_k enabled, use what's available.
    """
    
    num_enabled_experts = expert_masks.sum().item()
    effective_top_k = min(default_top_k, num_enabled_experts)
    effective_top_k = max(effective_top_k, min_top_k)
    
    router_logits = x @ W_router
    
    # Mask disabled experts
    masked_logits = torch.where(
        expert_masks.unsqueeze(0).unsqueeze(0),
        router_logits,
        torch.full_like(router_logits, -1e9)
    )
    
    # Select top-K from enabled
    top_k_values, top_k_indices = torch.topk(masked_logits, k=effective_top_k, dim=-1)
    gate_weights = softmax(top_k_values, dim=-1)
    
    # Apply experts...
    return output
```

### 4.4 Expert Capacity

In distributed MoE, each expert has a fixed capacity per batch:

```
Expert capacity = (batch_size * seq_len * top_k) / num_experts * capacity_factor
```

With expert masking, capacity **per enabled expert** changes:

```python
def compute_expert_capacity(
    batch_size: int,
    seq_len: int,
    top_k: int,
    num_experts_total: int,
    expert_masks: Tensor[num_experts_total],
    capacity_factor: float = 1.25,
) -> int:
    """
    Compute expert capacity accounting for masking.
    
    If only 50% of experts are enabled:
    - Tokens redistribute across fewer experts
    - Per-expert capacity increases
    """
    
    num_enabled_experts = expert_masks.sum().item()
    total_tokens = batch_size * seq_len * top_k
    capacity_per_expert = (total_tokens / num_enabled_experts) * capacity_factor
    
    return int(capacity_per_expert)
```

---

## 5. LoRA Adapter Composition

### 5.1 LoRA Basics

LoRA (Low-Rank Adaptation) adds a learnable low-rank update to weights:

```
W_effective = W_base + B @ A^T

Where:
- W_base: original weight matrix (frozen)
- A: [hidden_dim, rank] — trainable
- B: [hidden_dim, rank] — trainable
- rank ≪ hidden_dim (typically 8-64)
```

### 5.2 Stacking Multiple Adapters

For multiple adapters, we sum the deltas:

```python
def forward_with_lora_stack(
    x: Tensor[batch, seq_len, hidden_dim],
    adapters: List[LoRAAdapter],
    layer_idx: int,
) -> Tensor:
    """
    Apply stacked LoRA adapters.
    
    W_eff = W_base + sum(B_i @ A_i^T for i in adapters)
    output = x @ W_eff^T
    """
    
    # Compute sum of deltas
    delta_W = torch.zeros_like(W_base)
    for adapter in adapters:
        if adapter.applies_to_layer(layer_idx):
            A = adapter.A[layer_idx]  # [hidden_dim, rank]
            B = adapter.B[layer_idx]  # [hidden_dim, rank]
            delta_W = delta_W + (B @ A.T)
    
    # Apply to activations
    W_eff = W_base + delta_W
    output = x @ W_eff.T
    
    return output
```

### 5.3 Efficient LoRA Application

Instead of computing `delta_W` explicitly (expensive), compute `x @ (B @ A^T)^T = (x @ A) @ B^T`:

```python
def forward_with_lora_efficient(
    x: Tensor[batch, seq_len, hidden_dim],
    adapters: List[LoRAAdapter],
    layer_idx: int,
    apply_lora_after_residual: bool = True,
) -> Tensor:
    """
    Efficient LoRA composition: avoid materializing full delta_W.
    
    Computation:
    1. Base forward: x @ W_base
    2. For each adapter: x @ A @ B^T (low-rank matmul)
    """
    
    # Base forward pass
    output = x @ W_base.T  # [batch, seq_len, hidden_dim]
    
    # Add LoRA deltas
    for adapter in adapters:
        if adapter.applies_to_layer(layer_idx):
            A = adapter.A[layer_idx]  # [hidden_dim, rank]
            B = adapter.B[layer_idx]  # [hidden_dim, rank]
            
            # Compute low-rank update: (x @ A) @ B^T
            x_proj = x @ A  # [batch, seq_len, rank]
            lora_delta = x_proj @ B.T  # [batch, seq_len, hidden_dim]
            
            output = output + lora_delta
    
    return output
```

**Complexity:** O(batch * seq_len * hidden_dim * rank) per adapter (vs. O(batch * seq_len * hidden_dim^2) for full update).

### 5.4 LoRA with Width Scaling

When width_factor < 1.0, adapters only apply to active dimensions:

```python
def forward_with_lora_and_width_scaling(
    x: Tensor[batch, seq_len, active_hidden_dim],
    adapters: List[LoRAAdapter],
    layer_idx: int,
    width_factor: float,
) -> Tensor:
    """
    Apply LoRA only to active dimensions.
    
    If width_factor = 0.5:
    - x shape: [batch, seq_len, hidden_dim/2]
    - A shape: [hidden_dim/2, rank]  (only active rows)
    - B shape: [hidden_dim/2, rank]
    """
    
    active_dim = x.shape[-1]
    
    output = x  # Start from input (we'll add deltas in-place)
    
    for adapter in adapters:
        if adapter.applies_to_layer(layer_idx):
            # Select only active rows from A and B
            A_active = adapter.A[layer_idx][:active_dim, :]  # [active_dim, rank]
            B_active = adapter.B[layer_idx][:active_dim, :]  # [active_dim, rank]
            
            # Efficient LoRA: (x @ A_active) @ B_active^T
            x_proj = x @ A_active  # [batch, seq_len, rank]
            lora_delta = x_proj @ B_active.T  # [batch, seq_len, active_dim]
            
            output = output + lora_delta
    
    return output
```

### 5.5 Inactive Adapter Handling

If an adapter is disabled (not in `active_lora_adapters`), skip it entirely:

```python
def forward_with_selective_lora(
    x: Tensor,
    all_adapters: Dict[str, LoRAAdapter],
    active_adapter_names: List[str],
    layer_idx: int,
) -> Tensor:
    """
    Only compute deltas for active adapters.
    """
    
    output = x
    
    for adapter_name in active_adapter_names:
        adapter = all_adapters[adapter_name]
        if adapter.applies_to_layer(layer_idx):
            A = adapter.A[layer_idx]
            B = adapter.B[layer_idx]
            
            x_proj = x @ A
            delta = x_proj @ B.T
            output = output + delta
    
    return output
```

---

## 6. Positional Encoding Handling

### 6.1 The Challenge

Standard positional encodings (RoPE, ALiBi, absolute PE):

```
PE(pos, dim) = sin(pos / 10000^(dim/d_model))  for absolute PE
```

When we change layer masks or width_factor, the **position indices stay the same** (absolute positioning), but the **dimension might change** (width scaling).

### 6.2 Solution: Dimension-Aware PE

**Approach 1: Zero-pad/truncate**

```python
def position_encoding_with_width_factor(
    seq_len: int,
    hidden_dim: int,
    width_factor: float,
    pos_start: int = 0,  # For generation (in KV cache)
) -> Tensor[seq_len, int(hidden_dim * width_factor)]:
    """
    Generate position encoding for active dimensions only.
    """
    
    active_dim = int(hidden_dim * width_factor)
    
    # Standard position encoding (e.g., RoPE)
    pe = compute_rope_or_absolute_pe(
        seq_len=seq_len,
        dim=active_dim,  # Only compute for active_dim
        pos_start=pos_start,
    )
    
    return pe  # [seq_len, active_dim]
```

**Approach 2: Interpolation (if scaling > 1.0)**

If width_factor > 1.0, we're expanding dimensions. We can interpolate:

```python
def interpolate_pe(
    base_pe: Tensor[seq_len, base_dim],
    target_dim: int,
) -> Tensor[seq_len, target_dim]:
    """
    Interpolate position encoding to larger dimension.
    
    Options:
    1. Repeat: duplicate entries
    2. Linear interpolation: blend nearby entries
    3. Fourier interpolation: use sine/cosine at intermediate frequencies
    """
    
    if target_dim <= base_dim:
        # Truncate
        return base_pe[:, :target_dim]
    
    # Expand via linear interpolation
    scale = target_dim / base_dim
    new_pe = torch.zeros(base_pe.shape[0], target_dim)
    
    for i in range(target_dim):
        src_i = i / scale
        src_floor = int(src_i)
        src_ceil = min(src_floor + 1, base_dim - 1)
        alpha = src_i - src_floor
        
        if src_floor < base_dim:
            new_pe[:, i] = (1 - alpha) * base_pe[:, src_floor] + alpha * base_pe[:, src_ceil]
    
    return new_pe
```

### 6.3 RoPE-Specific Design

RoPE (Rotary Position Embedding) is position-aware and dimension-aware:

```python
def rope_with_width_factor(
    Q: Tensor[batch, num_heads, seq_len, head_dim],
    K: Tensor[batch, num_heads, seq_len, head_dim],
    width_factor: float,
) -> Tuple[Tensor, Tensor]:
    """
    Apply RoPE using only active head_dim dimensions.
    
    head_dim typically = 128, so:
    - width_factor = 0.5: use first 64 dims
    - width_factor = 2.0: use first 128 dims (standard), but upscaled freq
    """
    
    active_head_dim = int(head_dim * width_factor)
    
    # Precompute freq
    theta = 10000.0 ** (-2.0 * (torch.arange(0, active_head_dim, 2) / active_head_dim))
    m = torch.arange(seq_len)
    freqs = torch.outer(m, theta)  # [seq_len, active_head_dim/2]
    
    # Complex number representation for rotation
    emb = torch.cat([freqs, freqs], dim=-1)  # [seq_len, active_head_dim]
    cos_emb = torch.cos(emb)
    sin_emb = torch.sin(emb)
    
    # Apply rotation
    def rotate_half(x):
        """Rotate by 90 degrees in complex plane."""
        return torch.cat(
            [-x[..., active_head_dim//2:], x[..., :active_head_dim//2]], dim=-1
        )
    
    Q_rope = Q[..., :active_head_dim] * cos_emb + rotate_half(Q[..., :active_head_dim]) * sin_emb
    K_rope = K[..., :active_head_dim] * cos_emb + rotate_half(K[..., :active_head_dim]) * sin_emb
    
    return Q_rope, K_rope
```

### 6.4 ALiBi Alternative

ALiBi (Attention with Linear Biases) doesn't need dimension-specific encoding:

```python
def alibi_with_layer_masking(
    seq_len: int,
    num_heads: int,
    width_factor: float,
) -> Tensor[num_heads, seq_len, seq_len]:
    """
    ALiBi: add bias based on relative position.
    
    ALiBi doesn't depend on hidden_dim, so width_factor has no effect.
    """
    
    active_num_heads = int(num_heads * width_factor)
    
    slopes = torch.arange(1, active_num_heads + 1)
    slopes = 2.0 ** (-torch.log2(torch.tensor(active_num_heads + 1.0)) * slopes)
    
    # Relative distance matrix
    m = torch.arange(seq_len).unsqueeze(1) - torch.arange(seq_len).unsqueeze(0)
    bias = m.unsqueeze(0) * slopes.unsqueeze(-1).unsqueeze(-1)  # [active_num_heads, seq_len, seq_len]
    
    return bias
```

**Recommendation:** Use ALiBi for simplicity if possible. It decouples positional information from width scaling.

---

## 7. KV-Cache Management

### 7.1 Cache Structure

```python
@dataclass
class KVCache:
    """Per-layer KV cache for generation."""
    K: Tensor  # [batch, num_heads, seq_len, head_dim]
    V: Tensor
    seq_len: int  # Current cache length
    layer_idx: int
    active_mask: bool  # Is this layer active?
```

### 7.2 Cache Invalidation on Mask Change

If we switch layer_masks mid-generation, some caches become invalid:

```python
class KVCacheStore:
    def __init__(self, num_layers, batch_size, max_seq_len, head_dim):
        self.caches = [
            KVCache(
                K=torch.zeros(batch_size, num_heads, max_seq_len, head_dim),
                V=torch.zeros_like(K),
                seq_len=0,
                layer_idx=i,
                active_mask=True,
            )
            for i in range(num_layers)
        ]
        self.current_seq_len = 0
    
    def invalidate_layer(self, layer_idx: int):
        """Clear cache for a specific layer."""
        cache = self.caches[layer_idx]
        cache.K.zero_()
        cache.V.zero_()
        cache.seq_len = 0
    
    def invalidate_on_mask_change(self, old_masks, new_masks):
        """Invalidate cache for layers that changed state."""
        for i, (old, new) in enumerate(zip(old_masks, new_masks)):
            if old != new:
                self.invalidate_layer(i)
    
    def append_kv(self, layer_idx: int, K_new, V_new):
        """Append new K, V to cache (during generation)."""
        cache = self.caches[layer_idx]
        cache.K[:, :, cache.seq_len, :] = K_new.squeeze(2)  # [batch, num_heads, 1, head_dim] → [batch, num_heads, head_dim]
        cache.V[:, :, cache.seq_len, :] = V_new.squeeze(2)
        cache.seq_len += 1
    
    def get_kv(self, layer_idx: int) -> Tuple[Tensor, Tensor]:
        """Get full K, V for attention (for decoding)."""
        cache = self.caches[layer_idx]
        K = cache.K[:, :, :cache.seq_len, :]  # Trim to actual length
        V = cache.V[:, :, :cache.seq_len, :]
        return K, V
```

### 7.3 Cache Size Calculation

```python
def compute_kv_cache_size_bytes(
    num_layers: int,
    batch_size: int,
    max_seq_len: int,
    num_heads: int,
    head_dim: int,
    dtype: torch.dtype = torch.float16,
) -> float:
    """
    Total memory for KV cache.
    """
    bytes_per_element = torch.tensor(0, dtype=dtype).itemsize
    
    # K and V per layer
    cache_per_layer = 2 * batch_size * num_heads * max_seq_len * head_dim * bytes_per_element
    
    total = num_layers * cache_per_layer
    
    return total / (1024 ** 3)  # GB
```

Example:
- 32 layers, batch 1, max_seq_len 2048
- 32 heads, 128 head_dim
- float16: ~32 * 2 * 1 * 32 * 2048 * 128 * 2 bytes ≈ **1.6 GB**

### 7.4 Efficient Cache Update

During generation, we only compute **one new token** per step:

```python
def generation_step(
    token_id: int,
    kv_cache_store: KVCacheStore,
    layer_masks: Tensor[num_layers],
) -> int:
    """
    Decode one token, updating KV cache.
    """
    
    x = embedding(token_id)  # [1, 1, hidden_dim]
    
    for layer_idx in range(num_layers):
        if not layer_masks[layer_idx]:
            continue
        
        # Retrieve cached K, V
        K_cached, V_cached = kv_cache_store.get_kv(layer_idx)
        
        # Compute attention for current position only
        Q = x @ W_q
        K_new = x @ W_k
        V_new = x @ W_v
        
        # Attention: Q with all previous K, V
        scores = Q @ K_cached.transpose(-2, -1) / sqrt(head_dim)
        attn_weights = softmax(scores, dim=-1)
        x = attn_weights @ V_cached  # [1, 1, hidden_dim]
        
        # Append new K, V to cache
        kv_cache_store.append_kv(layer_idx, K_new, V_new)
        
        # FFN + residual...
        x = x + residual
        x = layer_norm(x)
    
    # Return next token
    logits = lm_head(x)
    return argmax(logits)
```

---

## 8. Batch Processing with Mixed Scales

### 8.1 The Challenge

In a single batch, requests may have different configurations:
- Request A: layer_mask = [1, 1, 0, 1, ...], width_factor = 1.0
- Request B: layer_mask = [1, 1, 1, 1, ...], width_factor = 0.5

Can we process them together?

### 8.2 Strategy 1: Separate Batches (Simplest)

```python
def inference_with_mixed_scales(requests):
    """
    Group requests by scale config, process separately.
    """
    
    groups = defaultdict(list)
    for req in requests:
        key = (tuple(req.layer_masks), req.width_factor)
        groups[key].append(req)
    
    results = []
    for (masks, width_factor), group in groups.items():
        # Batch process this group
        batch_logits = forward_pass(group, masks, width_factor)
        results.extend(batch_logits)
    
    return results
```

**Pros:** Simple, no overhead  
**Cons:** Loses batching benefits if configs are diverse

### 8.2 Strategy 2: Unified Batch with Masking (Advanced)

```python
def inference_unified_batch(requests):
    """
    Process all requests in one batch, using per-request masking.
    
    Requires:
    - Per-sequence layer masks
    - Conditional computation (torch.where, scatter operations)
    """
    
    batch_size = len(requests)
    layer_masks = torch.stack([r.layer_masks for r in requests])  # [batch, num_layers]
    width_factors = torch.tensor([r.width_factor for r in requests])  # [batch]
    
    x = embedding(all_input_ids)  # [batch, seq_len, hidden_dim]
    
    for layer_idx in range(num_layers):
        x_residual = x.clone()
        
        # Per-sequence layer masking
        active_mask = layer_masks[:, layer_idx]  # [batch]
        
        if active_mask.all():
            # All sequences active: normal forward
            x = transformer_layer(x, layer_idx)
        elif not active_mask.any():
            # All sequences inactive: skip
            x = x_residual
        else:
            # Mixed: some active, some inactive
            x_transformed = transformer_layer(x, layer_idx)
            x = torch.where(
                active_mask.unsqueeze(-1).unsqueeze(-1),  # broadcast to [batch, 1, 1]
                x_transformed,
                x_residual
            )
        
        x = x + x_residual
        x = layer_norm(x)
    
    return lm_head(x)
```

**Pros:** Better batching utilization  
**Cons:** More complex, overhead from torch.where operations

### 8.3 Recommendation

**For production:** Use Strategy 1 (separate batches). Easier to implement, debug, and optimize. Batching gain is usually outweighed by simplicity.

---

## 9. Gradient Computation (Training)

### 9.1 Training Forward Pass

During fine-tuning, we need gradients for **only active layers**:

```python
def training_forward(
    input_ids: Tensor[batch, seq_len],
    layer_masks: Tensor[num_layers],
    labels: Tensor[batch, seq_len],
) -> Tensor:
    """
    Forward pass during fine-tuning.
    
    Compute loss only on active layers.
    """
    
    x = embedding(input_ids)
    
    for layer_idx in range(num_layers):
        if not layer_masks[layer_idx]:
            # Inactive: no computation, no gradient
            continue
        
        # Normal forward for active layer
        x = transformer_layer(x, layer_idx)
        x = x + residual
        x = layer_norm(x)
    
    logits = lm_head(x)
    
    # Loss only on active layers
    loss = cross_entropy(logits, labels)
    
    return loss
```

### 9.2 Gradient Flow

When we backpropagate:

```
∂loss/∂W_base (for active layer) ← gradient
∂loss/∂W_base (for inactive layer) = 0  ← no gradient
```

This is naturally handled by PyTorch: if a parameter isn't used in the forward pass, its gradient is None.

### 9.3 LoRA Gradient Computation

For LoRA adapters, we only need gradients for A and B:

```python
def training_forward_with_lora(x, layer_idx, adapters):
    """
    Forward pass with LoRA (only A, B get gradients).
    """
    
    # Base forward (no gradient for W_base!)
    with torch.no_grad():
        output = x @ W_base.T
    
    # LoRA forward (gradients for A, B)
    for adapter in adapters:
        A = adapter.A[layer_idx]  # requires_grad=True
        B = adapter.B[layer_idx]  # requires_grad=True
        
        x_proj = x @ A
        delta = x_proj @ B.T
        output = output + delta
    
    return output
```

### 9.4 Mixed Precision Training

With adaptive scales, we must be careful with mixed precision:

```python
def training_forward_mixed_precision(x, layer_idx, layer_masks):
    """
    Forward pass with mixed precision (fp32 for loss, fp16 for compute).
    
    For inactive layers, we skip entirely (no computation).
    """
    
    with torch.cuda.amp.autocast():
        for i in range(num_layers):
            if not layer_masks[i]:
                continue
            
            x = transformer_layer(x, i)  # computed in fp16
            x = layer_norm(x)
    
    # Loss computation in fp32 (automatic by autocast)
    logits = lm_head(x)
    loss = cross_entropy(logits, labels)
    
    return loss
```

---

## 10. Efficient Implementation in PyTorch

### 10.1 PyTorch Implementation Sketch

```python
import torch
import torch.nn.functional as F
from typing import Optional, List, Tuple

class AdaptiveTransformer(torch.nn.Module):
    """
    Full adaptive transformer implementation.
    """
    
    def __init__(
        self,
        hidden_dim: int = 4096,
        num_layers: int = 32,
        num_heads: int = 32,
        ffn_dim: int = 16384,
        num_experts: int = 1024,
        vocab_size: int = 128256,
    ):
        super().__init__()
        
        self.hidden_dim = hidden_dim
        self.num_layers = num_layers
        self.num_heads = num_heads
        self.ffn_dim = ffn_dim
        self.num_experts = num_experts
        
        # Embeddings
        self.token_embedding = torch.nn.Embedding(vocab_size, hidden_dim)
        self.pos_embedding = RotaryEmbedding(hidden_dim // num_heads)
        
        # Transformer layers
        self.layers = torch.nn.ModuleList([
            TransformerLayer(hidden_dim, num_heads, ffn_dim)
            for _ in range(num_layers)
        ])
        
        # MoE layers (sparse, only on certain layers)
        self.moe_layers = {4, 8, 12, 16, 20, 24, 28}  # which layers have MoE
        self.experts = torch.nn.ModuleList([
            Expert(hidden_dim, ffn_dim)
            for _ in range(num_experts)
        ])
        self.router = torch.nn.Linear(hidden_dim, num_experts)
        
        # LoRA adapter registry
        self.adapter_registry = {}
        
        # LM head
        self.lm_head = torch.nn.Linear(hidden_dim, vocab_size)
    
    def forward(
        self,
        input_ids: torch.Tensor,
        layer_masks: Optional[torch.Tensor] = None,
        width_factor: float = 1.0,
        expert_masks: Optional[torch.Tensor] = None,
        active_adapters: Optional[List[str]] = None,
        kv_cache: Optional[List[Tuple[torch.Tensor, torch.Tensor]]] = None,
    ) -> torch.Tensor:
        """
        Adaptive forward pass.
        
        Args:
            input_ids: [batch, seq_len]
            layer_masks: [num_layers], bool or float
            width_factor: float in [0.1, 2.0]
            expert_masks: [num_experts], bool
            active_adapters: list of adapter names
            kv_cache: KV cache for generation
        
        Returns:
            logits [batch, seq_len, vocab_size]
        """
        
        if layer_masks is None:
            layer_masks = torch.ones(self.num_layers, device=input_ids.device, dtype=torch.bool)
        if expert_masks is None:
            expert_masks = torch.ones(self.num_experts, device=input_ids.device, dtype=torch.bool)
        if active_adapters is None:
            active_adapters = []
        
        # Embedding
        x = self.token_embedding(input_ids)  # [batch, seq_len, hidden_dim]
        
        # Active hidden dimension
        active_hidden_dim = int(self.hidden_dim * width_factor)
        
        # Position encoding (for active dims only)
        pos_enc = self.pos_embedding(x.shape[1], active_hidden_dim)
        x = x[:, :, :active_hidden_dim] + pos_enc.unsqueeze(0)
        
        # Layer stack
        for layer_idx in range(self.num_layers):
            x_residual = x.clone()
            
            # Layer masking
            if isinstance(layer_masks, torch.Tensor) and not layer_masks[layer_idx]:
                x = x_residual
                continue
            
            # Attention
            x = self.layers[layer_idx].attention(
                x, width_factor=width_factor,
                kv_cache=kv_cache[layer_idx] if kv_cache else None
            )
            
            # MoE routing (if this is a MoE layer)
            if layer_idx in self.moe_layers:
                x = self._selective_moe(
                    x, expert_masks=expert_masks
                )
            else:
                # FFN
                x = self.layers[layer_idx].ffn(x, width_factor=width_factor)
            
            # LoRA adapters
            for adapter_name in active_adapters:
                adapter = self.adapter_registry.get(adapter_name)
                if adapter and adapter.applies_to_layer(layer_idx):
                    x = adapter.forward(x, layer_idx, active_hidden_dim)
            
            # Residual + LayerNorm
            x = x + x_residual
            x = self.layers[layer_idx].norm(x)
        
        # Output projection (if width_factor != 1.0)
        if width_factor != 1.0 and active_hidden_dim != self.hidden_dim:
            x = F.pad(x, (0, self.hidden_dim - active_hidden_dim))
        
        # LM head
        logits = self.lm_head(x)  # [batch, seq_len, vocab_size]
        
        return logits
    
    def _selective_moe(
        self,
        x: torch.Tensor,
        expert_masks: torch.Tensor,
        top_k: int = 2,
    ) -> torch.Tensor:
        """Expert routing with masking."""
        
        router_logits = self.router(x)  # [batch, seq_len, num_experts]
        
        # Mask disabled experts
        mask_value = -1e9
        masked_logits = torch.where(
            expert_masks.unsqueeze(0).unsqueeze(0),
            router_logits,
            torch.full_like(router_logits, mask_value)
        )
        
        # Select top-K
        top_k_values, top_k_indices = torch.topk(masked_logits, k=top_k, dim=-1)
        gate_weights = F.softmax(top_k_values, dim=-1)
        
        # Route to experts
        output = torch.zeros_like(x)
        for i in range(top_k):
            expert_idx = top_k_indices[:, :, i]
            weight = gate_weights[:, :, i].unsqueeze(-1)
            
            # Apply expert (inefficient; use scatter for production)
            expert_output = self.experts[expert_idx[0, 0]](x)
            output = output + weight * expert_output
        
        return output
```

### 10.2 Avoiding Conditional Branches

CUDA kernels don't handle conditionals well. Use vectorized operations:

```python
# Bad: conditional in a loop
for i in range(num_layers):
    if layer_mask[i]:
        x = layer(x)
    # Slow on GPU due to branch divergence

# Good: vectorized masking
x = layer(x)
x = torch.where(layer_mask, x, x_residual)
```

### 10.3 Custom CUDA Kernels (for Extreme Optimization)

For high throughput, implement custom kernels for:
- Adaptive attention (selective head computation)
- Adaptive FFN (selective neurons)
- Expert routing (scatter/gather)

Example (pseudo-CUDA):

```cuda
// Adaptive Attention Kernel
__global__ void adaptive_attention_kernel(
    const float* Q, const float* K, const float* V,
    int num_heads, int active_num_heads, int seq_len, int head_dim,
    float* output
) {
    int head_idx = blockIdx.x;
    
    // Only compute for active heads
    if (head_idx >= active_num_heads) return;
    
    // Standard attention for this head...
}
```

---

## 11. Verification & Testing

### 11.1 Correctness Tests

```python
import unittest
import torch

class TestAdaptiveTransformer(unittest.TestCase):
    
    def setUp(self):
        self.model = AdaptiveTransformer(hidden_dim=512, num_layers=4)
        self.batch_size = 2
        self.seq_len = 16
        self.input_ids = torch.randint(0, 1000, (self.batch_size, self.seq_len))
    
    def test_all_layers_active(self):
        """Verify that all_active == identity."""
        layer_masks = torch.ones(4, dtype=torch.bool)
        logits = self.model(self.input_ids, layer_masks=layer_masks)
        self.assertEqual(logits.shape, (self.batch_size, self.seq_len, self.model.num_vocab))
    
    def test_all_layers_inactive(self):
        """Verify that all_inactive == embedding passthrough."""
        layer_masks = torch.zeros(4, dtype=torch.bool)
        logits = self.model(self.input_ids, layer_masks=layer_masks)
        # Output should still be valid logits
        self.assertEqual(logits.shape, (self.batch_size, self.seq_len, self.model.num_vocab))
    
    def test_width_factor_consistency(self):
        """Verify that width_factor changes don't break forward pass."""
        for width_factor in [0.25, 0.5, 1.0, 2.0]:
            logits = self.model(
                self.input_ids,
                layer_masks=torch.ones(4, dtype=torch.bool),
                width_factor=width_factor
            )
            self.assertEqual(logits.shape, (self.batch_size, self.seq_len, self.model.num_vocab))
    
    def test_expert_masking(self):
        """Verify expert masking doesn't crash."""
        expert_masks = torch.ones(1024, dtype=torch.bool)
        expert_masks[500:] = False  # Disable half
        logits = self.model(self.input_ids, expert_masks=expert_masks)
        self.assertEqual(logits.shape, (self.batch_size, self.seq_len, self.model.num_vocab))
    
    def test_lora_composition(self):
        """Verify LoRA adapters stack correctly."""
        # Load two adapters
        self.model.register_adapter("adapter-a", LoRAAdapter(...))
        self.model.register_adapter("adapter-b", LoRAAdapter(...))
        
        logits = self.model(
            self.input_ids,
            active_adapters=["adapter-a", "adapter-b"]
        )
        self.assertEqual(logits.shape, (self.batch_size, self.seq_len, self.model.num_vocab))
    
    def test_numerical_stability_extreme_masks(self):
        """Test numerical stability with only 4 layers active."""
        layer_masks = torch.tensor([True, False, False, True])
        
        with torch.no_grad():
            logits = self.model(self.input_ids, layer_masks=layer_masks)
        
        # Check for NaN/Inf
        self.assertFalse(torch.isnan(logits).any())
        self.assertFalse(torch.isinf(logits).any())
        
        # Check magnitudes are reasonable
        self.assertTrue(logits.abs().max() < 1000)
```

### 11.2 Performance Benchmarks

```python
def benchmark_adaptive_forward():
    """Measure overhead of adaptive forward pass."""
    
    model = AdaptiveTransformer(hidden_dim=4096, num_layers=32).cuda()
    input_ids = torch.randint(0, 128000, (8, 512)).cuda()
    
    import time
    
    # Baseline: all layers active, width_factor=1.0
    start = time.time()
    for _ in range(100):
        _ = model(input_ids)
    baseline = time.time() - start
    
    # Adaptive: 50% layers, width_factor=0.5
    layer_masks = torch.bernoulli(torch.ones(32) * 0.5).bool()
    start = time.time()
    for _ in range(100):
        _ = model(input_ids, layer_masks=layer_masks, width_factor=0.5)
    adaptive = time.time() - start
    
    overhead = (adaptive / baseline - 1) * 100
    print(f"Baseline: {baseline:.2f}s, Adaptive: {adaptive:.2f}s, Overhead: {overhead:.1f}%")
    
    assert overhead < 10, f"Overhead too high: {overhead:.1f}%"
```

### 11.3 Layer-Wise Validation

```python
def validate_layer_masks():
    """Ensure layer masks are applied correctly."""
    
    model = AdaptiveTransformer(hidden_dim=512, num_layers=4)
    model.eval()
    
    input_ids = torch.tensor([[1, 2, 3]])
    
    # Trace layer activations
    activations_all = []
    activations_partial = []
    
    def hook_all(module, input, output):
        activations_all.append(output.clone().detach())
    
    def hook_partial(module, input, output):
        activations_partial.append(output.clone().detach())
    
    # Register hooks
    for layer in model.layers:
        layer.register_forward_hook(hook_all)
    
    with torch.no_grad():
        # All active
        _ = model(input_ids, layer_masks=torch.ones(4, dtype=torch.bool))
        activations_all = activations_all[:]
        activations_all.clear()
        
        # Partial active
        layer_masks = torch.tensor([True, False, True, False])
        _ = model(input_ids, layer_masks=layer_masks)
        activations_partial = activations_partial[:]
    
    # Layer 0 and 2 should have different activations
    # Layer 1 and 3 should be identical (skipped)
    print("Layer 0 changes:", not torch.allclose(activations_all[0], activations_partial[0]))
    print("Layer 1 skip:", torch.allclose(activations_all[1], activations_partial[1]))
```

---

## 12. Integration with llama.cpp / Inference Engine

### 12.1 Architecture Overview

```
Bonsai Adaptive Layer
    ↓
    ├─ Layer Mask Router (C/C++)
    ├─ Width Scaler (C/C++)
    ├─ Expert Router (C/C++)
    └─ LoRA Composer (C/C++)
    ↓
llama.cpp Core
    ├─ Embeddings
    ├─ Transformer Blocks
    ├─ MoE Routing
    └─ KV Cache Management
    ↓
GPU/CPU Backend
    ├─ CUDA kernels
    ├─ Metal (iOS)
    └─ CPU (x86/ARM)
```

### 12.2 FFI Layer (Rust → C)

```rust
// File: crates/bonsai-inference/src/adaptive_forward.rs

use std::ffi::{c_float, c_int};

#[repr(C)]
pub struct AdaptiveConfig {
    pub num_layers: c_int,
    pub num_experts: c_int,
    pub width_factor: c_float,
    pub layer_masks_ptr: *const u8,  // bool array
    pub expert_masks_ptr: *const u8,
}

#[repr(C)]
pub struct AdaptiveForwardInput {
    pub x_ptr: *const c_float,
    pub x_shape: [c_int; 3],  // [batch, seq_len, hidden_dim]
    pub config: AdaptiveConfig,
}

#[repr(C)]
pub struct AdaptiveForwardOutput {
    pub output_ptr: *mut c_float,
    pub output_shape: [c_int; 3],  // [batch, seq_len, vocab_size]
}

extern "C" {
    pub fn adaptive_forward_pass(
        input: *const AdaptiveForwardInput,
        output: *mut AdaptiveForwardOutput,
    ) -> c_int;  // 0 = success, non-zero = error
}
```

### 12.3 llama.cpp Extension

```cpp
// File: src/adaptive.cpp
#include "llama.h"
#include <vector>

struct adaptive_config {
    std::vector<bool> layer_masks;
    std::vector<bool> expert_masks;
    float width_factor;
};

void llama_forward_adaptive(
    llama_context * ctx,
    const std::vector<llama_token> & tokens,
    const adaptive_config & config,
    float * logits
) {
    // Step 1: Embedding
    auto * input = llama_get_embedding(ctx);
    
    // Step 2: Iterate through layers
    for (int layer_idx = 0; layer_idx < ctx->model.n_layers; layer_idx++) {
        // Check layer mask
        if (!config.layer_masks[layer_idx]) {
            // Skip this layer
            continue;
        }
        
        // Step 3: Apply width scaling
        int active_heads = (int)(ctx->model.hparams.n_head * config.width_factor);
        
        // Step 4: Adaptive attention
        llama_forward_attention_adaptive(
            ctx, layer_idx, input,
            active_heads,
            config.width_factor
        );
        
        // Step 5: Expert routing (if MoE)
        if (is_moe_layer(layer_idx)) {
            llama_forward_moe_adaptive(
                ctx, layer_idx, input,
                config.expert_masks
            );
        }
        
        // Step 6: Residual + LayerNorm
        llama_add_residual(ctx, layer_idx);
        llama_apply_norm(ctx, layer_idx);
    }
    
    // Step 7: LM Head
    llama_forward_lm_head(ctx, logits);
}
```

### 12.4 KV-Cache Integration

llama.cpp already has robust KV-cache management. We extend it:

```cpp
struct llama_kv_cache_adaptive {
    llama_kv_cache base;
    std::vector<bool> layer_invalidation_flags;  // Mark layers needing cache clear
    int current_width_factor;  // Track width to detect changes
};

void llama_invalidate_cache_on_mask_change(
    llama_context * ctx,
    const std::vector<bool> & old_masks,
    const std::vector<bool> & new_masks
) {
    auto * cache = (llama_kv_cache_adaptive *)ctx->kv_cache;
    
    for (int i = 0; i < old_masks.size(); i++) {
        if (old_masks[i] != new_masks[i]) {
            // Layer i changed state; invalidate its cache
            cache->layer_invalidation_flags[i] = true;
            // Optionally: llama_kv_cache_clear_layer(cache, i);
        }
    }
}
```

### 12.5 LoRA Integration

LoRA is already supported in llama.cpp (via GGML). We integrate:

```cpp
void llama_forward_with_lora(
    llama_context * ctx,
    const std::vector<llama_lora> & adapters
) {
    // For each layer
    for (int layer_idx = 0; layer_idx < ctx->model.n_layers; layer_idx++) {
        // Compute base layer output
        ggml_tensor * output = llama_forward_layer(ctx, layer_idx);
        
        // For each active LoRA adapter
        for (const auto & adapter : adapters) {
            if (adapter.applies_to_layer(layer_idx)) {
                // Compute: output += (x @ A) @ B^T
                ggml_tensor * x_proj = ggml_mul_mat(ctx->compute_graph, adapter.A, output);
                ggml_tensor * delta = ggml_mul_mat(ctx->compute_graph, adapter.B, x_proj);
                output = ggml_add(ctx->compute_graph, output, delta);
            }
        }
        
        // Continue with residual + norm
        llama_add_residual(ctx, layer_idx);
    }
}
```

---

## 13. Recommendation Summary

### Key Design Decisions

| Component | Recommendation | Rationale |
|-----------|----------------|-----------|
| **Layer Masking** | Binary masks (bool), soft masks for training | Simplicity, zero-copy |
| **Width Scaling** | Column slicing from weight matrices | No weight copying |
| **Expert Routing** | Masking via -inf logits, top-K selection | Compatible with softmax |
| **LoRA** | Efficient stacking via (x @ A) @ B^T | O(rank) instead of O(d^2) |
| **Position Encoding** | Dimension-aware (truncate/interpolate) | Works across width factors |
| **KV Cache** | Per-layer caches, invalidate on mask change | Standard llama.cpp design |
| **Batching** | Separate batches per config (Strategy 1) | Simplicity over throughput |
| **Training** | Soft masks with gradient flow | Enable layer selection learning |
| **Verification** | Unit tests + numerical stability checks | Catch silent failures |

### Implementation Roadmap

**Phase 1 (Foundation):** Layer masks + width scaling (Weeks 1-2)
- Implement adaptive attention/FFN
- Test correctness on small models
- Benchmark overhead

**Phase 2 (Completeness):** Expert routing + LoRA (Weeks 3-4)
- Selective MoE with expert masks
- LoRA adapter composition
- Integrate with llama.cpp

**Phase 3 (Production):** Performance + Integration (Weeks 5-6)
- Custom CUDA kernels for critical paths
- Full bonsai-inference integration
- Comprehensive tests + benchmarks

**Phase 4 (Optimization):** Advanced Features (Weeks 7+)
- Mixed-precision training
- Dynamic layer selection
- Unified batching (Strategy 2)

---

## Appendix: Notation Reference

| Symbol | Meaning |
|--------|---------|
| `d` | hidden_dim |
| `n` | num_layers |
| `h` | num_heads |
| `r` | LoRA rank |
| `k` | num_experts or top-K |
| `α` | width_factor |
| `W_q, W_k, W_v` | Attention projections |
| `W_router` | MoE router weights |
| `A_i, B_i` | LoRA matrices for adapter i |
| `m[i]` | Mask for layer i (bool or float) |

---

**Document End**

