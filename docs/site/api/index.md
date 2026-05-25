# API Reference

Base URL: `http://127.0.0.1:<port>/api/v1`

| Method | Path | Description |
|--------|------|-------------|
| POST | `/chat` | Send a chat message |
| POST | `/agents/message` | Send a message to a named agent |
| POST | `/sandbox/run` | Execute code in a sandbox |
| POST | `/images/generate` | Generate an image via Stable Diffusion |
| POST | `/tts/speak` | Synthesise speech via Piper |
| POST | `/compare` | Dual-model comparison |
| POST | `/training/loop/start` | Start continuous training loop |
| POST | `/training/loop/stop` | Stop the training loop |
| GET  | `/training/loop/status` | Training loop status |
| GET  | `/features` | Feature flag state |
| GET  | `/core/stats` | GPU and model stats |
