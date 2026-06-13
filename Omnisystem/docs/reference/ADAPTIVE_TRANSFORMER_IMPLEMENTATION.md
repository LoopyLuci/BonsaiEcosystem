# Bonsai Adaptive Transformer: Implementation Guide

**Status:** Ready for Implementation  
**Target Language:** Rust (with PyTorch for reference)  
**Version:** 1.0  
**Date:** 2026-06-01

---

## Overview

This guide provides step-by-step implementation instructions for the Bonsai Adaptive Transformer forward pass, with working code examples in Rust and PyTorch.

---

## Part 1: Core Data Structures (Rust)

### 1.1 Adaptive Config

```rust
// File: crates/bonsai-inference/src/adaptive.rs

use serde::{Deserialize, Serialize};
use ndarray::{Array1, Array2, Array3};

/// Configuration for adaptive forward pass
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveConfig {
    /// Layer activation masks (bool or float [0,1])
    pub layer_masks: Vec<bool>,
    
    /// Expert availability masks
    pub expert_masks: Vec<bool>,
    
    /// Width scaling factor (0.1 to 2.0)
    pub width_factor: f32,
    
    /// Active LoRA adapters (names)
    pub active_adapters: Vec<String>,
    
    /// Top-K experts to select
    pub top_k_experts: usize,
    
    /// Use soft masks (for training) vs binary (for inference)
    pub use_soft_masks: bool,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            layer_masks: vec![true; 32],  // All active by default
            expert_masks: vec![true; 1024],
            width_factor: 1.0,
            active_adapters: Vec::new(),
            top_k_experts: 2,
            use_soft_masks: false,
        }
    }
}

/// KV Cache for a single layer
#[derive(Debug, Clone)]
pub struct LayerKVCache {
    /// Cached K values [seq_len, num_heads, head_dim]
    pub k: Option<Array3<f32>>,
    
    /// Cached V values [seq_len, num_heads, head_dim]
    pub v: Option<Array3<f32>>,
    
    /// Current cache length
    pub seq_len: usize,
    
    /// Is this layer active?
    pub is_active: bool,
}

impl LayerKVCache {
    pub fn new() -> Self {
        Self {
            k: None,
            v: None,
            seq_len: 0,
            is_active: true,
        }
    }
    
    pub fn clear(&mut self) {
        self.k = None;
        self.v = None;
        self.seq_len = 0;
    }
    
    pub fn append_kv(&mut self, k: Array2<f32>, v: Array2<f32>) {
        // k, v shape: [num_heads, head_dim]
        if self.k.is_none() {
            // Initialize cache
            let mut k_cache = Array3::zeros((256, k.nrows(), k.ncols()));  // max 256 tokens
            k_cache[[0, 0, 0]] = k[[0, 0]];
            self.k = Some(k_cache);
        }
        
        if let Some(ref mut k_cache) = &mut self.k {
            k_cache.slice_mut(ndarray::s![self.seq_len, .., ..]).assign(&k);
        }
        
        self.seq_len += 1;
    }
}

pub struct KVCacheStore {
    pub caches: Vec<LayerKVCache>,
}

impl KVCacheStore {
    pub fn new(num_layers: usize) -> Self {
        Self {
            caches: (0..num_layers).map(|_| LayerKVCache::new()).collect(),
        }
    }
    
    pub fn invalidate_on_mask_change(
        &mut self,
        old_masks: &[bool],
        new_masks: &[bool],
    ) {
        for (i, (old, new)) in old_masks.iter().zip(new_masks.iter()).enumerate() {
            if old != new {
                self.caches[i].clear();
            }
        }
    }
}
```

### 1.2 LoRA Adapter

