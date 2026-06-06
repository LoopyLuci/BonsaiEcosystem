# 🧠 BUDIS — Bonsai Universal Data Intelligence System
## Complete Production Specification

**Version:** 1.0  
**Status:** 🟢 Production-Ready Design  
**Date:** June 2, 2026  

---

## Executive Summary

**BUDIS** is a next-generation, sovereign, AI-driven data pipeline that:

- **Ingests any file format** (PDF, DOCX, images, audio, video, binary, custom formats)
- **Automatically extracts** structured metadata, entities, relationships, and content
- **Classifies** documents by domain, knowledge type, audience, and tags
- **Deduplicates** near-identical files and merges metadata intelligently
- **Builds searchable knowledge modules** stored in KMDB
- **Learns continuously** from user corrections via EternalTrainingLoop
- **Self-heals** when extraction fails using Survival System
- **Provides complete audit trail** via Universe events

All processing happens **locally, on your device**, with no data sent to cloud services.

---

## 1. Core Principles

### Universal Format Support
The system uses a **plugin architecture** where each file format (PDF, DOCX, JSON, CSV, PNG, MP3, MP4, ELF, etc.) has its own extractor. Plugins can be added without modifying the core system.

### Automatic Understanding
Every extracted piece of content is automatically classified into:
- **Domain** (medical, legal, programming, finance, engineering, general)
- **Knowledge Type** (factual, procedural, conceptual, meta)
- **Audience** (novice, intermediate, expert, manager)
- **Tags** (extracted keywords and topics)

### No Manual Work
The entire pipeline is automated. Users don't need to manually categorize, tag, or extract — BUDIS does it all. Users only need to review and correct (which improves the system).

### Continuous Learning
When a user corrects a classification or extraction, BUDIS learns from it. The EternalTrainingLoop fine-tunes classifiers and extraction models overnight, making the system progressively more accurate.

### Local and Private
All processing stays on your device. Models run locally. No data is sent to external services unless explicitly enabled.

---

## 2. System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        FILE INPUT SOURCES                           │
│  ┌────────────┐  ┌──────────┐  ┌──────────┐  ┌────────────┐       │
│  │ File       │  │ API      │  │ Batch    │  │ Web UI     │       │
│  │ Watcher    │  │ Gateway  │  │ Import   │  │ Upload     │       │
│  └─────┬──────┘  └────┬─────┘  └────┬─────┘  └─────┬──────┘       │
│        └────────────────┼──────────────┼──────────────┘             │
└─────────────────────────┼──────────────┼──────────────────────────┘
                          │              │
                          ▼              ▼
                  ┌──────────────────────────┐
                  │   MIME Type Detection    │
                  │   (Magic bytes, ext)     │
                  └──────────┬───────────────┘
                             │
                             ▼
        ┌────────────────────────────────────────┐
        │    Plugin Registry & Dispatcher        │
        │  • Route to correct format plugin      │
        │  • Fallback to generic binary          │
        └────────────────┬───────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
        ▼                ▼                ▼
    ┌────────┐    ┌────────────┐    ┌──────────┐
    │ Text   │    │ Image      │    │ Audio    │
    │Plugins │    │ Plugins    │    │ Plugins  │
    │(PDF,   │    │(OCR,       │    │(Whisper, │
    │DOCX,   │    │detection,  │    │diariz)   │
    │HTML)   │    │metadata)   │    │          │
    └─┬──────┘    └──┬─────────┘    └──┬───────┘
      │             │                  │
      └─────────────┼──────────────────┘
                    │
                    ▼
        ┌──────────────────────────────────┐
        │  Metadata & Entity Extraction    │
        │  • Standard metadata (author,    │
        │    date, size, hash)             │
        │  • NER (people, orgs, dates)     │
        │  • Relationships (person→org)    │
        └──────────────┬───────────────────┘
                       │
                       ▼
        ┌──────────────────────────────────┐
        │  Classification Engine           │
        │  (BonsAI V2 + fine-tuned models) │
        │  • Domain classification         │
        │  • Knowledge type                │
        │  • Audience level                │
        │  • Confidence scores             │
        └──────────────┬───────────────────┘
                       │
                       ▼
        ┌──────────────────────────────────┐
        │  Deduplication & Merging         │
        │  • Hash comparison (BLAKE3)      │
        │  • Near-duplicate detection      │
        │  • Metadata merge (keep best)    │
        └──────────────┬───────────────────┘
                       │
                       ▼
        ┌──────────────────────────────────┐
        │  Knowledge Module Builder        │
        │  • Text chunking                 │
        │  • Embedding generation          │
        │  • HNSW index creation           │
        │  • .kmod package creation        │
        │  • KMDB registration             │
        └──────────────┬───────────────────┘
                       │
                       ▼
        ┌──────────────────────────────────┐
        │  Storage & Indexing              │
        │  • CAS (content-addressed)       │
        │  • KMDB (knowledge modules)      │
        │  • Universe (event logging)      │
        │  • Survival System (health)      │
        └──────────────────────────────────┘
