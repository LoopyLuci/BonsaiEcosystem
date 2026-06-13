# Mobile Remote Desktop API Reference

Complete API documentation with code examples and integration patterns.

## Table of Contents
1. [Authentication](#authentication)
2. [Session Management](#session-management)
3. [Input Controls](#input-controls)
4. [Screen Capture](#screen-capture)
5. [Statistics](#statistics)
6. [Peer Discovery](#peer-discovery)
7. [Error Handling](#error-handling)
8. [Integration Examples](#integration-examples)

---

## Authentication

### Capability Token Structure

All requests require a valid capability token in the `Authorization` header.

```
Authorization: Bearer <JWT_TOKEN>
```

#### Token Format (JWT)

```json
{
  "iss": "bonsai-brdf",
  "sub": "device-uuid",
  "aud": "desktop-peer-uuid",
  "scope": [
    "screen:view",
    "input:touch",
    "input:text",
    "clipboard:read",
    "clipboard:write",
    "file:transfer"
  ],
  "exp": 1719864000,
  "iat": 1719777600,
  "nbf": 1719777600,
  "iss_cert": "https://brdf.bonsai.local/ca/cert.pem"
}
```

#### Token Verification

```rust
use chrono::Utc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub fn verify_capability_token(token: &str, peer_id: &str) -> Result<CapabilityToken> {
    let key = DecodingKey::from_rsa_pem(BRDF_CA_CERT.as_bytes())?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[peer_id]);
    
    let token_data = decode::<CapabilityToken>(token, &key, &validation)?;
    
    // Verify not expired
    let now = Utc::now().timestamp() as u64;
    if token_data.claims.exp < now {
        return Err("Token expired".into());
    }
    
    // Verify nbf (not before)
    if token_data.claims.nbf > now {
        return Err("Token not yet valid".into());
    }
    
    Ok(token_data.claims)
}
```

---

## Session Management

### Create Remote Session

Start a new remote desktop session with a specific peer.

```
POST /api/remote-desktop/session/start
Content-Type: application/json
Authorization: Bearer <TOKEN>

{
  "peer_id": "550e8400-e29b-41d4-a716-446655440000",
  "encryption_enabled": true,
  "preferred_connection": "local"
}
```

#### Response (Success - 200)

```json
{
  "session_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
  "peer_id": "550e8400-e29b-41d4-a716-446655440000",
  "started_at": "2024-06-30T10:30:45Z",
  "status": "connecting",
  "connection_type": "local",
  "encryption_enabled": true
}
```

#### Response (Error - 400)

```json
{
  "error": "peer_not_found",
  "message": "Peer with ID 550e8400... is not available",
  "suggestion": "Try :remote list to see available peers"
}
```

#### Code Example (Rust)

```rust
use reqwest::Client;
use serde_json::json;

async fn create_session(
    client: &Client,
    peer_id: &str,
    token: &str,
) -> Result<SessionState> {
    let response = client
        .post("http://localhost:8000/api/remote-desktop/session/start")
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "peer_id": peer_id,
            "encryption_enabled": true,
            "preferred_connection": "local"
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.json::<ErrorResponse>().await?;
        return Err(anyhow::anyhow!("Session start failed: {}", error.message));
    }

    let session = response.json::<SessionState>().await?;
    Ok(session)
}
```

#### Code Example (Python)

```python
import requests
import json

def create_session(peer_id: str, token: str) -> dict:
    """Create a new remote session with the given peer."""
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json"
    }
    payload = {
        "peer_id": peer_id,
        "encryption_enabled": True,
        "preferred_connection": "local"
    }
    
    response = requests.post(
        "http://localhost:8000/api/remote-desktop/session/start",
        headers=headers,
        json=payload
    )
    response.raise_for_status()
    return response.json()

# Usage
try:
    session = create_session("550e8400-...", "your-token")
    print(f"Session ID: {session['session_id']}")
    print(f"Status: {session['status']}")
except requests.HTTPError as e:
    print(f"Failed to create session: {e.response.json()['message']}")
```

#### Code Example (JavaScript)

```javascript
async function createSession(peerId, token) {
  const response = await fetch(
    'http://localhost:8000/api/remote-desktop/session/start',
    {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        peer_id: peerId,
        encryption_enabled: true,
        preferred_connection: 'local'
      })
    }
  );

  if (!response.ok) {
    const error = await response.json();
    throw new Error(`Session start failed: ${error.message}`);
  }

  return response.json();
}

// Usage
try {
  const session = await createSession('550e8400-...', 'your-token');
  console.log(`Session ID: ${session.session_id}`);
} catch (error) {
  console.error(error.message);
}
```

### Get Session State

Retrieve the current state of an active session.

```
GET /api/remote-desktop/session/{session_id}
Authorization: Bearer <TOKEN>
```

#### Response (Success - 200)

```json
{
  "session_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
  "peer_id": "550e8400-e29b-41d4-a716-446655440000",
  "started_at": "2024-06-30T10:30:45Z",
  "status": "streaming",
  "connection_type": "local",
  "encryption_enabled": true
}
```

### List Active Sessions

Get all currently active remote sessions.

```
GET /api/remote-desktop/sessions
Authorization: Bearer <TOKEN>
```

#### Response (Success - 200)

```json
{
  "sessions": [
    {
      "session_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
      "peer_id": "550e8400-e29b-41d4-a716-446655440000",
      "started_at": "2024-06-30T10:30:45Z",
      "status": "streaming",
      "connection_type": "local",
      "encryption_enabled": true
    }
  ],
  "total_count": 1
}
```

### Stop Remote Session

Gracefully terminate an active session.

```
POST /api/remote-desktop/session/{session_id}/stop
Authorization: Bearer <TOKEN>
```

#### Response (Success - 200)

```json
{
  "session_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
  "status": "disconnected",
  "duration_seconds": 3600,
  "frames_decoded": 216000,
  "frames_dropped": 45
}
```

---

## Input Controls

### Inject Text Input

Send text to be typed into the remote desktop's active field.

```
POST /api/remote-desktop/session/{session_id}/inject-text
Content-Type: application/json
Authorization: Bearer <TOKEN>

{
  "text": "Hello, World!",
  "target_field": null
}
```

#### Response (Success - 200)

```json
{
  "injected": true,
  "characters": 13,
  "latency_ms": 2.3
}
```

#### Response (Error - 413)

```json
{
  "error": "payload_too_large",
  "message": "Text exceeds maximum length of 10000 characters",
  "current_length": 12500
}
```

#### Code Example (Voice Dictation)

```python
import speech_recognition as sr

def voice_to_remote(session_id: str, token: str):
    """Transcribe speech and send to remote desktop."""
    recognizer = sr.Recognizer()
    
    with sr.Microphone() as source:
        print("Listening...")
        audio = recognizer.listen(source)
    
    try:
        text = recognizer.recognize_google(audio)
        print(f"Transcribed: {text}")
        
        response = requests.post(
            f"http://localhost:8000/api/remote-desktop/session/{session_id}/inject-text",
            headers={"Authorization": f"Bearer {token}"},
            json={"text": text}
        )
        response.raise_for_status()
        print("Text injected successfully")
    except sr.UnknownValueError:
        print("Could not understand audio")
    except sr.RequestError as e:
        print(f"API error: {e}")
```

### Inject Touch Input

Send touch/tap events to the remote desktop.

```
POST /api/remote-desktop/session/{session_id}/inject-touch
Content-Type: application/json
Authorization: Bearer <TOKEN>

{
  "x": 500,
  "y": 1200,
  "action": "DOWN",
  "pointer_id": 0
}
```

#### Parameters

- **x** (number, required): X coordinate (0 to screen width)
- **y** (number, required): Y coordinate (0 to screen height)
- **action** (string, required): One of: "DOWN", "MOVE", "UP"
- **pointer_id** (number, optional): Multi-touch pointer ID (0-9)

#### Response (Success - 200)

```json
{
  "injected": true,
  "action": "DOWN",
  "coordinates": {"x": 500, "y": 1200},
  "latency_ms": 1.8
}
```

#### Code Example (Multi-Touch Gesture)

```python
async def pinch_zoom(session_id: str, token: str, center_x: int, center_y: int):
    """Simulate pinch-zoom gesture on remote desktop."""
    client = requests.Session()
    client.headers.update({"Authorization": f"Bearer {token}"})
    
    base_url = f"http://localhost:8000/api/remote-desktop/session/{session_id}"
    
    # Finger 1: start
    client.post(f"{base_url}/inject-touch", json={
        "x": center_x - 50, "y": center_y,
        "action": "DOWN", "pointer_id": 0
    })
    
    # Finger 2: start
    client.post(f"{base_url}/inject-touch", json={
        "x": center_x + 50, "y": center_y,
        "action": "DOWN", "pointer_id": 1
    })
    
    # Both fingers: move inward (pinch)
    for i in range(1, 10):
        offset = 50 - (i * 5)
        
        client.post(f"{base_url}/inject-touch", json={
            "x": center_x - offset, "y": center_y,
            "action": "MOVE", "pointer_id": 0
        })
        
        client.post(f"{base_url}/inject-touch", json={
            "x": center_x + offset, "y": center_y,
            "action": "MOVE", "pointer_id": 1
        })
    
    # Both fingers: up
    client.post(f"{base_url}/inject-touch", json={
        "x": center_x - 5, "y": center_y,
        "action": "UP", "pointer_id": 0
    })
    
    client.post(f"{base_url}/inject-touch", json={
        "x": center_x + 5, "y": center_y,
        "action": "UP", "pointer_id": 1
    })
    
    print("Pinch-zoom gesture sent")
```

---

## Screen Capture

### Take Screenshot

Capture the current remote desktop screen as base64-encoded image.

```
POST /api/remote-desktop/session/{session_id}/screenshot
Content-Type: application/json
Authorization: Bearer <TOKEN>

{
  "quality": 85,
  "format": "jpeg"
}
```

#### Parameters

- **quality** (number, 0-100, optional): JPEG compression quality. Default: 85
- **format** (string, optional): "jpeg" or "png". Default: "jpeg"

#### Response (Success - 200)

```json
{
  "session_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
  "timestamp": "2024-06-30T10:35:22Z",
  "width": 1080,
  "height": 2340,
  "format": "jpeg",
  "quality": 85,
  "size_bytes": 387400,
  "image_base64": "iVBORw0KGgoAAAA...AAAAAAAAAAAAAAA="
}
```

#### Code Example (Save Screenshot)

```python
import base64
from pathlib import Path

def save_screenshot(session_id: str, token: str, output_file: str):
    """Capture and save screenshot from remote desktop."""
    response = requests.post(
        f"http://localhost:8000/api/remote-desktop/session/{session_id}/screenshot",
        headers={"Authorization": f"Bearer {token}"},
        json={"quality": 90, "format": "jpeg"}
    )
    response.raise_for_status()
    
    data = response.json()
    image_bytes = base64.b64decode(data['image_base64'])
    
    Path(output_file).write_bytes(image_bytes)
    print(f"Screenshot saved to {output_file} ({data['size_bytes']} bytes)")

# Usage
save_screenshot(session_id, token, "desktop-screenshot.jpg")
```

#### Code Example (Continuous Screenshot Stream)

```python
import asyncio
import time

async def screenshot_stream(session_id: str, token: str, interval_ms: float = 500):
    """Stream screenshots from remote desktop at specified interval."""
    while True:
        try:
            response = requests.post(
                f"http://localhost:8000/api/remote-desktop/session/{session_id}/screenshot",
                headers={"Authorization": f"Bearer {token}"},
                json={"quality": 75, "format": "jpeg"}
            )
            response.raise_for_status()
            
            data = response.json()
            yield data['image_base64'], data['timestamp']
            
            await asyncio.sleep(interval_ms / 1000.0)
        except requests.RequestException as e:
            print(f"Screenshot error: {e}")
            break

# Usage
async def main():
    async for image_base64, timestamp in screenshot_stream(session_id, token):
        # Process or display image
        print(f"Frame at {timestamp}")

asyncio.run(main())
```

---

## Statistics

### Get Session Statistics

Retrieve real-time performance metrics for the active session.

```
GET /api/remote-desktop/session/{session_id}/stats
Authorization: Bearer <TOKEN>
```

#### Response (Success - 200)

```json
{
  "session_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
  "timestamp": "2024-06-30T10:35:22Z",
  "fps": 59.8,
  "bitrate_mbps": 8.4,
  "latency_ms": 2.3,
  "bandwidth_usage_mb": 42.5,
  "frames_decoded": 3600,
  "frames_dropped": 2,
  "connection_uptime_secs": 300,
  "battery_drain_percent_per_hour": 10.2
}
```

#### Code Example (Performance Monitor)

```rust
use tokio::time::{interval, Duration};
use reqwest::Client;

async fn monitor_performance(
    client: &Client,
    session_id: &str,
    token: &str,
) -> Result<()> {
    let mut ticker = interval(Duration::from_secs(5));
    let mut prev_bytes = 0.0;
    let mut prev_frames = 0u64;
    
    loop {
        ticker.tick().await;
        
        let response = client
            .get(&format!(
                "http://localhost:8000/api/remote-desktop/session/{}/stats",
                session_id
            ))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        
        let stats: SessionStats = response.json().await?;
        
        // Calculate bandwidth change
        let bandwidth_delta = stats.bandwidth_usage_mb - prev_bytes;
        let frames_delta = stats.frames_decoded - prev_frames;
        
        // Alert on degradation
        if stats.fps < 50.0 {
            eprintln!("⚠️ FPS dropped to {}", stats.fps);
        }
        if stats.latency_ms > 30.0 {
            eprintln!("⚠️ Latency increased to {}ms", stats.latency_ms);
        }
        if stats.frames_dropped > 10 {
            eprintln!("⚠️ Frame drops detected: {}", stats.frames_dropped);
        }
        
        println!(
            "FPS: {:.1} | Bitrate: {:.1} Mbps | Latency: {:.1}ms | BW: {:.1}MB",
            stats.fps, stats.bitrate_mbps, stats.latency_ms, bandwidth_delta
        );
        
        prev_bytes = stats.bandwidth_usage_mb;
        prev_frames = stats.frames_decoded;
    }
}
```

---

## Peer Discovery

### List Available Peers

Get list of all available desktop peers for connection.

```
GET /api/remote-desktop/peers
Authorization: Bearer <TOKEN>
```

#### Query Parameters

- **status** (string, optional): Filter by status: "online", "offline", "pairing", or "all". Default: "all"
- **trusted_only** (boolean, optional): Only return trusted peers. Default: false

#### Response (Success - 200)

```json
{
  "peers": [
    {
      "peer_id": "550e8400-e29b-41d4-a716-446655440000",
      "device_name": "Desktop #1",
      "device_model": "Windows 10 Professional",
      "last_seen": "2024-06-30T10:35:22Z",
      "status": "online",
      "local_ip": "192.168.1.100",
      "is_trusted": true
    },
    {
      "peer_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
      "device_name": "Work Laptop",
      "device_model": "macOS 14.5",
      "last_seen": "2024-06-30T08:00:00Z",
      "status": "offline",
      "local_ip": null,
      "is_trusted": true
    }
  ],
  "total_count": 2,
  "online_count": 1
}
```

#### Code Example (Peer Selection Dialog)

```python
def select_peer(token: str) -> str:
    """Display interactive peer selection dialog."""
    response = requests.get(
        "http://localhost:8000/api/remote-desktop/peers",
        headers={"Authorization": f"Bearer {token}"},
        params={"status": "online"}
    )
    response.raise_for_status()
    
    peers = response.json()['peers']
    
    if not peers:
        print("No online peers available")
        return None
    
    print("\nAvailable Peers:")
    for i, peer in enumerate(peers, 1):
        status_icon = "🟢" if peer['status'] == 'online' else "🔴"
        print(f"{i}. {status_icon} {peer['device_name']} ({peer['device_model']})")
        if peer['local_ip']:
            print(f"   IP: {peer['local_ip']}")
        print(f"   Last seen: {peer['last_seen']}")
    
    choice = int(input("\nSelect peer (number): ")) - 1
    return peers[choice]['peer_id']

# Usage
peer_id = select_peer(token)
session = create_session(peer_id, token)
```

---

## Error Handling

### Standard Error Response

All errors follow this format:

```json
{
  "error": "error_code",
  "message": "Human-readable message",
  "details": {
    "field_name": "additional context"
  }
}
```

### Common Error Codes

| Code | HTTP Status | Meaning | Retry? |
|------|-------------|---------|--------|
| `unauthorized` | 401 | Invalid or expired token | No |
| `peer_not_found` | 404 | Requested peer does not exist | No |
| `session_not_found` | 404 | Requested session does not exist | No |
| `session_disconnected` | 409 | Session is no longer active | No |
| `payload_too_large` | 413 | Request body exceeds limits | No |
| `rate_limit_exceeded` | 429 | Too many requests | Yes (exponential backoff) |
| `internal_error` | 500 | Server-side error | Yes (exponential backoff) |

### Error Handling Pattern

```python
import time
from requests.exceptions import RequestException

def call_with_retry(
    method: str,
    url: str,
    token: str,
    max_retries: int = 3,
    **kwargs
) -> dict:
    """Make API call with automatic retry on transient errors."""
    
    for attempt in range(max_retries):
        try:
            response = requests.request(
                method,
                url,
                headers={"Authorization": f"Bearer {token}"},
                **kwargs
            )
            
            if response.status_code == 429:  # Rate limit
                wait_time = int(response.headers.get('Retry-After', 2 ** attempt))
                print(f"Rate limited. Waiting {wait_time}s...")
                time.sleep(wait_time)
                continue
            
            response.raise_for_status()
            return response.json()
            
        except RequestException as e:
            if attempt == max_retries - 1:
                raise
            
            wait_time = 2 ** attempt  # Exponential backoff
            print(f"Attempt {attempt + 1} failed: {e}. Retrying in {wait_time}s...")
            time.sleep(wait_time)

# Usage
try:
    result = call_with_retry(
        "POST",
        f"http://localhost:8000/api/remote-desktop/session/{session_id}/screenshot",
        token,
        json={"quality": 85}
    )
except Exception as e:
    print(f"Failed after retries: {e}")
```

---

## Integration Examples

### Example 1: Web-Based Remote Desktop Dashboard

```html
<!DOCTYPE html>
<html>
<head>
    <title>Bonsai Remote Desktop</title>
    <style>
        body { font-family: Arial; margin: 20px; }
        .session-card { border: 1px solid #ccc; padding: 15px; margin: 10px 0; }
        .peer-list { display: grid; gap: 10px; }
        .peer-item { background: #f5f5f5; padding: 10px; border-radius: 5px; }
        .status { display: inline-block; width: 10px; height: 10px; border-radius: 50%; }
        .status.online { background: #4caf50; }
        .status.offline { background: #f44336; }
        canvas { border: 1px solid #ccc; max-width: 100%; }
    </style>
</head>
<body>
    <h1>Bonsai Remote Desktop</h1>
    
    <div id="peers" class="peer-list"></div>
    
    <div id="session" class="session-card" style="display: none;">
        <h2>Active Session</h2>
        <canvas id="screen" width="1080" height="2340"></canvas>
        <div id="stats"></div>
    </div>

    <script>
        const TOKEN = 'your-token-here';
        const API_URL = 'http://localhost:8000/api/remote-desktop';
        
        async function listPeers() {
            const res = await fetch(`${API_URL}/peers`, {
                headers: { 'Authorization': `Bearer ${TOKEN}` }
            });
            const data = await res.json();
            
            const container = document.getElementById('peers');
            container.innerHTML = data.peers.map(peer => `
                <div class="peer-item">
                    <span class="status ${peer.status}"></span>
                    ${peer.device_name} (${peer.device_model})
                    <button onclick="connectPeer('${peer.peer_id}')">Connect</button>
                </div>
            `).join('');
        }
        
        async function connectPeer(peerId) {
            const res = await fetch(`${API_URL}/session/start`, {
                method: 'POST',
                headers: {
                    'Authorization': `Bearer ${TOKEN}`,
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    peer_id: peerId,
                    encryption_enabled: true
                })
            });
            
            const session = await res.json();
            document.getElementById('session').style.display = 'block';
            streamSession(session.session_id);
        }
        
        async function streamSession(sessionId) {
            const canvas = document.getElementById('screen');
            const ctx = canvas.getContext('2d');
            
            while (true) {
                try {
                    const res = await fetch(`${API_URL}/session/${sessionId}/screenshot`, {
                        method: 'POST',
                        headers: {
                            'Authorization': `Bearer ${TOKEN}`,
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify({ quality: 75 })
                    });
                    
                    const data = await res.json();
                    const img = new Image();
                    img.onload = () => ctx.drawImage(img, 0, 0);
                    img.src = `data:image/jpeg;base64,${data.image_base64}`;
                    
                    updateStats(sessionId);
                    
                    await new Promise(resolve => setTimeout(resolve, 100));
                } catch (e) {
                    console.error('Stream error:', e);
                    break;
                }
            }
        }
        
        async function updateStats(sessionId) {
            const res = await fetch(`${API_URL}/session/${sessionId}/stats`, {
                headers: { 'Authorization': `Bearer ${TOKEN}` }
            });
            const stats = await res.json();
            
            document.getElementById('stats').innerHTML = `
                FPS: ${stats.fps.toFixed(1)} | 
                Bitrate: ${stats.bitrate_mbps.toFixed(1)} Mbps | 
                Latency: ${stats.latency_ms.toFixed(1)}ms | 
                Battery Drain: ${stats.battery_drain_percent_per_hour.toFixed(1)}%/h
            `;
        }
        
        // Canvas click for touch input
        document.getElementById('screen').addEventListener('click', async (e) => {
            const canvas = document.getElementById('screen');
            const rect = canvas.getBoundingClientRect();
            const x = Math.round((e.clientX - rect.left) * (canvas.width / rect.width));
            const y = Math.round((e.clientY - rect.top) * (canvas.height / rect.height));
            
            const sessionId = 'active-session-id'; // Get from state
            await fetch(`${API_URL}/session/${sessionId}/inject-touch`, {
                method: 'POST',
                headers: {
                    'Authorization': `Bearer ${TOKEN}`,
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ x, y, action: 'DOWN' })
            });
        });
        
        listPeers();
    </script>
</body>
</html>
```

---

**Last Updated**: 2024-06-30
**API Version**: 1.0