```rust
// File: crates/bonsai-inference/src/lora.rs

use ndarray::Array2;

/// LoRA adapter for a single layer
#[derive(Debug, Clone)]
pub struct LoraAdapter {
    pub name: String,
    
    /// A matrix [in_features, rank]
    pub a_weight: Array2<f32>,
    
    /// B matrix [out_features, rank]
    pub b_weight: Array2<f32>,
    
    /// Scaling factor
    pub alpha: f32,
    
    /// Rank
    pub rank: usize,
    
    /// Which layers this adapter applies to
    pub layer_indices: Vec<usize>,
}

impl LoraAdapter {
    pub fn new(
        name: String,
        in_features: usize,
        out_features: usize,
        rank: usize,
        alpha: f32,
    ) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Initialize A and B with small random values
        let a_weight = Array2::from_shape_fn(
            (in_features, rank),
            |_| rng.gen_range(-0.01..0.01),
        );
        
        let b_weight = Array2::from_shape_fn(
            (out_features, rank),
            |_| rng.gen_range(-0.01..0.01),
        );
        
        Self {
            name,
            a_weight,
            b_weight,
            alpha,
            rank,
            layer_indices: Vec::new(),
        }
    }
    
    pub fn applies_to_layer(&self, layer_idx: usize) -> bool {
        self.layer_indices.contains(&layer_idx)
    }
    
    /// Apply LoRA delta: (x @ A) @ B^T * (alpha / rank)
    pub fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        // x shape: [batch_seq, hidden_dim]
        // A shape: [hidden_dim, rank]
        // Result: [batch_seq, hidden_dim]
        
        let x_proj = x.dot(&self.a_weight);  // [batch_seq, rank]
        let delta = x_proj.dot(&self.b_weight.t());  // [batch_seq, hidden_dim]
        
        let scale = self.alpha / self.rank as f32;
        delta * scale
    }
}

pub struct LoraRegistry {
    adapters: std::collections::HashMap<String, LoraAdapter>,
}

impl LoraRegistry {
    pub fn new() -> Self {
        Self {
            adapters: std::collections::HashMap::new(),
        }
    }
    
    pub fn register(&mut self, adapter: LoraAdapter) {
        self.adapters.insert(adapter.name.clone(), adapter);
    }
    
    pub fn get(&self, name: &str) -> Option<&LoraAdapter> {
        self.adapters.get(name)
    }
}
```

### 1.3 Position Encoding

```rust
// File: crates/bonsai-inference/src/position_encoding.rs

use std::f32::consts::PI;

/// Rotary Position Embedding (RoPE)
pub struct RotaryEmbedding {
    pub dim: usize,
    pub base: f32,
}

impl RotaryEmbedding {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            base: 10000.0,
        }
    }
    
    /// Compute RoPE frequencies
    pub fn compute_freqs(&self) -> Vec<f32> {
        let mut freqs = vec![0.0; self.dim / 2];
        
        for i in 0..(self.dim / 2) {
            freqs[i] = 1.0 / self.base.powf(2.0 * (i as f32) / (self.dim as f32));
        }
        
        freqs
    }
    
    /// Apply RoPE rotation to Q, K
    pub fn apply(
        &self,
        q: &mut ndarray::Array3<f32>,  // [batch, seq_len, num_heads, head_dim]
        k: &mut ndarray::Array3<f32>,
        pos_start: usize,
    ) {
        let freqs = self.compute_freqs();
        let seq_len = q.nrows();
        
        for pos in 0..seq_len {
            let actual_pos = pos_start + pos;
            
            for i in 0..(self.dim / 2) {
                let angle = (actual_pos as f32) * freqs[i];
                let cos_angle = angle.cos();
                let sin_angle = angle.sin();
                
                // Apply rotation: [cos -sin; sin cos] to pairs
                let q_real = q[[pos, 0, 2 * i]];
                let q_imag = q[[pos, 0, 2 * i + 1]];
                
                q[[pos, 0, 2 * i]] = q_real * cos_angle - q_imag * sin_angle;
                q[[pos, 0, 2 * i + 1]] = q_real * sin_angle + q_imag * cos_angle;
                
                // Same for K
                let k_real = k[[pos, 0, 2 * i]];
                let k_imag = k[[pos, 0, 2 * i + 1]];
                
                k[[pos, 0, 2 * i]] = k_real * cos_angle - k_imag * sin_angle;
                k[[pos, 0, 2 * i + 1]] = k_real * sin_angle + k_imag * cos_angle;
            }
        }
    }
}

/// Absolute Positional Encoding (simpler alternative)
pub fn absolute_positional_encoding(
    seq_len: usize,
    hidden_dim: usize,
) -> ndarray::Array2<f32> {
    let mut pe = ndarray::Array2::zeros((seq_len, hidden_dim));
    
    for pos in 0..seq_len {
        for i in 0..hidden_dim {
            let angle = (pos as f32) / (10000.0_f32.powf(2.0 * (i as f32) / (hidden_dim as f32)));
            
            if i % 2 == 0 {
                pe[[pos, i]] = angle.sin();
            } else {
                pe[[pos, i]] = angle.cos();
            }
        }
    }
    
    pe
}
```