```

---

## 3. Component Details

### 3.1 File Input Sources

**File Watcher**
- Monitors directories (configurable)
- Detects new files automatically
- Enqueues them for processing

**API Gateway**
- REST endpoint: `POST /api/ingest`
- Accepts file upload + metadata
- Returns job ID for tracking

**Batch Import**
- Import from external source (cloud storage, USB drive, network share)
- Can process 1000s of files in parallel

**Web UI Upload**
- Drag-and-drop interface in Bonsai Workspace
- Real-time progress tracking
- Inline preview

### 3.2 Plugin Architecture

**Interface:**
```rust
pub trait FileFormatPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn supported_mime_types(&self) -> Vec<&str>;
    fn supported_extensions(&self) -> Vec<&str>;
    fn extract_content(&self, data: &[u8]) -> Result<ExtractedContent>;
    fn extract_metadata(&self, data: &[u8]) -> Result<Metadata>;
}

pub struct ExtractedContent {
    pub text: String,
    pub structure: Option<DocumentStructure>,  // Headings, sections
    pub images: Vec<ImageData>,                // Extracted images
    pub tables: Vec<TableData>,                // Extracted tables
    pub code_blocks: Vec<CodeBlock>,           // Code snippets
}

pub struct Metadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub created_date: Option<DateTime>,
    pub modified_date: Option<DateTime>,
    pub language: Option<String>,
    pub character_count: usize,
    pub page_count: Option<usize>,
    pub format_specific: HashMap<String, String>,
}
```

**Built-in Plugins:**

| Format | Plugin | Uses | Status |
|--------|--------|------|--------|
| **Plain text** | `PlainTextPlugin` | `std::str` | ✅ Built-in |
| **Markdown** | `MarkdownPlugin` | `pulldown-cmark` | ✅ Built-in |
| **JSON** | `JsonPlugin` | `serde_json` | ✅ Built-in |
| **CSV** | `CsvPlugin` | `csv` crate | ✅ Built-in |
| **PDF** | `PdfPlugin` | `pdf-extract` | ✅ Included |
| **DOCX** | `DocxPlugin` | `docx-rs` | ✅ Included |
| **HTML** | `HtmlPlugin` | `html5ever` | ✅ Included |
| **Images** | `ImagePlugin` | `image-rs` + vision models | ✅ Included |
| **Audio** | `AudioPlugin` | `ffmpeg-next` + Whisper | ⚙️ Phase 9 |
| **Video** | `VideoPlugin` | `ffmpeg-next` | ⚙️ Phase 9 |
| **Binary** | `BinaryPlugin` | `object` crate (ELF, PE) | ⚙️ Phase 9 |
| **YAML/TOML** | Built-in TOML/YAML | `toml`, `yaml-rust` | ✅ Built-in |
| **XML** | `XmlPlugin` | `quick-xml` | ✅ Built-in |

**Plugin Discovery:**
Plugins are loaded from `/etc/bonsai/budis/plugins/`. Each `.so` file must export a `create_plugin()` function. The system hot-reloads plugins via BPCF.

### 3.3 Metadata & Entity Extraction

**Standard Metadata:**
- File hash (BLAKE3, SHA256)
- File size
- MIME type
- Creation / modification date
- Encoding (UTF-8, Latin-1, etc.)

**Domain-Specific Metadata:**
- **Images:** EXIF (camera, GPS, datetime), dominant colors, image size
- **Audio/Video:** Codec, bitrate, duration, sample rate
- **Documents:** Language, word count, heading hierarchy
- **Binary:** ELF sections, PE imports, symbol table

**Named Entity Recognition (NER):**
Using a fine-tuned BERT-base model (via `rust-bert`), extract:
- **PERSON** — Names of people
- **ORG** — Organization names
- **GPE** — Geographic locations
- **DATE** — Dates and times
- **MONEY** — Currency amounts
- **PERCENT** — Percentages
- **PRODUCT** — Product names
- **EVENT** — Event names

Each entity gets a confidence score (0-1).

**Relationship Extraction:**
Small transformer model identifies relations like:
- `(person, works_for, org)`
- `(document, cites, other_document)`
- `(technology, solves_problem, problem)`
- `(person, located_in, place)`

Relations are stored as edges in the KMDB graph.

### 3.4 Classification Engine

**Domain Classification:**
A lightweight transformer (DistilBERT, quantized to Q8_0) classifies the document into one of:
- **medical** — Healthcare, medicine, biology
- **legal** — Law, contracts, compliance
- **programming** — Code, software, development
- **finance** — Money, accounting, markets
- **engineering** — Hardware, infrastructure, physics
- **business** — Management, strategy, operations
- **creative** — Art, writing, design
- **educational** — Learning materials, textbooks
- **general** — Everything else

Confidence score per domain (sum to 1.0).

**Knowledge Type Classification:**
- **factual** — Statement of fact ("Paris is capital of France")
- **procedural** — Steps to accomplish something ("How to build X")
- **conceptual** — Explanation of a concept ("What is X")
- **meta** — Information about other documents
- **narrative** — Story or account
- **reference** — Dictionary, manual, lookup

**Audience Classification:**
- **novice** — Beginner level, simple explanation
- **intermediate** — Some background assumed
- **expert** — Advanced, assumes deep knowledge
- **manager** — Business-focused, executive summary

**Tag Extraction:**
Top-N keywords extracted from:
- Title and headings
- NER results (people, places, organizations)
- TF-IDF scores
- Named concepts from domain-specific knowledge bases

### 3.5 Deduplication & Merging

**Exact Duplicates:**
1. Compute BLAKE3 hash of raw file
2. Check if hash exists in CAS
3. If yes: skip re-storing; merge metadata

**Near-Duplicates:**
1. Compute MinHash signature of extracted text (64-bit hashes, 128 band, 2 hash functions)
2. LSH lookup to find similar documents (similarity threshold: 0.85)
3. If found: compare full Jaccard similarity
4. If >0.9: merge documents
   - Keep full text of the most complete version
   - Merge metadata (keep highest quality classification)
   - Store both references in KMDB (link as "similar_to")
   - User can review and confirm merge

**Merge Logic:**
```rust
fn merge_classifications(a: &Classification, b: &Classification) -> Classification {
    // Take domain with highest confidence
    let domain = if a.domain_confidence > b.domain_confidence {
        a.domain.clone()
    } else {
        b.domain.clone()
    };
    
    // Take knowledge type with highest confidence
    let knowledge_type = if a.knowledge_type_confidence > b.knowledge_type_confidence {
        a.knowledge_type.clone()
    } else {
        b.knowledge_type.clone()
    };
    
    // Merge tags (union)
    let mut tags = a.tags.clone();
    tags.extend(b.tags.clone());
    tags.sort();
    tags.dedup();
    
    Classification { domain, knowledge_type, tags }
}
```

### 3.6 Knowledge Module Builder

**Chunking:**
Documents are split into semantic chunks using:
- **Sentence splitting** (for text)
- **Page boundaries** (for PDFs, images)
- **Paragraph boundaries** (markdown, HTML)
- **Timestamp ranges** (audio/video)

Target chunk size: 200-500 tokens (using `tiktoken`).

**Embedding Generation:**
Each chunk gets a vector embedding using a local embedding model:
- **Model:** `all-MiniLM-L6-v2` (ONNX format, 384-dim)
- **Latency:** ~5ms per chunk
- **Memory:** <100MB for the model

**HNSW Index:**
Build a hierarchical navigable small world graph (HNSW) index for fast similarity search:
- **M:** 16 (connections per layer)
- **efConstruction:** 200
- **ef:** 100 (at search time)
- **Metric:** Cosine similarity

**Knowledge Module Package (.kmod):**
A ZIP file containing:
```
my-document.kmod
├── metadata.json          # Source, dates, classification, tags
├── chunks.jsonl           # Chunks with metadata
├── embeddings.bin         # Vector embeddings (packed floats)
├── index.hnsw             # HNSW index
├── entities.jsonl         # NER results and confidence
├── relationships.jsonl    # Extracted relationships
└── source_hash            # BLAKE3 of original file
```

**KMDB Registration:**
The module is registered in KMDB (SQLite) with:
- Module ID (UUID)
- Module name (derived from document title)
- Version (1.0)
- Domain, knowledge_type, tags
- Source file hash
- Creation timestamp
- Quality score (confidence-based)

### 3.7 Quality Scoring

Every extracted piece of information gets a quality score:

```
quality = 0.1 * source_quality 
        + 0.3 * extraction_confidence 
        + 0.4 * classification_confidence 
        + 0.2 * freshness_bonus
