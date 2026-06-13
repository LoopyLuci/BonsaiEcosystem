# Bonsai Inference Fabric

The **Inference Fabric** is a production-grade replacement for Ollama, fully integrated with the Bonsai Ecosystem and deeply optimized for local and distributed inference.

## Overview

The Inference Fabric provides:

- **Model Registry** — Discover, pull, and manage models
- **GPU/CPU Orchestration** — Automatic layer offloading and device selection
- **Quantization Pipeline** — Build custom quantized models
- **OpenAI-Compatible API** — Drop-in replacement for `chat/completions`
- **Token Streaming** — Real-time token generation
- **Tool Calling** — First-class support for agent tools
- **Telemetry** — Complete observability via structured logs
- **Sanctum Sandboxing** — Cryptographic isolation for inference

## Quick Start

### 1. Download a Model

```bash
bonsai pull llama-3-8b
```

Available models:
- `llama-2-7b` — 7B parameters, 4GB quantized
- `llama-3-8b` — 8B parameters, 5GB quantized
- `mistral-7b` — 7B parameters, high-quality
- `neural-chat-7b` — Fine-tuned for conversation

### 2. List Local Models

```bash
bonsai list
```

Output:
```
llama-3-8b           5.2GB  q4_k_m  ✓ verified
mistral-7b           4.8GB  q4_k_m  ✓ verified
neural-chat-7b       3.1GB  q3_k_s  ✓ verified
```

### 3. Start Interactive Chat

```bash
bonsai run llama-3-8b

> What is Bonsai?
Bonsai is a production-grade...
```

### 4. Start API Server

```bash
bonsai serve --port 11425
```

Navigate to: **http://127.0.0.1:11425/docs**

## OpenAI-Compatible API

The Inference Fabric exposes an OpenAI-compatible endpoint:

### Endpoint

```
POST http://127.0.0.1:11425/v1/chat/completions
```

### Example

```bash
curl -X POST http://127.0.0.1:11425/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer sk-local" \
  -d '{
    "model": "llama-3-8b",
    "messages": [
      {"role": "user", "content": "What is Bonsai?"}
    ],
    "temperature": 0.7,
    "max_tokens": 512,
    "stream": true
  }'
```

### Python Example

```python
from openai import OpenAI

client = OpenAI(
    api_key="sk-local",
    base_url="http://127.0.0.1:11425/v1"
)

response = client.chat.completions.create(
    model="llama-3-8b",
    messages=[
        {"role": "user", "content": "What is Bonsai?"}
    ]
)

print(response.choices[0].message.content)
```

### TypeScript Example

```typescript
import { OpenAI } from 'openai';

const client = new OpenAI({
  apiKey: 'sk-local',
  baseURL: 'http://127.0.0.1:11425/v1',
  dangerouslyAllowBrowser: true,
});

const completion = await client.chat.completions.create({
  model: 'llama-3-8b',
  messages: [{ role: 'user', content: 'What is Bonsai?' }],
});

console.log(completion.choices[0].message.content);
```

## Custom Models with Bluebonnet

Create custom models using **Bluebonnet blueprints**:

### Create a Custom Model

```toml
# bluebonnet-neural-chat.toml
[model]
name = "neural-chat-custom"
base = "llama-3-8b"
version = "1.0"

[quantization]
method = "q4_k_m"
precision = "float16"

[parameters]
temperature = 0.6
top_p = 0.95
context_window = 8192
max_tokens = 2048

[system_prompt]
content = """
You are a helpful AI assistant built by Bonsai. You are knowledgeable about software development,
AI, and the Bonsai Ecosystem. You provide clear, concise, and accurate answers.
"""

[tools]
allowed = ["read_file", "write_file", "run_cargo_check"]
max_parallel = 4
timeout_seconds = 60

[fine_tuning]
base_model = "llama-3-8b"
dataset = "path/to/training-data.jsonl"
epochs = 3
learning_rate = 1e-5
```

### Build the Model

```bash
bonsai create bluebonnet-neural-chat.toml
```

Output:
```
Building neural-chat-custom:1.0...
✓ Downloading base model (llama-3-8b)
✓ Loading training dataset (5,432 examples)
✓ Fine-tuning with LoRA (4 GPU hours)
✓ Quantizing to q4_k_m (2.1 GB)
✓ Verifying outputs
✓ Signing with Ed25519

Model: neural-chat-custom:1.0
Size: 2.1 GB
Signature: <public-key-hash>
Crystal Hash: blake3:abc123...
```