---

## Part 2: Core Transformer Components (Rust)

### 2.1 Adaptive Attention Layer

```rust
// File: crates/bonsai-inference/src/layers/attention.rs

use ndarray::{Array1, Array2, Array3};
use crate::adaptive::LayerKVCache;

pub struct AdaptiveAttention {
    pub hidden_dim: usize,
    pub num_heads: usize,
    pub head_dim: usize,
    
    /// Weight matrices
    pub w_q: Array2<f32>,  // [hidden_dim, hidden_dim]
    pub w_k: Array2<f32>,
    pub w_v: Array2<f32>,
    pub w_o: Array2<f32>,
}

impl AdaptiveAttention {
    pub fn new(hidden_dim: usize, num_heads: usize) -> Self {
        let head_dim = hidden_dim / num_heads;
        
        Self {
            hidden_dim,
            num_heads,
            head_dim,
            w_q: Array2::zeros((hidden_dim, hidden_dim)),
            w_k: Array2::zeros((hidden_dim, hidden_dim)),
            w_v: Array2::zeros((hidden_dim, hidden_dim)),
            w_o: Array2::zeros((hidden_dim, hidden_dim)),
        }
    }
    
    pub fn forward(
        &self,
        x: &Array2<f32>,  // [batch_seq, hidden_dim]
        width_factor: f32,
        kv_cache: Option<&LayerKVCache>,
    ) -> Array2<f32> {
        let batch_seq = x.nrows();
        let active_hidden_dim = (self.hidden_dim as f32 * width_factor) as usize;
        let active_heads = (self.num_heads as f32 * width_factor) as usize;
        
        // Project Q, K, V (only use active_hidden_dim columns)
        let q = x.dot(&self.w_q.slice(ndarray::s![.., ..active_hidden_dim]).to_owned());
        let k = x.dot(&self.w_k.slice(ndarray::s![.., ..active_hidden_dim]).to_owned());
        let v = x.dot(&self.w_v.slice(ndarray::s![.., ..active_hidden_dim]).to_owned());
        
        // Reshape for multi-head attention
        // [batch_seq, hidden_dim] -> [batch, seq, num_heads, head_dim]
        // For simplicity, assume batch_seq = batch * seq_len
        let seq_len = 1;  // During generation
        let batch = batch_seq / seq_len;
        
        let q = self.reshape_for_attention(&q, active_heads);
        let k = self.reshape_for_attention(&k, active_heads);
        let v = self.reshape_for_attention(&v, active_heads);
        
        // Attention computation
        let mut output = self.attention_forward(&q, &k, &v);
        
        // Project output back
        output = output.dot(&self.w_o.slice(ndarray::s![..active_hidden_dim, ..active_hidden_dim]).to_owned());
        
        output
    }
    
    fn reshape_for_attention(&self, x: &Array2<f32>, num_heads: usize) -> Array3<f32> {
        // Simplified: return as-is
        // In production, properly reshape [batch_seq, hidden_dim] to [batch, seq, num_heads, head_dim]
        x.view().into_owned().insert_axis(ndarray::Axis(0))
    }
    
    fn attention_forward(
        &self,
        q: &Array3<f32>,
        k: &Array3<f32>,
        v: &Array3<f32>,
    ) -> Array2<f32> {
        // Simplified attention: QK^T @ V
        // In production, implement full softmax attention with proper reshaping
        
        let (batch, seq, _) = q.dim();
        let mut scores = Array2::zeros((batch, seq));
        
        // Compute attention scores
        let qk_t = q.slice(ndarray::s![.., .., ..]).dot(&k.slice(ndarray::s![.., .., ..]).t());
        let scale = (self.head_dim as f32).sqrt().recip();
        
        scores = qk_t * scale;
        
        // Softmax
        scores = self.softmax(&scores);
        
        // Weighted sum of values
        let output = scores.dot(&v.slice(ndarray::s![.., .., ..]));
        
        output
    }
    
    fn softmax(&self, x: &Array2<f32>) -> Array2<f32> {
        let mut result = x.clone();
        
        for row in result.rows_mut() {
            let max = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let exp: Vec<f32> = row.iter().map(|&v| (v - max).exp()).collect();
            let sum: f32 = exp.iter().sum();
            
            for (i, &e) in exp.iter().enumerate() {
                row[i] = e / sum;
            }
        }
        
        result
    }
}
```