```

Where:
- **source_quality:** 1.0 for trusted sources, 0.7 for user-uploaded, 0.5 for web scrapes
- **extraction_confidence:** Model confidence for NER, entity extraction
- **classification_confidence:** Domain + knowledge type classifier confidence
- **freshness_bonus:** 1.0 for documents <1 year old, decays over time

Chunks with quality <0.6 are flagged for human review.

---

## 4. Continuous Learning Loop

### User Corrections Trigger Learning

**Scenario 1:** User re-classifies a document
```
Document was classified as: medical (confidence 0.72)
User changes it to: legal (confidence: user override)

EternalTrainingLoop logs this as a training example:
{
  "original_text": "...",
  "predicted_domain": "medical",
  "actual_domain": "legal",
  "timestamp": "2026-06-02T15:30:00Z"
}
```

**Scenario 2:** User corrects extracted entity
```
NER extracted: ["John Smith" (PERSON, 0.94), "Doe Corp" (ORG, 0.81)]
User corrects: Doe Corp → "Doe Corporation" (typo fix)

ETL logs:
{
  "entity": "Doe Corp",
  "correction": "Doe Corporation",
  "type": "ORGANIZATION"
}
```

### Nightly Fine-Tuning

Each night, the EternalTrainingLoop:
1. Collects all user corrections from the past day
2. Fine-tunes lightweight LoRA adapters on top of the base models
3. Tests on a holdout validation set
4. If accuracy improves: use new adapters; if not: discard
5. Updates the classifier model in-place via BPCF hot-reload

The base models never change—only small adapters are updated, keeping inference fast and memory-light.

---

## 5. Self-Healing & Error Recovery

### Extraction Failures

```
Plugin crashes on PDF → { "error": "segfault in pdf-extract" }
  │
  ├─ Survival System detects
  ├─ Logs to Universe
  ├─ Retries with fallback plugin (`pdftotext` command-line)
  │
  ├─ Fallback succeeds → extract text, continue
  ├─ Fallback fails → quarantine file, alert user via OmniBot
  │
