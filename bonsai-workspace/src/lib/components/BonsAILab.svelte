<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onDestroy } from 'svelte';

  // ── State ──────────────────────────────────────────────────────────────────
  let baseModel = '';
  let bonsaiAdapter = '';
  let prompt = '';
  let comparing = false;
  let comparison: ComparisonResult | null = null;
  let error = '';

  // Training loop state
  let loopRunning = false;
  let loopPolling = false;
  let loopStatus: LoopStatus | null = null;
  let loopTimer: ReturnType<typeof setInterval> | null = null;

  // ── Types ─────────────────────────────────────────────────────────────────
  interface GapDetail {
    gap_type: string;
    description: string;
  }

  interface ComparisonResult {
    prompt: string;
    intent_match: boolean;
    tool_overlap_pct: number;
    bonsai_tools: string[];
    reference_tools: string[];
    bonsai_confidence: number | null;
    gaps: GapDetail[];
  }

  interface LoopStatus {
    running: boolean;
    rounds_completed: number;
    examples_collected: number;
    last_intent_match_pct: number;
    last_tool_overlap_pct: number;
    finetune_queued: boolean;
    last_error: string | null;
    elapsed_secs: number;
  }

  // ── Compare ───────────────────────────────────────────────────────────────
  async function runComparison() {
    if (!baseModel || !bonsaiAdapter || !prompt) return;
    comparing = true;
    error = '';
    comparison = null;
    try {
      comparison = await invoke<ComparisonResult>('compare_models', {
        baseModelPath: baseModel,
        bonsaiAdapterPath: bonsaiAdapter,
        prompt,
        gpuLayers: 35,
      });
    } catch (e: any) {
      error = String(e);
    } finally {
      comparing = false;
    }
  }

  // ── Training loop ─────────────────────────────────────────────────────────
  async function startLoop() {
    if (!baseModel || !bonsaiAdapter) {
      error = 'Set base model and adapter paths before starting the loop.';
      return;
    }
    error = '';
    try {
      await invoke('start_training_loop', {
        config: {
          base_model_path: baseModel,
          bonsai_adapter_path: bonsaiAdapter,
          output_data_path: '',
          output_adapter_path: '',
          gpu_layers: 35,
          finetune_threshold: 50,
          prompts: [],
          interval_secs: 5,
        },
      });
      loopRunning = true;
      startPolling();
    } catch (e: any) {
      error = String(e);
    }
  }

  async function stopLoop() {
    try {
      await invoke('stop_training_loop');
      loopRunning = false;
      stopPolling();
    } catch (e: any) {
      error = String(e);
    }
  }

  async function pollLoopStatus() {
    try {
      loopStatus = await invoke<LoopStatus>('get_training_loop_status');
      loopRunning = loopStatus?.running ?? false;
    } catch { /* silently skip */ }
  }

  function startPolling() {
    stopPolling();
    loopTimer = setInterval(pollLoopStatus, 2000);
    pollLoopStatus();
  }

  function stopPolling() {
    if (loopTimer) { clearInterval(loopTimer); loopTimer = null; }
  }

  onDestroy(stopPolling);

  // ── Helpers ───────────────────────────────────────────────────────────────
  function formatSecs(s: number) {
    const m = Math.floor(s / 60);
    const sec = s % 60;
    return m > 0 ? `${m}m ${sec}s` : `${sec}s`;
  }

  function overlapColor(pct: number) {
    if (pct >= 80) return '#34d399';
    if (pct >= 50) return '#fb923c';
    return '#f87171';
  }
</script>