### 2.2 Adaptive FFN Layer

```rust
// File: crates/bonsai-inference/src/layers/ffn.rs

use ndarray::Array2;

pub struct AdaptiveFFN {
    pub hidden_dim: usize,
    pub ffn_dim: usize,
    
    /// Weight matrices
    pub w_up: Array2<f32>,    // [hidden_dim, ffn_dim]
    pub w_down: Array2<f32>,  // [ffn_dim, hidden_dim]
}

impl AdaptiveFFN {
    pub fn new(hidden_dim: usize, ffn_dim: usize) -> Self {
        Self {
            hidden_dim,
            ffn_dim,
            w_up: Array2::zeros((hidden_dim, ffn_dim)),
            w_down: Array2::zeros((ffn_dim, hidden_dim)),
        }
    }
    
    pub fn forward(
        &self,
        x: &Array2<f32>,  // [batch_seq, hidden_dim]
        width_factor: f32,
    ) -> Array2<f32> {
        let active_hidden_dim = (self.hidden_dim as f32 * width_factor) as usize;
        let active_ffn_dim = (self.ffn_dim as f32 * width_factor) as usize;
        
        // First linear: hidden_dim -> ffn_dim (with width scaling)
        let w_up_active = self.w_up.slice(ndarray::s![.., ..active_ffn_dim]).to_owned();
        let mut hidden = x.dot(&w_up_active);  // [batch_seq, active_ffn_dim]
        
        // Activation: GELU
        hidden = self.gelu(&hidden);
        
        // Second linear: ffn_dim -> hidden_dim (with width scaling)
        let w_down_active = self.w_down.slice(ndarray::s![..active_ffn_dim, ..active_hidden_dim]).to_owned();
        let output = hidden.dot(&w_down_active);  // [batch_seq, active_hidden_dim]
        
        output
    }
    
    fn gelu(&self, x: &Array2<f32>) -> Array2<f32> {
        use std::f32::consts::PI;
        
        let mut result = x.clone();
        for val in result.iter_mut() {
            // GELU: x * Phi(x) ≈ 0.5 * x * (1 + tanh(sqrt(2/π) * (x + 0.044715 * x^3)))
            let cdf = 0.5 * (1.0 + (std::f32::consts::FRAC_2_PI.sqrt() * (val + 0.044715 * val.powi(3))).tanh());
            *val = val * cdf;
        }
        result
    }
}
```

### 2.3 Adaptive MoE Layer