└─ Bug Hunter fuzzes PDF plugin to find the crash cause
   └─ Report CVE / fix applied in next release
```

### Quarantine & Review

Failed files are stored in a quarantine directory (`/var/bonsai/budis/quarantine/`) with:
- Original file
- Extraction attempts + error logs
- Suggestion for manual processing

User can retry, skip, or manually categorize via UI.

### Self-Repair

If the same extraction error happens 3 times, the Survival System:
1. Disables the problematic plugin
2. Notifies the developer via GitHub issue
3. Routes future files of that type to an alternative plugin

---

## 6. Integration with Bonsai Subsystems

| Subsystem | Integration |
|-----------|-------------|
| **KMDB** | Ingested files become knowledge modules. Searchable via KMDB API. |
| **Universe** | Every ingestion, classification, merge is logged as an event. Full audit trail. |
| **Survival System** | Retries failed extractions. Auto-restarts crashed plugins. |
| **EternalTrainingLoop** | Collects user corrections. Fine-tunes classifiers overnight. |
| **Bug Hunter** | Fuzzes file format plugins for crashes and security vulnerabilities. |
| **Compute Fabric** | Offloads heavy AI (transcription, OCR, image analysis) to GPU nodes. |
| **BonsAI V2** | Provides the classification and NER models. |
| **OmniBot** | Users query: "Show me all engineering documents." → BUDIS retrieves. |
| **API Bridge** | Exposes tools: `ingest_file`, `classify_document`, `search_knowledge`. |
| **Bonsai Workspace** | File watcher + drag-and-drop UI. Real-time progress. |

---

## 7. Security & Privacy Considerations

### Data Privacy
- All processing is local. Files are never sent to external services.
- Extracted embeddings are stored locally in HNSW indices.
- NER results (people, organizations) are stored with a privacy flag—can be redacted before sharing.

### Access Control
- Each ingestion gets a capability token.
- Users can grant tokens with restricted access (e.g., "read but not write").
- System events (Universe) are immutable and auditable.

### Malware Detection
- Files are scanned for malicious payloads (via YARA rules or ClamAV integration).
- Binary analysis plugin detects suspicious functions (e.g., direct syscalls, inline assembly).
- Quarantine suspected malware.

---

## 8. Performance & Resource Requirements

### Processing Latency

| Task | Latency | Notes |
|------|---------|-------|
| **Plain text (1MB)** | 10ms | Instant |
| **PDF extraction (100 pages)** | 500ms | Text extraction |
| **Image OCR (1 image)** | 1-5s | GPU optional |
| **Audio transcription (1 minute)** | 3-10s | CPU efficient |
| **Classification (text)** | 50ms | DistilBERT on CPU |
| **Embedding (100 chunks)** | 500ms | Batch processing |
| **NER (text)** | 100ms | BERT-base |
| **Deduplication (1000 docs)** | <1s | MinHash LSH |

### Resource Usage

| Component | Memory | CPU |
|-----------|--------|-----|
| **Classification model** | 250MB | 1 core |
| **Embedding model** | 150MB | 1 core |
| **NER model** | 400MB | 1 core |
| **HNSW index (100K chunks)** | 2GB | — |
| **BUDIS service overhead** | 100MB | — |
| **Total (typical)** | **3-4GB** | **2-3 cores** |

---

## 9. Example Flows

### Flow 1: User Drops a PDF

```
User drags ~/Research/paper.pdf into Bonsai Workspace
  ↓
