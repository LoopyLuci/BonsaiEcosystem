# 🌐 BONSAI WEB INTELLIGENCE FABRIC (BWIF)
## Complete Production Specification

**Status**: 🟢 PRODUCTION-READY DESIGN  
**Type**: Distributed web browser + AI scraper  
**Core Stack**: Tauri 2 (Rust backend) + Svelte frontend  
**Integration**: Full Bonsai Ecosystem (Echo, TransferDaemon, Compute Fabric, Universe)  

---

## 1. SYSTEM OVERVIEW

BWIF is a **sovereign, AI-augmented, peer-to-peer web intelligence system** that combines:

- **Bonsai Browser**: Tauri-based desktop browser with AI co-pilot, fingerprint rotation, ad blocking
- **Distributed Scraper**: Echo-backed distributed job system for high-throughput web extraction
- **AI Extraction Engine**: BonsAI V2-powered intelligent data extraction that adapts to layout changes
- **Anti-Detection**: TransferDaemon proxy rotation, fingerprint spoofing, behavioral mimicry
- **Full Observability**: Universe integration for complete audit trail and time-travel debugging

**Why it matters**: Replaces Chrome (limited), Playwright (fragile), and Scrapy (isolated) with an integrated system that leverages the entire Bonsai Ecosystem.

---

## 2. ARCHITECTURE

```
BONSAI BROWSER (Desktop)
│
├─ WebView (system native - WebView2/WebKit)
│  ├─ Tab management
│  ├─ Network monitoring
│  ├─ DevTools integration
│  └─ Screenshot capture
│
├─ AI Co-Pilot (sidebar Svelte component)
│  ├─ Natural language page analysis
│  ├─ Form filling
│  ├─ Data extraction
│  └─ Navigation planning
│
├─ Privacy Suite
│  ├─ DNS-level ad blocker
│  ├─ Tracker blocker
│  ├─ Fingerprint randomizer
│  └─ Permission manager
│
└─ Session Recorder
   ├─ Action recording
   ├─ Script generation
   └─ Playback engine

DISTRIBUTED SCRAPER (Headless)
│
├─ Orchestrator
│  ├─ Job splitting (by domain)
│  ├─ Worker discovery (via Echo)
│  ├─ Progress tracking
│  ├─ Result merging
│  └─ CAS storage
│
├─ Worker Nodes (in Sanctum vaults)
│  ├─ Chromium instance (via Playwright)
│  ├─ Fingerprint profile (unique per worker)
│  ├─ Proxy rotation (TransferDaemon)
│  ├─ AI extractor (BonsAI V2)
│  └─ Fallback selector engine
│
└─ Infrastructure
   ├─ Echo (job distribution)
   ├─ TransferDaemon (proxy pool)
   ├─ Compute Fabric (GPU extraction)
   ├─ Universe (observability)
   ├─ Sanctum (worker isolation)
   └─ CAS (page storage)
```

---

## 3. BROWSER COMPONENT

### 3.1 Core Browser (Tauri)

**Architecture**: Tauri 2 app with system WebView backend

```rust
// src-tauri/src/main.rs
#[tauri::command]
async fn navigate(url: String) -> Result<String, String> {
    // Navigate WebView to URL, capture navigation time
    universe::emit_event("browser_navigate", json!({
        "url": url,
        "timestamp": now(),
    })).await;
    Ok("success".into())
}

#[tauri::command]
async fn screenshot() -> Result<Vec<u8>, String> {
    // Capture current viewport as PNG
}

#[tauri::command]
async fn extract_page_data(query: String) -> Result<serde_json::Value, String> {
    // Send page DOM + query to BonsAI V2
    // Return structured extraction
}

#[tauri::command]
async fn get_page_metadata() -> Result<PageMetadata, String> {
    // Title, description, links, forms, scripts
}
```

### 3.2 AI Co-Pilot (Svelte Component)