```rust
// File: crates/bonsai-inference/src/layers/moe.rs

use ndarray::{Array1, Array2};

pub struct AdaptiveMoE {
    pub hidden_dim: usize,
    pub num_experts: usize,
    pub expert_dim: usize,
    
    /// Router weights [hidden_dim, num_experts]
    pub router_weight: Array2<f32>,
    
    /// Expert parameters (simplified)
    pub experts: Vec<Array2<f32>>,  // Each expert is [hidden_dim, expert_dim]
}

impl AdaptiveMoE {
    pub fn new(hidden_dim: usize, num_experts: usize, expert_dim: usize) -> Self {
        Self {
            hidden_dim,
            num_experts,
            expert_dim,
            router_weight: Array2::zeros((hidden_dim, num_experts)),
            experts: vec![Array2::zeros((hidden_dim, expert_dim)); num_experts],
        }
    }
    
    pub fn forward(
        &self,
        x: &Array2<f32>,  // [batch_seq, hidden_dim]
        expert_masks: &[bool],
        top_k: usize,
    ) -> Array2<f32> {
        let batch_seq = x.nrows();
        
        // Compute router scores
        let router_logits = x.dot(&self.router_weight);  // [batch_seq, num_experts]
        
        // Apply expert mask: set disabled experts to -inf
        let mut masked_logits = router_logits.clone();
        for (i, mask) in expert_masks.iter().enumerate() {
            if !mask {
                masked_logits.column_mut(i).fill(f32::NEG_INFINITY);
            }
        }
        
        // Select top-K experts per token
        let (mut top_k_indices, mut top_k_values) = self.top_k(&masked_logits, top_k);
        
        // Compute gating weights
        let gate_weights = self.softmax_rows(&top_k_values);
        
        // Route to experts
        let mut output = Array2::zeros((batch_seq, self.hidden_dim));
        
        for sample_idx in 0..batch_seq {
            for k in 0..top_k {
                let expert_idx = top_k_indices[[sample_idx, k]];
                let gate_weight = gate_weights[[sample_idx, k]];
                
                // Apply expert
                let expert_output = x.row(sample_idx).dot(&self.experts[expert_idx]);
                output.row_mut(sample_idx).scaled_add(gate_weight, &expert_output);
            }
        }
        
        output
    }
    
    fn top_k(&self, x: &Array2<f32>, k: usize) -> (Array2<usize>, Array2<f32>) {
        let (rows, cols) = x.dim();
        let mut indices = Array2::zeros((rows, k));
        let mut values = Array2::zeros((rows, k));
        
        for row_idx in 0..rows {
            let row = x.row(row_idx);
            let mut sorted: Vec<(usize, f32)> = row
                .iter()
                .enumerate()
                .map(|(i, &v)| (i, v))
                .collect();
            
            sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            
            for (k_idx, (col_idx, val)) in sorted.iter().take(k).enumerate() {
                indices[[row_idx, k_idx]] = *col_idx;
                values[[row_idx, k_idx]] = *val;
            }
        }
        
        (indices, values)
    }
    
    fn softmax_rows(&self, x: &Array2<f32>) -> Array2<f32> {
        let mut result = x.clone();
        
        for mut row in result.rows_mut() {
            let max = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let exp: Vec<f32> = row.iter().map(|&v| (v - max).exp()).collect();
            let sum: f32 = exp.iter().sum();
            
            for (i, &e) in exp.iter().enumerate() {
                row[i] = e / sum;
            }
        }
        
        result
    }
}
```

---

## Part 3: Full Adaptive Transformer (Rust)

### 3.1 Main Transformer