File watcher detects new file
  ↓
Plugin dispatcher routes to PdfPlugin
  ↓
PdfPlugin.extract_content() returns text: "Introduction...\n\nMethods..."
PdfPlugin.extract_metadata() returns: title="Novel NLP Methods", author="...", pages=12
  ↓
NER extracts: entities=[("Stanford", ORG), ("2026-05", DATE)]
  ↓
Classifier runs:
  - domain=programming (0.87)
  - knowledge_type=conceptual (0.92)
  - audience=expert (0.78)
  ↓
Deduplicator checks hash: not a duplicate
  ↓
Knowledge module builder:
  - Chunks text into 8 semantic chunks
  - Generates 384-dim embeddings per chunk
  - Builds HNSW index
  - Creates paper.kmod ZIP file
  - Stores in CAS
  - Registers in KMDB
  ↓
Universe logs: FileIngested { path: "paper.pdf", chunks: 8, domain: "programming" }
  ↓
OmniBot notifies user: "✅ Ingested 'Novel NLP Methods' (8 chunks, domain: programming)"
  ↓
User can now search: "How do they tokenize text?" → BUDIS retrieves relevant chunks
```

### Flow 2: User Corrects Classification

```
User opens BUDIS dashboard → sees "paper.pdf" classified as "programming"
User thinks: "Actually, this is more biology/bioinformatics"
User clicks: Change domain → biology
  ↓
UI sends: PATCH /api/document/{id}/classification with { domain: "biology" }
  ↓
BUDIS updates KMDB metadata and logs to Universe:
  DocumentReclassified { id, old_domain: "programming", new_domain: "biology" }
  ↓
EternalTrainingLoop collects this as a training example:
  {
    text_sample: "...[first 500 tokens]...",
    predicted_domain: "programming",
    actual_domain: "biology"
  }
  ↓