### Test the Model

```bash
bonsai run neural-chat-custom:1.0
```

## Model Registry

### Publish a Model

```bash
bonsai push my-model --public
```

This publishes to the Bonsai Model Registry, making it discoverable by others.

### List Published Models

```bash
bonsai registry search --filter size=small
```

### Model Manifest

Each model has a manifest:

```json
{
  "name": "llama-3-8b",
  "version": "1.0",
  "family": "llama",
  "parameters_billion": 8.0,
  "context_length": 8192,
  "quantization": "q4_k_m",
  "crystal_hash": "blake3:abc123...",
  "size_bytes": 5368709120,
  "created_at": "2026-05-31T12:00:00Z",
  "author": "meta",
  "license": "LLAMA2",
  "verified": true,
  "signature_public_key": "...",
  "capabilities": ["chat", "tools"],
  "benchmarks": {
    "mmlu": 0.79,
    "arc": 0.81,
    "hellaswag": 0.85
  }
}
```

## GPU Optimization

The Inference Fabric automatically optimizes for your hardware:

### Auto-Detection

```bash
bonsai info

Hardware:
  GPU: NVIDIA RTX 4090
  VRAM: 24 GB
  CPU: Intel Core i9-14900K
  RAM: 128 GB

Optimal Configuration:
  Model: llama-3-8b
  Quantization: q4_k_m (GPU offload: 80 layers)
  Batch Size: 16
  Estimated Throughput: 45 tokens/second
```

### Manual GPU Configuration

```toml
[inference]
device = "gpu"
gpu_layers = 80      # Offload 80 layers to GPU
batch_size = 16
num_threads = 16
```

### Multi-GPU Setup

```bash
bonsai serve --gpus 0,1,2,3 --model llama-3-70b
```

The Inference Fabric distributes the model across GPUs automatically.

## Tool Calling

Agents can call tools during inference:

### Define Tools

```json
{
  "name": "read_file",
  "description": "Read a file from disk",
  "parameters": {
    "type": "object",
    "properties": {
      "path": {"type": "string", "description": "File path"}
    },
    "required": ["path"]
  }
}
```

### Agent Uses Tools

```bash
curl -X POST http://127.0.0.1:11425/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "llama-3-8b",
    "messages": [{"role": "user", "content": "Read README.md and summarize it"}],
    "tools": [
      {
        "type": "function",
        "function": {
          "name": "read_file",
          "description": "Read a file from disk",
          "parameters": {"type": "object", "properties": {"path": {"type": "string"}}}
        }
      }
    ]
  }'
```

Response:

```json
{
  "choices": [{
    "message": {
      "role": "assistant",
      "tool_calls": [{
        "id": "call_abc123",
        "function": {
          "name": "read_file",
          "arguments": "{\"path\": \"README.md\"}"
        }
      }]
    }
  }]
}
```

### Tool Executor Integration

```rust
// Execute the tool and return result
let executor = ToolExecutor::new();
let result = executor.call("read_file", json!({"path": "README.md"})).await?;

// Send result back to model for continued generation
let response = model.chat(messages_with_tool_result).await?;
```

## Telemetry & Observability

### Structured Logging

All inference operations are logged as JSON:

```bash
tail -f ~/.cache/bonsai/inference.log
```

Example log:

```json
{
  "timestamp": "2026-05-31T14:32:15.123Z",
  "level": "INFO",
  "event": "inference_complete",
  "model": "llama-3-8b",
  "tokens_generated": 256,
  "duration_ms": 3200,
  "tokens_per_second": 80,
  "gpu_utilization": 85,
  "memory_used_gb": 8.2
}
```

### Prometheus Metrics

```bash
curl http://127.0.0.1:11425/metrics
```

Tracked metrics:
- `inference_tokens_total` — Total tokens generated
- `inference_duration_ms` — Inference latency (histogram)
- `gpu_memory_used_bytes` — GPU memory consumption
- `inference_queue_length` — Pending requests
- `model_load_time_ms` — Model startup time

### OpenTelemetry Integration

Export traces to Jaeger, Datadog, or other backends:

```bash
bonsai serve \
  --otel-endpoint http://localhost:4317 \
  --otel-service-name bonsai-inference
```

## Sanctum Sandboxing

Each inference run happens in an isolated Sanctum vault:

```cml
{
    program: { binary: "pkgs.bonsai-inference" },
    capabilities: { gpu: true, memory: { limit: "16GiB" }, crypto: true },
    compartment: { type: "vault", memory: "16GiB", cpu: 8, isolation: "strict" },
    oath: "gpu memory crypto",
}
```

Benefits:
- ✅ Model cannot escape the sandbox
- ✅ Tool calls are confined to allowed operations
- ✅ Resource limits enforced (memory, CPU, time)
- ✅ Cryptographic attestation of inference

## Advanced Usage

### Batch Inference

Process multiple prompts efficiently:

```python
from bonsai_inference import InferenceEngine

engine = InferenceEngine.load("llama-3-8b")

prompts = [
    "What is Bonsai?",
    "How does UACS work?",
    "Explain GPU inference.",
]

results = engine.batch_infer(prompts, max_tokens=256)

for prompt, result in zip(prompts, results):
    print(f"Q: {prompt}")
    print(f"A: {result.text}\n")
```

### Streaming Responses

```python
async for chunk in engine.stream_infer(
    messages=[{"role": "user", "content": "Write a poem about Bonsai."}],
    model="llama-3-8b"
):
    print(chunk.text, end="", flush=True)
```

### Token Budget Control

```bash
bonsai run llama-3-8b \
  --max-tokens 2048 \
  --context-window 8192 \
  --stop-sequences "User:", "Assistant:"
```

### Model Caching

Loaded models are kept in memory for fast switching:

```bash
bonsai serve --model-cache-size 3 --models llama-3-8b,mistral-7b,neural-chat-7b
```

Switching between cached models is instant (~10ms).

## Troubleshooting

### Out of Memory

If you get OOM errors:

1. Check available VRAM:
   ```bash
   bonsai info
   ```

2. Use a smaller quantization:
   ```bash
   bonsai pull llama-3-8b:q3_k_s
   ```

3. Reduce batch size:
   ```bash
   bonsai serve --batch-size 4
   ```

### Slow Inference

If inference is slow:

1. Check GPU utilization:
   ```bash
   nvidia-smi
   ```

2. Ensure layers are offloaded to GPU:
   ```bash
   bonsai info --verbose
   ```

3. Increase batch size (if latency, not throughput, is the concern).

### Model Won't Load

If a model fails to load:

```bash
bonsai pull <model> --force  # Re-download
bonsai verify <model>        # Check integrity
bonsai cache clear           # Clear local cache
```

## Metrics & Benchmarks

Typical performance on modern hardware:

| Model | Quantization | GPU (RTX 4090) | CPU (i9-14900K) |
|-------|--------------|----------------|-----------------|
| llama-3-8b | q4_k_m | 80 tok/s | 12 tok/s |
| mistral-7b | q4_k_m | 95 tok/s | 14 tok/s |
| llama-3-70b | q4_k_m | 12 tok/s | 0.5 tok/s |

Memory requirements:

| Model | q4_k_m | q3_k_s | q2_k |
|-------|--------|--------|------|
| llama-3-8b | 5.2GB | 3.1GB | 2.1GB |
| mistral-7b | 4.8GB | 2.9GB | 1.9GB |
| llama-3-70b | 38GB | 22GB | 14GB |

## Integration Examples

### With Claude

```python
from openai import OpenAI

client = OpenAI(
    api_key="sk-local",
    base_url="http://127.0.0.1:11425/v1"
)

# Claude can now use local inference
response = client.chat.completions.create(
    model="llama-3-8b",
    messages=[
        {"role": "system", "content": "You are a helpful Bonsai engineer."},
        {"role": "user", "content": "Fix this code..."}
    ]
)
```

### With LangChain

```python
from langchain.llms.openai import OpenAI
from langchain.chains import LLMChain

llm = OpenAI(
    model_name="llama-3-8b",
    openai_api_base="http://127.0.0.1:11425/v1",
    openai_api_key="sk-local",
    temperature=0.7
)

# Use with LangChain as normal
chain = LLMChain(llm=llm, prompt=...)
```

### With LlamaIndex

```python
from llama_index.llms import OpenAI
from llama_index.callbacks import CallbackManager

llm = OpenAI(
    model="llama-3-8b",
    api_base="http://127.0.0.1:11425/v1",
    api_key="sk-local",
)

# Create index, query documents, etc.
index = VectorStoreIndex.from_documents(docs, llm=llm)
```

---

The Inference Fabric is your local, private, fully-controlled inference engine. 🧠🚀