<div class="lab">
  <h2 class="lab-title">BonsAI Lab</h2>
  <p class="lab-sub">Dual-model comparison and continuous training loop</p>

  <!-- ── Config ─────────────────────────────────────────────────────────── -->
  <section class="config-panel">
    <label>
      <span>Base model path (GGUF)</span>
      <input bind:value={baseModel} placeholder="D:/Models/model.gguf" spellcheck="false"/>
    </label>
    <label>
      <span>BonsAI adapter path (LoRA)</span>
      <input bind:value={bonsaiAdapter} placeholder="~/.bonsai/adapters/bonsai-latest" spellcheck="false"/>
    </label>
  </section>

  <!-- ── Comparison ─────────────────────────────────────────────────────── -->
  <section class="compare-panel">
    <h3>Single Comparison</h3>
    <div class="prompt-row">
      <textarea bind:value={prompt} placeholder="Enter a prompt to compare both models…" rows="3"/>
      <button class="btn-primary" on:click={runComparison} disabled={comparing || !baseModel || !bonsaiAdapter || !prompt}>
        {comparing ? 'Comparing…' : 'Compare'}
      </button>
    </div>

    {#if error}
      <div class="error-box">{error}</div>
    {/if}

    {#if comparison}
      <div class="results">
        <div class="metric-row">
          <div class="metric" class:good={comparison.intent_match} class:bad={!comparison.intent_match}>
            <span class="metric-label">Intent Match</span>
            <span class="metric-value">{comparison.intent_match ? '✓ Match' : '✗ Mismatch'}</span>
          </div>
          <div class="metric">
            <span class="metric-label">Tool Overlap</span>
            <span class="metric-value" style="color:{overlapColor(comparison.tool_overlap_pct)}">
              {comparison.tool_overlap_pct.toFixed(1)}%
            </span>
          </div>
          {#if comparison.bonsai_confidence != null}
            <div class="metric">
              <span class="metric-label">BonsAI Confidence</span>
              <span class="metric-value">{(comparison.bonsai_confidence * 100).toFixed(0)}%</span>
            </div>
          {/if}
        </div>

        <div class="tools-row">
          <div class="tools-col">
            <h4>BonsAI Tools</h4>
            {#if comparison.bonsai_tools.length}
              <ul>{#each comparison.bonsai_tools as t}<li>{t}</li>{/each}</ul>
            {:else}
              <span class="none">none</span>
            {/if}
          </div>
          <div class="tools-col">
            <h4>Reference Tools</h4>
            {#if comparison.reference_tools.length}
              <ul>{#each comparison.reference_tools as t}<li>{t}</li>{/each}</ul>
            {:else}
              <span class="none">none</span>
            {/if}
          </div>
        </div>

        {#if comparison.gaps.length}
          <div class="gaps">
            <h4>Gaps ({comparison.gaps.length})</h4>
            {#each comparison.gaps as gap}
              <div class="gap-item">
                <span class="gap-type">{gap.gap_type}</span>
                <span class="gap-desc">{gap.description}</span>
              </div>
            {/each}
          </div>
        {:else}
          <div class="no-gaps">No gaps detected — models agree.</div>
        {/if}
      </div>
    {/if}
  </section>

  <!-- ── Continuous Training Loop ───────────────────────────────────────── -->
  <section class="loop-panel">
    <h3>Continuous Training Loop</h3>
    <div class="loop-controls">
      {#if !loopRunning}
        <button class="btn-primary" on:click={startLoop} disabled={!baseModel || !bonsaiAdapter}>
          ▶ Start Loop
        </button>
      {:else}
        <button class="btn-stop" on:click={stopLoop}>■ Stop Loop</button>
      {/if}
    </div>

    {#if loopStatus}
      <div class="loop-status">
        <div class="loop-stat">
          <span>Status</span>
          <span class:running={loopStatus.running}>{loopStatus.running ? 'Running' : 'Stopped'}</span>
        </div>
        <div class="loop-stat">
          <span>Rounds</span>
          <span>{loopStatus.rounds_completed}</span>
        </div>
        <div class="loop-stat">
          <span>Examples</span>
          <span>{loopStatus.examples_collected}</span>
        </div>
        <div class="loop-stat">
          <span>Intent Match</span>
          <span>{loopStatus.last_intent_match_pct.toFixed(0)}%</span>
        </div>
        <div class="loop-stat">
          <span>Tool Overlap</span>
          <span style="color:{overlapColor(loopStatus.last_tool_overlap_pct)}">
            {loopStatus.last_tool_overlap_pct.toFixed(1)}%
          </span>
        </div>
        <div class="loop-stat">
          <span>Elapsed</span>
          <span>{formatSecs(loopStatus.elapsed_secs)}</span>
        </div>
        {#if loopStatus.finetune_queued}
          <div class="finetune-badge">Fine-tune queued…</div>
        {/if}
        {#if loopStatus.last_error}
          <div class="error-box">{loopStatus.last_error}</div>
        {/if}
      </div>
    {/if}
  </section>
</div>

<style>
  .lab {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding: 1.5rem;
    color: #e2e8f0;
    max-width: 860px;
    margin: 0 auto;
  }
  .lab-title { font-size: 1.4rem; margin: 0; }
  .lab-sub { color: #64748b; margin: -0.5rem 0 0; font-size: 0.9rem; }

  section { background: #1e293b; border-radius: 10px; padding: 1.25rem; }
  h3 { margin: 0 0 1rem; font-size: 1rem; color: #94a3b8; text-transform: uppercase; letter-spacing: .05em; }
  h4 { margin: 0 0 0.5rem; font-size: 0.85rem; color: #64748b; }

  .config-panel { display: flex; flex-direction: column; gap: 0.75rem; }
  label { display: flex; flex-direction: column; gap: 0.3rem; font-size: 0.85rem; color: #94a3b8; }
  input {
    background: #0f172a; border: 1px solid #334155; border-radius: 6px;
    color: #e2e8f0; padding: 0.5rem 0.75rem; font-size: 0.9rem; width: 100%;
    box-sizing: border-box;
  }
  input:focus { outline: none; border-color: #38bdf8; }

  .prompt-row { display: flex; gap: 0.75rem; align-items: flex-start; }
  textarea {
    flex: 1; background: #0f172a; border: 1px solid #334155; border-radius: 6px;
    color: #e2e8f0; padding: 0.5rem 0.75rem; font-size: 0.9rem; resize: vertical;
    font-family: inherit;
  }
  textarea:focus { outline: none; border-color: #38bdf8; }

  .btn-primary {
    background: #38bdf8; color: #0f172a; border: none; border-radius: 6px;
    padding: 0.6rem 1.25rem; font-weight: 600; cursor: pointer; white-space: nowrap;
  }
  .btn-primary:disabled { opacity: 0.4; cursor: not-allowed; }
  .btn-primary:hover:not(:disabled) { background: #7dd3fc; }

  .btn-stop {
    background: #f87171; color: #0f172a; border: none; border-radius: 6px;
    padding: 0.6rem 1.25rem; font-weight: 600; cursor: pointer;
  }
  .btn-stop:hover { background: #fca5a5; }

  .error-box { background: #3b1f1f; color: #f87171; border-radius: 6px; padding: 0.75rem; font-size: 0.85rem; margin-top: 0.75rem; }

  .results { display: flex; flex-direction: column; gap: 1rem; margin-top: 1rem; }
  .metric-row { display: flex; gap: 1rem; flex-wrap: wrap; }
  .metric {
    background: #0f172a; border-radius: 8px; padding: 0.75rem 1rem;
    display: flex; flex-direction: column; gap: 0.2rem; min-width: 120px;
  }
  .metric.good { border-left: 3px solid #34d399; }
  .metric.bad  { border-left: 3px solid #f87171; }
  .metric-label { font-size: 0.75rem; color: #64748b; }
  .metric-value { font-size: 1.1rem; font-weight: 600; }

  .tools-row { display: flex; gap: 1rem; }
  .tools-col { flex: 1; background: #0f172a; border-radius: 8px; padding: 0.75rem; }
  ul { margin: 0; padding-left: 1.25rem; font-size: 0.85rem; }
  li { margin: 0.15rem 0; }
  .none { font-size: 0.85rem; color: #475569; }

  .gaps { background: #0f172a; border-radius: 8px; padding: 0.75rem; }
  .gap-item { display: flex; gap: 0.75rem; align-items: baseline; margin: 0.3rem 0; font-size: 0.85rem; }
  .gap-type { background: #1e3a5f; color: #38bdf8; border-radius: 4px; padding: 0.1rem 0.4rem; font-size: 0.75rem; white-space: nowrap; }
  .gap-desc { color: #94a3b8; }
  .no-gaps { color: #34d399; font-size: 0.9rem; margin-top: 0.5rem; }

  .loop-controls { margin-bottom: 1rem; }
  .loop-status { display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.75rem; }
  .loop-stat {
    background: #0f172a; border-radius: 8px; padding: 0.6rem 0.75rem;
    display: flex; flex-direction: column; gap: 0.15rem;
  }
  .loop-stat span:first-child { font-size: 0.7rem; color: #64748b; text-transform: uppercase; }
  .loop-stat span:last-child { font-size: 1rem; font-weight: 600; }
  .running { color: #34d399; }
  .finetune-badge {
    grid-column: 1 / -1; background: #1e3a5f; color: #38bdf8;
    border-radius: 6px; padding: 0.5rem 0.75rem; font-size: 0.85rem;
    animation: pulse 1.5s ease-in-out infinite;
  }
  @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
</style>