Next night, ETL fine-tunes the domain classifier on all collected corrections
  ↓
Model accuracy improves: 89% → 91%
  ↓
New model is hot-reloaded via BPCF (no restart)
  ↓
Future biology papers are classified more accurately
```

### Flow 3: Batch Import with Deduplication

```
User uploads a folder with 500 documents (~50 duplicates)
  ↓
BUDIS processes in parallel (8 workers)
  ↓
For each document:
  - Extract
  - Compute BLAKE3 hash
  - Check CAS for hash: ~50 are duplicates
  ↓
For duplicates:
  - Merge metadata (keep highest quality classification)
  - Store single reference in KMDB
  - Link as "duplicate_of"
  ↓
Final result: 450 unique knowledge modules + 50 duplicate references
  ↓
OmniBot notifies: "✅ Ingested 500 files → 450 unique modules (50 duplicates merged)"
  ↓
User can review merged duplicates in dashboard and confirm merge or split
```

---

## 10. Implementation Phases

| Phase | Goal | Components |
|-------|------|-----------|
| **1** | Core infra + text plugins | Plugin registry, file watcher, PlainText/Markdown/JSON/CSV parsers |
| **2** | PDF, DOCX, HTML extraction | Add PDF, DOCX, HTML plugins using established libraries |
| **3** | Metadata + NER | Standard metadata extraction, BERT-base NER via `rust-bert` |
| **4** | Classification engine | Fine-tuned DistilBERT model, domain + knowledge type classifiers |
| **5** | Dedup + merging | BLAKE3 hashing, MinHash LSH, metadata merging |
| **6** | Knowledge module builder | Text chunking, embedding generation, HNSW indexing, .kmod packaging, KMDB registration |
| **7** | Continuous learning | EternalTrainingLoop integration, user correction UI, nightly fine-tuning |
| **8** | Self-healing | Survival System hooks, quarantine, auto-retry, escalation |
| **9** | Advanced media | OCR, Whisper transcription, video frame extraction, binary analysis |
| **10** | Performance & scale | Distributed processing, caching, incremental indexing |

---

## 11. API Reference

### REST Endpoints

**POST /api/budis/ingest**
```json
Request:
{
  "file": "<base64-encoded file data>",
  "filename": "document.pdf",
  "metadata": { "source": "user-upload" }
}

Response:
{
  "job_id": "uuid-...",
  "status": "queued",
  "estimated_time_ms": 5000
}
```

**GET /api/budis/ingest/{job_id}**
```json
Response:
{
  "job_id": "uuid-...",
  "status": "completed",
  "result": {
    "module_id": "uuid-...",
    "title": "Novel NLP Methods",
    "chunks": 8,
    "domain": "programming",
    "knowledge_type": "conceptual",
    "quality_score": 0.87,
    "entities": [...]
  }
}
```

**PATCH /api/budis/document/{id}/classification**
```json
Request:
{
  "domain": "biology",
  "knowledge_type": "procedural",
  "tags": ["bioinformatics", "sequence-analysis"]
}

Response:
{
  "success": true,
  "updated_fields": ["domain", "tags"]
}
```

**GET /api/budis/search**
```
Query: ?text=mRNA&domain=biology&limit=10

Response:
{
  "results": [
    {
      "chunk_id": "uuid-...",
      "module_id": "uuid-...",
      "text": "mRNA is...",
      "relevance_score": 0.94,
      "source": "document.pdf"
    }
  ]
}
```

### MCP Tools

- **`ingest_file`** — Ingest a file and return module ID
- **`classify_document`** — Get classification for a document
- **`search_knowledge`** — Search ingested knowledge modules
- **`merge_duplicates`** — Review and confirm duplicate merges
- **`get_extraction_stats`** — Show processing statistics

---

## 12. Conclusion

**BUDIS** transforms your file system into a **living, searchable, self-improving knowledge base**. By combining intelligent extraction, automatic classification, continuous learning, and self-healing, BUDIS ensures that every document you own becomes accessible, organized, and understandable—without manual work.

With BUDIS integrated into the Bonsai Ecosystem, you have a complete sovereign knowledge management platform running entirely on your own hardware. 🌳🚀