```svelte
<!-- src/lib/CoPI lot.svelte -->
<script>
  let query = "";
  let extractionResult = null;
  let loading = false;

  async function extractData() {
    loading = true;
    extractionResult = await invoke("extract_page_data", { query });
    loading = false;
  }

  async function fillForm(selector, value) {
    await invoke("browser_fill", { selector, value });
  }

  async function navigateTo(url) {
    await invoke("browser_navigate", { url });
  }
</script>

<div class="copilot">
  <h3>AI Co-Pilot</h3>
  
  <input bind:value={query} placeholder="What do you want to extract?" />
  <button on:click={extractData} disabled={loading}>
    {loading ? "Extracting..." : "Extract"}
  </button>

  {#if extractionResult}
    <pre>{JSON.stringify(extractionResult, null, 2)}</pre>
  {/if}

  <details>
    <summary>Page Analysis</summary>
    <!-- Forms, links, images, etc. -->
  </details>
</div>

<style>
  .copilot { padding: 20px; border-left: 1px solid #ccc; }
</style>
```

### 3.3 Privacy Suite

**Ad Blocker** (DNS + Cosmetic):
```rust
// src-tauri/src/privacy.rs
pub struct AdBlocker {
    dns_blocklist: HashSet<String>,
    cosmetic_rules: Vec<CosmeticRule>,
}

impl AdBlocker {
    pub fn filter_request(&self, url: &str) -> bool {
        // Check DNS blocklist (EasyList, AdGuard)
        // Return false to block
    }

    pub fn inject_cosmetic_css(&self) -> String {
        // Return CSS to hide ad elements
        "display: none !important".into()
    }
}
```

**Fingerprint Rotation**:
```rust
pub struct FingerprintProfile {
    pub user_agent: String,
    pub screen_resolution: (u32, u32),
    pub timezone: String,
    pub language: String,
    pub fonts: Vec<String>,
    pub gpu: String,
    pub webgl_vendor: String,
}

impl FingerprintProfile {
    pub fn random() -> Self {
        // Generate realistic profile from pool
        Self {
            user_agent: REALISTIC_USER_AGENTS[rand::random::<usize>() % REALISTIC_USER_AGENTS.len()].into(),
            screen_resolution: SCREEN_SIZES[rand::random()],
            timezone: TIMEZONES[rand::random()],
            // ...
        }
    }

    pub fn inject_javascript(&self) -> String {
        // JavaScript to spoof fingerprint at runtime
        format!(
            r#"
            Object.defineProperty(navigator, 'userAgent', {{ value: '{}' }});
            Object.defineProperty(screen, 'width', {{ value: {} }});
            Object.defineProperty(screen, 'height', {{ value: {} }});
            "#,
            self.user_agent, self.screen_resolution.0, self.screen_resolution.1
        )
    }
}
```

**Session Recorder**:
```rust
pub struct SessionRecorder {
    actions: Vec<BrowserAction>,
}

pub enum BrowserAction {
    Navigate { url: String, timestamp: i64 },
    Click { selector: String, timestamp: i64 },
    Fill { selector: String, value: String, timestamp: i64 },
    Wait { duration_ms: u64, timestamp: i64 },
    Extract { query: String, result: serde_json::Value, timestamp: i64 },
}

impl SessionRecorder {
    pub fn to_python_script(&self) -> String {
        // Export as Playwright script
        r#"
from playwright.async_api import async_playwright

async def run():
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page()
        "#.into()
        // ... append each action
    }

    pub fn to_sylva_script(&self) -> String {
        // Export as Bonsai Sylva script
        // (Sylva is Bonsai's native scripting language)
    }
}
```

---

## 4. DISTRIBUTED SCRAPER COMPONENT

### 4.1 Orchestrator

```rust
// src/orchestrator.rs
pub struct ScrapingOrchestrator {
    echo: EchoClient,
    cas: CasClient,
    universe: UniverseClient,
}

#[derive(Serialize)]
pub struct ScrapingJob {
    pub id: String,
    pub urls: Vec<String>,
    pub extraction_schema: String,  // Natural language or JSON schema
    pub rate_limit_per_domain: u32,
    pub output_format: OutputFormat,
}

impl ScrapingOrchestrator {
    pub async fn submit_job(&self, job: ScrapingJob) -> Result<String> {
        universe::emit("scraping_job_submitted", &job).await;

        // Split URLs by domain
        let batches = self.split_by_domain(&job.urls, job.rate_limit_per_domain);

        // Broadcast to workers via Echo
        for (domain, urls) in batches {
            let batch = BatchJob {
                parent_job_id: job.id.clone(),
                domain,
                urls,
                extraction_schema: job.extraction_schema.clone(),
            };
            echo.publish("scraper.jobs", &batch).await?;
        }

        Ok(job.id)
    }

    pub async fn get_results(&self, job_id: &str) -> Result<ScrapeResults> {
        // Wait for all workers to complete
        // Merge results
        // Store in CAS
    }

    fn split_by_domain(&self, urls: &[String], rate_limit: u32) -> HashMap<String, Vec<String>> {
        let mut by_domain: HashMap<String, Vec<String>> = HashMap::new();
        for url in urls {
            let domain = extract_domain(url);
            by_domain.entry(domain).or_insert_with(Vec::new).push(url.clone());
        }
        by_domain
    }
}
```

