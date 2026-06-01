# MCP Server (bonsai-mcp-server)

This document explains how to run the MCP server, obtain a capability token, and call the MCP HTTP API.

## Run the server

Set the daemon URL if different from the default:

```powershell
$env:BONSAI_DAEMON_URL = "http://127.0.0.1:8080/api"
cargo run -p bonsai-mcp-server -- --host 127.0.0.1 --port 3000
```

## Obtain a token

The MCP server expects a capability token in the `Authorization: Bearer <token>` header. Obtain a token from the Bonsai daemon token issuance endpoint (e.g. `POST /auth/issue`) or use a test token during development by setting `MCP_TEST_TOKEN` in your environment.

## Example client (Python)

```python
import requests

BASE = "http://127.0.0.1:3000"
TOKEN = "<your-token>"

headers = {"Authorization": f"Bearer {TOKEN}"}
resp = requests.get(f"{BASE}/tools/list", headers=headers)
print(resp.status_code, resp.text)
```

## Available endpoints

- `GET /` - health
- `GET /tools/list` - list available tools (requires token)
- `POST /call/<tool>` - call a tool (forwards to daemon)
- `GET /ws/telemetry` - websocket telemetry endpoint