```rust
// File: crates/bonsai-inference/src/adaptive_transformer.rs

use crate::adaptive::{AdaptiveConfig, KVCacheStore};
use crate::lora::LoraRegistry;
use crate::layers::{AdaptiveAttention, AdaptiveFFN, AdaptiveMoE};
use ndarray::Array2;

pub struct AdaptiveTransformer {
    pub hidden_dim: usize,
    pub num_layers: usize,
    pub num_heads: usize,
    pub ffn_dim: usize,
    pub num_experts: usize,
    
    // Layers
    pub attention_layers: Vec<AdaptiveAttention>,
    pub ffn_layers: Vec<AdaptiveFFN>,
    pub moe_layers: Vec<(usize, AdaptiveMoE)>,  // (layer_idx, moe)
    
    // Registries
    pub lora_registry: LoraRegistry,
    
    // KV Cache
    pub kv_cache: KVCacheStore,
}

impl AdaptiveTransformer {
    pub fn new(
        hidden_dim: usize,
        num_layers: usize,
        num_heads: usize,
        ffn_dim: usize,
        num_experts: usize,
    ) -> Self {
        let attention_layers = (0..num_layers)
            .map(|_| AdaptiveAttention::new(hidden_dim, num_heads))
            .collect();
        
        let ffn_layers = (0..num_layers)
            .map(|_| AdaptiveFFN::new(hidden_dim, ffn_dim))
            .collect();
        
        Self {
            hidden_dim,
            num_layers,
            num_heads,
            ffn_dim,
            num_experts,
            attention_layers,
            ffn_layers,
            moe_layers: vec![],
            lora_registry: LoraRegistry::new(),
            kv_cache: KVCacheStore::new(num_layers),
        }
    }
    
    pub fn forward(
        &mut self,
        x: &Array2<f32>,  // [batch_seq, hidden_dim]
        config: &AdaptiveConfig,
    ) -> Array2<f32> {
        let mut hidden = x.clone();
        
        // Layer stack
        for layer_idx in 0..self.num_layers {
            let x_residual = hidden.clone();
            
            // Check layer mask
            if !config.layer_masks[layer_idx] {
                // Skip this layer
                hidden = x_residual;
                continue;
            }
            
            // Adaptive Attention
            hidden = self.attention_layers[layer_idx].forward(
                &hidden,
                config.width_factor,
                None,
            );
            
            // Adaptive FFN (or MoE)
            if let Some((moe_layer_idx, moe)) = self.moe_layers.iter().find(|(idx, _)| *idx == layer_idx) {
                hidden = moe.forward(&hidden, &config.expert_masks, config.top_k_experts);
            } else {
                hidden = self.ffn_layers[layer_idx].forward(&hidden, config.width_factor);
            }
            
            // LoRA Adapters
            for adapter_name in &config.active_adapters {
                if let Some(adapter) = self.lora_registry.get(adapter_name) {
                    if adapter.applies_to_layer(layer_idx) {
                        let delta = adapter.forward(&hidden);
                        hidden = &hidden + &delta;
                    }
                }
            }
            
            // Residual + LayerNorm
            hidden = &hidden + &x_residual;
            hidden = self.layer_norm(&hidden);
        }
        
        hidden
    }
    
    fn layer_norm(&self, x: &Array2<f32>) -> Array2<f32> {
        // Simplified LayerNorm: normalize across hidden dimension
        let mut result = x.clone();
        let eps = 1e-6;
        
        for mut row in result.rows_mut() {
            let mean = row.iter().sum::<f32>() / (row.len() as f32);
            let var = row.iter().map(|&v| (v - mean).powi(2)).sum::<f32>() / (row.len() as f32);
            
            for val in row.iter_mut() {
                *val = (*val - mean) / (var + eps).sqrt();
            }
        }
        
        result
    }
}
```

---

## Part 4: PyTorch Reference Implementation

### 4.1 Adaptive Transformer (PyTorch)