### 4.2 Worker Node

```rust
// src/worker.rs
pub struct ScraperWorker {
    id: String,
    fingerprint: FingerprintProfile,
    proxy_pool: TransferDaemonProxy,
    browser: PlaywrightBrowser,
    ai_extractor: BonsAIExtractor,
}

impl ScraperWorker {
    pub async fn process_batch(&self, batch: BatchJob) -> Result<Vec<ExtractionResult>> {
        let mut results = Vec::new();
        for url in batch.urls {
            universe::emit("worker_processing", json!({
                "worker_id": self.id,
                "url": url,
            })).await;

            // Get proxy from pool
            let proxy = self.proxy_pool.get_next().await?;

            // Navigate with fingerprint spoofing
            let page = self.browser.new_page().await?;
            page.context().add_init_script(&self.fingerprint.inject_javascript()).await?;
            page.goto(&url).await?;

            // Wait for page load
            page.wait_for_load_state("networkidle").await?;

            // Capture page DOM
            let dom = page.content().await?;
            let screenshot = page.screenshot().await?;

            // Extract via AI or fallback
            let result = self.extract_data(&dom, &batch.extraction_schema, &screenshot).await?;
            results.push(result);

            page.close().await?;
        }
        Ok(results)
    }

    async fn extract_data(
        &self,
        dom: &str,
        query: &str,
        screenshot: &[u8],
    ) -> Result<ExtractionResult> {
        // Try AI first
        match self.ai_extractor.extract(dom, query).await {
            Ok(data) => Ok(ExtractionResult {
                data,
                method: "ai",
                confidence: 0.85,
            }),
            Err(_) => {
                // Fallback to rule-based
                let data = self.fallback_extract(dom)?;
                Ok(ExtractionResult {
                    data,
                    method: "fallback",
                    confidence: 0.60,
                })
            }
        }
    }

    fn fallback_extract(&self, dom: &str) -> Result<serde_json::Value> {
        // Use pre-trained selectors or heuristics
        // Try common patterns: h1, title, meta[name="description"], etc.
        Ok(json!({
            "title": extract_meta(dom, "title"),
            "description": extract_meta(dom, "description"),
        }))
    }
}
```

### 4.3 AI Extractor

```rust
// src/extractor.rs
pub struct BonsAIExtractor {
    client: BonsAIV2Client,
    kdb: KDBClient,
}

impl BonsAIExtractor {
    pub async fn extract(&self, dom: &str, query: &str) -> Result<serde_json::Value> {
        // Check KDB for cached pattern
        let domain = extract_domain_from_dom(dom);
        if let Ok(pattern) = self.kdb.get_pattern(&domain).await {
            // Use cached pattern (CSS selector or XPath)
            return self.apply_pattern(dom, &pattern).await;
        }

        // Otherwise, ask BonsAI V2
        let prompt = format!(
            "Extract structured data from this HTML using this query.\nQuery: {}\nHTML:\n{}",
            query, dom
        );

        let response = self.client.query(&prompt).await?;
        
        // Parse response as JSON
        let extracted = serde_json::from_str(&response)?;

        // Cache pattern for future use
        self.kdb.cache_pattern(&domain, &response).await.ok();

        Ok(extracted)
    }

    async fn apply_pattern(&self, dom: &str, pattern: &str) -> Result<serde_json::Value> {
        // Use selector library to extract
        // e.g., cssselect2, scraper crates
        Ok(json!({}))
    }
}
```

---

## 5. INTEGRATION WITH BONSAI ECOSYSTEM

