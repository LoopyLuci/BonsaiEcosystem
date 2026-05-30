# BonsAI – The AI Assistant

BonsAI is the intelligence at the heart of the Bonsai Ecosystem. It runs entirely on your hardware, has access to your local files and tools, and can be trained on your own data to match your style and domain.

---

## How to Chat

Open the **Chat Panel** (💬 sidebar icon). Type a message in the input box and press `Enter` (or `Shift+Enter` for a newline). BonsAI responds with streaming text — tokens appear as they are generated.

### Chat Features
- **Code blocks** – BonsAI wraps code in fenced blocks with syntax highlighting. Click **Copy** to copy to clipboard, or **Insert** to paste at your cursor.
- **File references** – BonsAI can reference files from your open workspace. It shows a chip with the filename; click it to jump to the file.
- **Conversation memory** – the full conversation history is kept in context (up to the model's context window). Use **Clear Chat** to start fresh.
- **Regenerate** – click the ↺ icon on any response to regenerate with a different random seed.
- **Rate response** – click 👍 / 👎 to add the exchange to your Training Data Library as a preference pair.

---

## Understanding Responses

### Streaming
BonsAI streams token-by-token. You can read the beginning of a long response immediately. If the response is going wrong, click **Stop** to halt generation.

### Tool Calls
BonsAI can call tools during a response. You'll see a **tool call block** appear inline:

```
[🔧 read_file] {"path": "src/main.rs"}
  → 142 lines read
```

After the tool returns its result, BonsAI incorporates it and continues the response. See [Tool Calls](#tool-calls) below.

### Thinking (Chain-of-Thought)
When using a reasoning model (DeepSeek-R1 or similar), BonsAI first emits a `<think>` block showing its internal reasoning. This is collapsed by default; click **Show Thinking** to expand it.

---

## Model Selection

You are not locked into one model. You can switch models at runtime.

### How to Switch
1. Click **Settings → Models** (or `Ctrl+,` → Models tab).
2. Select any loaded GGUF from the dropdown.
3. Click **Load**. The new model loads in seconds; the conversation context is cleared.

### Model Roles
You can assign different models to different roles:

| Role | Purpose | Typical model |
|---|---|---|
| **Chat** | Everyday conversation and coding | Qwen2.5-7B Q4 |
| **Reasoning** | Complex problem solving | DeepSeek-R1-8B Q4 |
| **Code** | Code generation and review | DeepSeek-Coder-7B Q4 |
| **Teacher** | Generates training data | DeepSeek-R1-32B (if available) |

Role assignments live in `config/model_registry.yaml`.

---

## BonsAI Buddy

**Buddy** is a detached, always-on-top chat window for quick interactions without opening the full IDE.

Launch it with:
```powershell
just run -- --mode buddy
```
Or toggle it from the taskbar tray icon.

### Buddy Features
- **Voice synthesis (TTS)** – BonsAI's responses are spoken aloud. Toggle the 🔊 button. Uses the system TTS engine or a local neural TTS model if configured.
- **Voice input** – press and hold the microphone button to dictate. Whisper transcribes your speech in real time.
- **Quick actions** – pre-set buttons for "What's the weather?", "Summarise clipboard", "Run last command", etc. Fully customisable in Settings.
- **Always on top** – stays visible above other windows so you can ask questions while working in another app.
- **Minimal mode** – collapse to a thin bar at screen edge; hover to expand.

---

## Tool Calls

BonsAI can call tools to act on your system. Tools are registered in the **Tool Registry** and each one declares the capabilities (effects) it requires.

### Built-in Tools (selection)

| Tool | What it does |
|---|---|
| `read_file` | Reads a file and returns its contents |
| `write_file` | Writes content to a file |
| `run_command` | Executes a shell command in the project directory |
| `search_files` | Searches file contents with ripgrep |
| `list_directory` | Lists files in a directory |
| `fetch_url` | Downloads a web page (requires `network` capability) |
| `kdb_retrieve` | Queries the Knowledge Database for relevant context |
| `cargo_check` | Runs `cargo check` and returns errors |
| `git_commit` | Stages and commits with a generated message |

### Plan Review Gate
For **destructive actions** (write, delete, run command), BonsAI presents a **Plan** before executing:

```
📋 Plan:
  1. Write updated code to src/lib.rs
  2. Run `cargo check`
  3. If errors: fix and repeat up to 3 times

[Approve] [Reject] [Edit Plan]
```

You can approve, reject, or edit the plan. Once approved, BonsAI executes each step and reports results.

### Approving / Rejecting
- **Approve** – BonsAI proceeds immediately.
- **Reject** – BonsAI cancels and asks what you'd prefer.
- **Edit** – you modify the plan in-place before approving.

---

## System Prompt & BONSAI.md

BonsAI's personality and knowledge are shaped by its system prompt, which is built from two sources:

### 1 · `BONSAI.md`
A markdown file in your workspace root. BonsAI reads it at the start of every conversation. Use it to tell BonsAI:
- Your preferred coding style
- Project-specific context (e.g., "we use Axum not Actix")
- Personas ("always respond as a senior Rust engineer")
- Off-limit actions ("never delete files without asking")

Example `BONSAI.md`:
```markdown
# My Project

This is a Tauri + Svelte app. The backend is Rust 2024 edition.

## Rules
- Always use `anyhow::Result` for error handling in Rust.
- Prefer functional Svelte stores over imperative DOM.
- Do not suggest rewriting existing components unless asked.

## Style
- Short, direct answers.
- Use British English spelling.
```

### 2 · Active Knowledge Modules
Any loaded knowledge module injects its top-k most relevant passages before each response. This gives BonsAI context-aware knowledge without needing a fine-tuned model.

### Editing the System Prompt
Settings → Assistant → System Prompt lets you see and edit the live prompt. Changes take effect on the next message.

---

## Undercover Mode

Undercover Mode removes all references to "Bonsai", "BonsAI", and internal tool names from the system prompt, activity logs, and git commits. This is useful when:

- Sharing screen captures publicly
- Contributing to open-source projects where you don't want to expose your toolchain
- Demonstrating to clients who have NDA restrictions

Enable it in **Settings → Privacy → Undercover Mode**. When active, a 🕵 icon appears in the status bar.

---

*← [Core IDE](02-CORE-IDE.md) · [Model Trainer →](04-MODEL-TRAINER.md)*