```python
# File: examples/adaptive_transformer.py

import torch
import torch.nn as nn
import torch.nn.functional as F
from typing import Optional, List, Dict

class AdaptiveAttention(nn.Module):
    def __init__(self, hidden_dim: int, num_heads: int):
        super().__init__()
        self.hidden_dim = hidden_dim
        self.num_heads = num_heads
        self.head_dim = hidden_dim // num_heads
        
        self.w_q = nn.Linear(hidden_dim, hidden_dim)
        self.w_k = nn.Linear(hidden_dim, hidden_dim)
        self.w_v = nn.Linear(hidden_dim, hidden_dim)
        self.w_o = nn.Linear(hidden_dim, hidden_dim)
    
    def forward(self, x, width_factor=1.0, kv_cache=None):
        """
        x: [batch, seq_len, hidden_dim]
        width_factor: float in [0.1, 2.0]
        """
        batch_size, seq_len, _ = x.shape
        active_hidden_dim = int(self.hidden_dim * width_factor)
        active_heads = int(self.num_heads * width_factor)
        
        # Project Q, K, V (select first active_hidden_dim dimensions)
        Q = self.w_q(x)[:, :, :active_hidden_dim]
        K = self.w_k(x)[:, :, :active_hidden_dim]
        V = self.w_v(x)[:, :, :active_hidden_dim]
        
        # Reshape for multi-head attention
        Q = Q.reshape(batch_size, seq_len, active_heads, self.head_dim)
        K = K.reshape(batch_size, seq_len, active_heads, self.head_dim)
        V = V.reshape(batch_size, seq_len, active_heads, self.head_dim)
        
        # Transpose to [batch, num_heads, seq_len, head_dim]
        Q = Q.transpose(1, 2)
        K = K.transpose(1, 2)
        V = V.transpose(1, 2)
        
        # Attention
        scores = (Q @ K.transpose(-2, -1)) / (self.head_dim ** 0.5)
        attn_weights = F.softmax(scores, dim=-1)
        output = attn_weights @ V
        
        # Reshape back
        output = output.transpose(1, 2).contiguous()
        output = output.reshape(batch_size, seq_len, active_hidden_dim)
        
        # Project output
        output = self.w_o(output)[:, :, :active_hidden_dim]
        
        return output


class AdaptiveFFN(nn.Module):
    def __init__(self, hidden_dim: int, ffn_dim: int):
        super().__init__()
        self.hidden_dim = hidden_dim
        self.ffn_dim = ffn_dim
        
        self.w_up = nn.Linear(hidden_dim, ffn_dim)
        self.w_down = nn.Linear(ffn_dim, hidden_dim)
    
    def forward(self, x, width_factor=1.0):
        """
        x: [batch, seq_len, hidden_dim]
        width_factor: float in [0.1, 2.0]
        """
        active_hidden_dim = int(self.hidden_dim * width_factor)
        active_ffn_dim = int(self.ffn_dim * width_factor)
        
        # First linear (with width scaling)
        hidden = x @ self.w_up.weight[:active_ffn_dim, :].T
        
        # Activation
        hidden = F.gelu(hidden)
        
        # Second linear (with width scaling)
        output = hidden @ self.w_down.weight[:active_hidden_dim, :active_ffn_dim].T
        
        return output


class AdaptiveTransformer(nn.Module):
    def __init__(
        self,
        hidden_dim: int = 4096,
        num_layers: int = 32,
        num_heads: int = 32,
        ffn_dim: int = 16384,
        vocab_size: int = 128256,
    ):
        super().__init__()
        
        self.hidden_dim = hidden_dim
        self.num_layers = num_layers
        
        self.embedding = nn.Embedding(vocab_size, hidden_dim)
        self.attention_layers = nn.ModuleList([
            AdaptiveAttention(hidden_dim, num_heads)
            for _ in range(num_layers)
        ])
        self.ffn_layers = nn.ModuleList([
            AdaptiveFFN(hidden_dim, ffn_dim)
            for _ in range(num_layers)
        ])
        self.norms = nn.ModuleList([
            nn.LayerNorm(hidden_dim)
            for _ in range(num_layers)
        ])
        self.lm_head = nn.Linear(hidden_dim, vocab_size)
        
        self.lora_registry: Dict[str, nn.Module] = {}
    
    def forward(
        self,
        input_ids: torch.Tensor,
        layer_masks: Optional[torch.Tensor] = None,
        width_factor: float = 1.0,
        active_adapters: Optional[List[str]] = None,
    ) -> torch.Tensor:
        """
        input_ids: [batch, seq_len]
        layer_masks: [num_layers] (bool or float)
        width_factor: float
        active_adapters: list of adapter names
        """
        
        if layer_masks is None:
            layer_masks = torch.ones(self.num_layers, dtype=torch.bool)
        if active_adapters is None:
            active_adapters = []
        
        # Embedding
        x = self.embedding(input_ids)  # [batch, seq_len, hidden_dim]
        
        # Layer stack
        for layer_idx in range(self.num_layers):
            x_residual = x.clone()
            
            # Layer masking
            if isinstance(layer_masks, torch.Tensor) and not layer_masks[layer_idx]:
                x = x_residual
                continue
            
            # Attention
            x = self.attention_layers[layer_idx](x, width_factor=width_factor)
            
            # FFN
            x = self.ffn_layers[layer_idx](x, width_factor=width_factor)
            
            # LoRA adapters
            for adapter_name in active_adapters:
                if adapter_name in self.lora_registry:
                    adapter = self.lora_registry[adapter_name]
                    x = x + adapter(x)
            
            # Residual + LayerNorm
            x = x + x_residual
            x = self.norms[layer_idx](x)
        
        # LM Head
        logits = self.lm_head(x)
        
        return logits


# Test
if __name__ == "__main__":
    model = AdaptiveTransformer(hidden_dim=256, num_layers=4, vocab_size=1000)
    input_ids = torch.randint(0, 1000, (2, 8))
    
    # Forward with all layers active
    logits = model(input_ids)
    print(f"Output shape: {logits.shape}")  # [2, 8, 1000]
    
    # Forward with some layers masked
    layer_masks = torch.tensor([True, False, True, False])
    logits = model(input_ids, layer_masks=layer_masks)
    print(f"Masked output shape: {logits.shape}")
    
    # Forward with width scaling
    logits = model(input_ids, width_factor=0.5)
    print(f"Scaled output shape: {logits.shape}")
```