| Component | Integration |
|-----------|-------------|
| **Echo** | Job distribution, worker discovery, result aggregation |
| **TransferDaemon** | Proxy rotation, encrypted proxy sharing |
| **Compute Fabric** | Offload AI extraction to GPU nodes |
| **Sanctum** | Worker isolation, sandboxing |
| **Universe** | Every click, navigation, extraction is logged |
| **Survival System** | Auto-restart workers, retry failed batches |
| **CAS** | Store page snapshots, screenshots, extracted data |
| **KDB** | Cache extraction patterns per domain/layout |
| **BonsAI V2** | Natural language extraction queries |
| **MCP Server** | Expose tools: `browser_navigate`, `scraper_submit_job`, `browser_extract` |
| **Credits** | Meter proxy usage, AI inference calls, CAS storage |

---

## 6. MCP TOOLS

```rust
#[tauri::command]
pub async fn browser_navigate(url: String) -> Result<()> { }

#[tauri::command]
pub async fn browser_click(selector: String) -> Result<()> { }

#[tauri::command]
pub async fn browser_fill(selector: String, value: String) -> Result<()> { }

#[tauri::command]
pub async fn browser_extract(query: String) -> Result<serde_json::Value> { }

#[tauri::command]
pub async fn browser_screenshot() -> Result<String> { }  // Returns base64 PNG

#[tauri::command]
pub async fn scraper_submit_job(
    urls: Vec<String>,
    schema: String,
    rate_limit: u32,
) -> Result<String> { }  // Returns job_id

#[tauri::command]
pub async fn scraper_get_results(job_id: String) -> Result<Vec<serde_json::Value>> { }

#[tauri::command]
pub async fn proxy_rotate() -> Result<String> { }  // Returns new IP
```

---

## 7. COMPARISON WITH EXISTING TOOLS

| Feature | Chrome | Firefox | Playwright | Scrapy | **BWIF** |
|---------|--------|---------|------------|--------|----------|
| **Distributed** | ❌ | ❌ | ❌ | ⚠️ Limited | ✅ Full (Echo) |
| **AI extraction** | ❌ | ❌ | ❌ | ❌ | ✅ BonsAI V2 |
| **Proxy rotation** | ❌ | ❌ | ❌ (plugins) | ⚠️ Limited | ✅ TransferDaemon |
| **Fingerprint rotation** | ❌ | ❌ | ❌ | ❌ | ✅ Automatic |
| **Built-in ad blocker** | ❌ | ✅ | ❌ | ❌ | ✅ DNS-level |
| **Observability** | Dev tools | Dev tools | Logs | Logs | ✅ Universe (time-travel) |
| **Script generation** | ❌ | ❌ | ❌ | ❌ | ✅ Session recorder |
| **AI co-pilot** | ❌ | ❌ | ❌ | ❌ | ✅ Sidebar |
| **Ecosystem integration** | ❌ | ❌ | ❌ | ❌ | ✅ Full (15+ systems) |

---

## 8. PERFORMANCE TARGETS

| Metric | Target |
|--------|--------|
| **Browser startup** | <1 second |
| **Page load (with blocker)** | 2x faster than Chrome |
| **Scraper throughput** | 10,000 pages/minute (100 workers) |
| **AI extraction latency** | <2s (GPU), <5s (CPU) |
| **Proxy rotation** | <50ms (TransferDaemon) |
| **Worker isolation overhead** | <10% (Sanctum) |

---

## 9. IMPLEMENTATION PHASES

| Phase | Deliverable |
|-------|-------------|
| **1** | Tauri browser shell with WebView integration |
| **2** | AI co-pilot (BonsAI V2 integration) |
| **3** | Session recorder & script export |
| **4** | Privacy suite (ad blocker, fingerprint rotation) |
| **5** | Distributed scraper orchestrator |
| **6** | Worker nodes (Playwright + AI extraction) |
| **7** | TransferDaemon proxy integration |
| **8** | Universe observability & Survival System hooks |
| **9** | Production hardening & load testing |

---

## CONCLUSION

BWIF is a **complete replacement for Chrome, Playwright, and Scrapy** — a sovereign, AI-augmented, distributed web intelligence system deeply integrated into the Bonsai Ecosystem. It combines the ease-of-use of a browser with the scalability of a distributed system, and the intelligence of AI-powered extraction.

With BWIF, web research, automation, and data extraction become first-class operations in the Bonsai Workspace. 🌐