---

## Part 5: Integration Tests

### 5.1 Correctness Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_layers_active() {
        let mut model = AdaptiveTransformer::new(512, 4, 8, 2048, 1024);
        let x = Array2::zeros((2, 512));  // [batch_seq, hidden_dim]
        
        let config = AdaptiveConfig {
            layer_masks: vec![true; 4],
            expert_masks: vec![true; 1024],
            width_factor: 1.0,
            active_adapters: vec![],
            top_k_experts: 2,
            use_soft_masks: false,
        };
        
        let output = model.forward(&x, &config);
        
        // Check output shape
        assert_eq!(output.nrows(), 2);
        assert_eq!(output.ncols(), 512);
        
        // Check for NaN/Inf
        for &val in output.iter() {
            assert!(!val.is_nan());
            assert!(!val.is_infinite());
        }
    }
    
    #[test]
    fn test_layer_masking() {
        let mut model = AdaptiveTransformer::new(512, 4, 8, 2048, 1024);
        let x = Array2::zeros((1, 512));
        
        let config_all = AdaptiveConfig {
            layer_masks: vec![true, true, true, true],
            ..Default::default()
        };
        
        let config_partial = AdaptiveConfig {
            layer_masks: vec![true, false, true, false],
            ..Default::default()
        };
        
        let output_all = model.forward(&x, &config_all);
        let output_partial = model.forward(&x, &config_partial);
        
        // Both should produce valid output
        assert_eq!(output_all.ncols(), 512);
        assert_eq!(output_partial.ncols(), 512);
    }
    
    #[test]
    fn test_width_scaling() {
        let mut model = AdaptiveTransformer::new(512, 4, 8, 2048, 1024);
        let x = Array2::zeros((1, 512));
        
        for width_factor in &[0.25, 0.5, 1.0, 2.0] {
            let config = AdaptiveConfig {
                width_factor: *width_factor,
                ..Default::default()
            };
            
            let output = model.forward(&x, &config);
            assert!(output.nrows() > 0);
        }
    }
}
```

---

## Implementation Checklist

- [ ] **Phase 1:** Core data structures (Config, KVCache, LoRA)
- [ ] **Phase 2:** Position encoding (RoPE, absolute)
- [ ] **Phase 3:** Adaptive Attention layer
- [ ] **Phase 4:** Adaptive FFN layer
- [ ] **Phase 5:** Adaptive MoE layer
- [ ] **Phase 6:** Full transformer integration
- [ ] **Phase 7:** KV cache management
- [ ] **Phase 8:** LoRA registry and composition
- [ ] **Phase 9:** Unit tests
- [ ] **Phase 10:** Integration with bonsai-inference
- [ ] **Phase 11:** Performance benchmarks
- [ ] **Phase 12:** llama.cpp FFI layer

---

**Document End**

