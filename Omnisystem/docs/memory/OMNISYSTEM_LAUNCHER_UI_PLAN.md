---
name: omnisystem_launcher_ui_plan
description: "Complete architecture and implementation plan for Omnisystem pre-launcher, launcher, and modular UI widget system"
metadata: 
  node_type: memory
  type: project
  originSessionId: c7ae2a7a-5206-469e-8d6b-97fc5255ee90
---

# OMNISYSTEM NEXT-GENERATION LAUNCHER & UI SYSTEM
## Comprehensive Enterprise-Grade Architecture Plan

**Status**: Approved for Implementation  
**Scope**: 3 interconnected systems, 150K+ LOC planned, 12-16 week implementation  
**Quality**: Enterprise-grade, bleeding-edge, childishly simple

---

## PART 1: SYSTEM ARCHITECTURE OVERVIEW

### Three-Layer Architecture

```
TIER 1: CORE FOUNDATION
├── Modular UI Widget System (SVG-based, 50+ components)
├── Theme Engine (Dark/Light/Custom, CSS-in-JS)
└── Animation Engine (Smooth transitions, 60fps)

TIER 2: SYSTEM SERVICES
├── Pre-Launcher Service (Compilation orchestrator)
├── Launcher Service (App registry, launch manager)
├── Monitor Service (System metrics, resource tracking)
└── Config Service (User preferences, themes, shortcuts)

TIER 3: USER INTERFACES
├── Pre-Launcher UI (One-click compilation & launch)
├── Launcher UI (App menu, system tray)
└── Feature UIs (100+ apps using modular widgets)
```

### System Relationships

```
User Click "Launch"
        ↓
PRE-LAUNCHER (Interactive Pre-Launcher)
├── Auto-detect system (CPU, GPU, RAM, OS)
├── Compile Omnisystem/modules
├── Run BugHunter diagnostics
├── Show visual progress (0-100%)
└── Launch → LAUNCHER
        ↓
LAUNCHER (App Menu + Orchestrator)
├── Display app grid (100+ apps)
├── User selects app
├── Auto-configure app for system
├── Monitor app lifecycle
└── Show system metrics (CPU, RAM, Network, Disk)
        ↓
FEATURE UIs (Built with Modular Widgets)
├── Each feature uses Widget System
├── Consistent look/feel across system
├── Responsive and accessible
└── Beautiful and childishly simple
```

---

## PART 2: INTERACTIVE PRE-LAUNCHER

### Purpose
One-click compilation, debugging, and system launch without manual configuration.

### Core Features

#### 2.1 System Auto-Detection
```rust
// omnisystem-prelauncher/src/detector.rs

pub struct SystemProfile {
    pub os: OperatingSystem,
    pub cpu_cores: u32,
    pub cpu_model: String,
    pub ram_gb: u32,
    pub gpu: Option<GPUInfo>,
    pub disk_free_gb: u64,
    pub screen_resolution: (u32, u32),
    pub is_vm: bool,
    pub cpu_vendor: String, // Intel/AMD/ARM/Apple
}

pub enum GPUInfo {
    NVIDIA { model: String, vram_gb: u32 },
    AMD { model: String, vram_gb: u32 },
    Intel { model: String },
    Apple { model: String },
    None,
}

impl SystemDetector {
    pub async fn detect() -> Result<SystemProfile>;
    pub async fn recommend_compile_flags() -> CompileFlags;
    pub async fn check_disk_space(needed_gb: u64) -> bool;
}
```

#### 2.2 One-Click Compilation

```rust
// omnisystem-prelauncher/src/compiler.rs

pub struct CompilationOrchestrator {
    pub profile: SystemProfile,
    pub target: BuildTarget,
    pub parallel_jobs: u32,
    pub progress_tx: Sender<CompilationProgress>,
}

pub struct CompilationProgress {
    pub phase: CompilePhase,
    pub percent: u8, // 0-100
    pub current_file: String,
    pub estimated_remaining_sec: u32,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

pub enum CompilePhase {
    Preparation(0-5),
    Dependency(5-10),
    Codegen(10-70),
    Linking(70-90),
    Optimization(90-98),
    Verification(98-100),
}

pub enum BuildTarget {
    Debug,
    Release,
    ReleaseLto,        // Link-time optimization
    ReleaseOptSize,    // Minimal binary size
}

impl CompilationOrchestrator {
    pub async fn compile_full_system() -> Result<BuildArtifact>;
    pub async fn compile_module(module: &str) -> Result<BuildArtifact>;
    pub async fn parallel_compile(modules: Vec<&str>) -> Result<Vec<BuildArtifact>>;
    pub async fn incremental_compile() -> Result<BuildArtifact>;
}
```

#### 2.3 BugHunter Pre-Flight Diagnostics

```rust
// omnisystem-prelauncher/src/diagnostics.rs

pub struct PreFlightCheck {
    pub compilation_errors: Vec<CompilationError>,
    pub dependency_conflicts: Vec<DependencyConflict>,
    pub missing_files: Vec<String>,
    pub version_mismatches: Vec<VersionMismatch>,
    pub security_warnings: Vec<SecurityWarning>,
    pub performance_concerns: Vec<PerformanceConcern>,
}

pub struct CompilationError {
    pub file_path: String,
    pub line: u32,
    pub message: String,
    pub severity: Severity, // Error/Warning/Info
    pub suggestion: Option<String>,
}

impl BugHunter {
    pub async fn run_preflight() -> PreFlightCheck;
    pub async fn validate_dependencies() -> Result<()>;
    pub async fn check_security() -> Vec<SecurityWarning>;
    pub async fn analyze_performance() -> Vec<PerformanceConcern>;
}
```

#### 2.4 Visual Progress UI

```svelte
<!-- pre-launcher.svelte -->

<div class="pre-launcher">
  <!-- Logo & Title -->
  <div class="header">
    <img src="/omnisystem-logo.svg" alt="Omnisystem" />
    <h1>Omnisystem Pre-Launcher</h1>
  </div>

  <!-- System Info -->
  <div class="system-info">
    <div class="stat">
      <span class="label">CPU:</span>
      <span class="value">{profile.cpu_model} ({profile.cpu_cores} cores)</span>
    </div>
    <div class="stat">
      <span class="label">RAM:</span>
      <span class="value">{profile.ram_gb} GB</span>
    </div>
    <div class="stat">
      <span class="label">GPU:</span>
      <span class="value">{profile.gpu?.model || 'Integrated'}</span>
    </div>
  </div>

  <!-- Compilation Progress -->
  <div class="progress-section">
    <h2>Compilation Progress</h2>
    
    <!-- Phase Progress Bar -->
    <div class="progress-bar">
      <div class="fill" style="width: {progress.percent}%">
        <span class="percent">{progress.percent}%</span>
      </div>
    </div>

    <!-- Phase Indicator -->
    <div class="phase-indicator">
      <span class="phase-label">{progress.phase}</span>
      <span class="time-remaining">{progress.estimated_remaining_sec}s remaining</span>
    </div>

    <!-- Current File -->
    <div class="current-file">
      <span class="label">Compiling:</span>
      <span class="file">{progress.current_file}</span>
    </div>

    <!-- Live Log -->
    <div class="live-log">
      {#each progress.messages as message}
        <div class="log-entry {message.level}">
          {message.text}
        </div>
      {/each}
    </div>
  </div>

  <!-- Diagnostics Section -->
  {#if diagnostics.errors.length > 0}
    <div class="diagnostics error">
      <h3>⚠️ Issues Found</h3>
      <div class="issue-list">
        {#each diagnostics.errors as error}
          <div class="issue {error.severity}">
            <span class="file">{error.file_path}:{error.line}</span>
            <span class="message">{error.message}</span>
            {#if error.suggestion}
              <span class="suggestion">💡 {error.suggestion}</span>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Action Buttons -->
  <div class="buttons">
    <button on:click={compile} disabled={isCompiling}>
      {isCompiling ? 'Compiling...' : 'Compile & Launch'}
    </button>
    <button on:click={compileFast} variant="secondary">
      Quick Build (Debug)
    </button>
    <button on:click={showAdvanced} variant="outline">
      Advanced Options
    </button>
  </div>
</div>

<style>
  .pre-launcher {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    padding: 2rem;
    background: var(--bg-primary);
    color: var(--text-primary);
    border-radius: 12px;
    max-width: 800px;
    margin: 0 auto;
  }

  .header {
    text-align: center;
    margin-bottom: 1rem;
  }

  .header img {
    width: 64px;
    height: 64px;
    margin-bottom: 1rem;
  }

  .system-info {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    padding: 1rem;
    background: var(--bg-secondary);
    border-radius: 8px;
  }

  .progress-bar {
    position: relative;
    height: 8px;
    background: var(--bg-tertiary);
    border-radius: 4px;
    overflow: hidden;
  }

  .progress-bar .fill {
    height: 100%;
    background: linear-gradient(90deg, #10b981, #06b6d4);
    transition: width 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .buttons {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
  }

  button {
    flex: 1;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }
</style>
```

### Pre-Launcher File Structure

```
omnisystem-prelauncher/
├── src/
│   ├── lib.rs                    # Main module
│   ├── detector.rs               # System detection (500 LOC)
│   ├── compiler.rs               # Compilation orchestrator (800 LOC)
│   ├── diagnostics.rs            # BugHunter integration (600 LOC)
│   ├── progress.rs               # Progress tracking (400 LOC)
│   └── config.rs                 # User preferences (300 LOC)
├── ui/
│   ├── prelauncher.svelte        # Main UI (400 LOC)
│   ├── progress.svelte           # Progress component (250 LOC)
│   └── diagnostics.svelte        # Diagnostics display (300 LOC)
└── tests/
    ├── detector_tests.rs
    ├── compiler_tests.rs
    └── diagnostics_tests.rs
```

**Phase 1 LOC**: ~3,500 (Core foundation)

---

## PART 3: NEXT-GENERATION LAUNCHER

### Purpose
Full-featured app menu and system orchestrator with 100+ applications.

### Core Features

#### 3.1 App Registry System

```rust
// omnisystem-launcher/src/app_registry.rs

pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AppCategory,
    pub icon: String,                    // SVG path or base64
    pub launch_command: String,
    pub dependencies: Vec<String>,
    pub min_ram_mb: u32,
    pub min_disk_mb: u32,
    pub tags: Vec<String>,
    pub is_favorite: bool,
    pub last_launched: Option<u64>,
    pub config: AppConfig,
}

pub enum AppCategory {
    System,
    Development,
    Productivity,
    Graphics,
    Network,
    Security,
    Gaming,
    Utilities,
    Custom(String),
}

pub struct AppRegistry {
    pub apps: DashMap<String, AppEntry>,
    pub search_index: SearchIndex,
}

impl AppRegistry {
    pub async fn load_from_disk() -> Result<Self>;
    pub async fn get_app(id: &str) -> Option<AppEntry>;
    pub async fn list_by_category(category: AppCategory) -> Vec<AppEntry>;
    pub async fn search(query: &str) -> Vec<SearchResult>;
    pub async fn get_recent(limit: u32) -> Vec<AppEntry>;
    pub async fn get_favorites() -> Vec<AppEntry>;
}
```

#### 3.2 Launch Manager

```rust
// omnisystem-launcher/src/launch_manager.rs

pub struct LaunchManager {
    pub app_registry: AppRegistry,
    pub config_manager: ConfigManager,
    pub process_monitor: ProcessMonitor,
    pub resource_monitor: ResourceMonitor,
}

pub struct LaunchConfig {
    pub app_id: String,
    pub args: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub working_dir: PathBuf,
    pub priority: ProcessPriority,
    pub auto_close_on_error: bool,
}

impl LaunchManager {
    pub async fn launch_app(app_id: &str) -> Result<ProcessHandle>;
    pub async fn launch_with_config(config: LaunchConfig) -> Result<ProcessHandle>;
    pub async fn pre_launch_checks(app_id: &str) -> Result<()>;
    pub async fn auto_configure_app(app_id: &str) -> Result<LaunchConfig>;
}
```

#### 3.3 System Monitoring Dashboard

```rust
// omnisystem-launcher/src/monitor.rs

pub struct SystemMetrics {
    pub cpu_percent: f32,
    pub ram_percent: f32,
    pub disk_percent: f32,
    pub network_mbps: f32,
    pub temperature_c: Option<f32>,
    pub uptime_seconds: u64,
}

pub struct ProcessMetrics {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub ram_mb: u32,
    pub status: ProcessStatus,
}

pub struct ResourceMonitor {
    pub metrics: Arc<DashMap<u32, ProcessMetrics>>,
    pub system_metrics: Arc<Mutex<SystemMetrics>>,
}

impl ResourceMonitor {
    pub async fn start_monitoring();
    pub async fn get_system_metrics() -> SystemMetrics;
    pub async fn get_process_metrics(pid: u32) -> Option<ProcessMetrics>;
    pub async fn get_all_processes() -> Vec<ProcessMetrics>;
}
```

#### 3.4 Launcher UI

```svelte
<!-- launcher.svelte -->

<div class="launcher">
  <!-- Top Bar -->
  <div class="top-bar">
    <div class="search-bar">
      <input 
        type="text" 
        placeholder="Search apps..." 
        bind:value={searchQuery}
        on:input={handleSearch}
      />
      <span class="search-icon">🔍</span>
    </div>

    <div class="system-stats">
      <div class="stat">
        <span class="label">CPU</span>
        <span class="value">{metrics.cpu_percent.toFixed(1)}%</span>
      </div>
      <div class="stat">
        <span class="label">RAM</span>
        <span class="value">{metrics.ram_percent.toFixed(1)}%</span>
      </div>
      <div class="stat">
        <span class="label">Disk</span>
        <span class="value">{metrics.disk_percent.toFixed(1)}%</span>
      </div>
    </div>

    <div class="top-actions">
      <button on:click={toggleSettings} title="Settings">⚙️</button>
      <button on:click={toggleTheme} title="Theme">🌙</button>
      <button on:click={toggleNotifications} title="Notifications">🔔</button>
    </div>
  </div>

  <!-- App Grid -->
  <div class="content">
    {#if searchResults.length > 0 || searchQuery}
      <div class="search-results">
        <h2>Search Results ({searchResults.length})</h2>
        <div class="app-grid">
          {#each searchResults as app}
            <AppCard {app} on:launch={launchApp} />
          {/each}
        </div>
      </div>
    {:else}
      <!-- Favorites Section -->
      {#if favorites.length > 0}
        <section class="category-section">
          <h2 class="section-title">⭐ Favorites</h2>
          <div class="app-grid">
            {#each favorites as app}
              <AppCard {app} on:launch={launchApp} isFavorite={true} />
            {/each}
          </div>
        </section>
      {/if}

      <!-- Recently Launched Section -->
      {#if recent.length > 0}
        <section class="category-section">
          <h2 class="section-title">🕐 Recently Launched</h2>
          <div class="app-grid compact">
            {#each recent as app}
              <AppCard {app} on:launch={launchApp} variant="compact" />
            {/each}
          </div>
        </section>
      {/if}

      <!-- Category Sections -->
      {#each categories as category}
        {#if categoryApps[category].length > 0}
          <section class="category-section">
            <h2 class="section-title">{getCategoryIcon(category)} {category}</h2>
            <div class="app-grid">
              {#each categoryApps[category] as app}
                <AppCard {app} on:launch={launchApp} />
              {/each}
            </div>
          </section>
        {/if}
      {/each}
    {/if}
  </div>

  <!-- Sidebar -->
  {#if showSidebar}
    <div class="sidebar">
      <div class="sidebar-section">
        <h3>Quick Actions</h3>
        <button on:click={openSettings}>Settings</button>
        <button on:click={openTerminal}>Terminal</button>
        <button on:click={openFileManager}>Files</button>
        <button on:click={restartSystem}>Restart</button>
      </div>

      <div class="sidebar-section">
        <h3>Running Apps ({runningApps.length})</h3>
        <div class="process-list">
          {#each runningApps as proc}
            <div class="process-item">
              <span class="name">{proc.name}</span>
              <span class="cpu">{proc.cpu_percent.toFixed(1)}%</span>
              <button on:click={() => killProcess(proc.pid)}>✕</button>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}

  <!-- Launch Dialog -->
  {#if launchingApp}
    <div class="launch-dialog">
      <div class="dialog-content">
        <div class="spinner"></div>
        <h2>Launching {launchingApp.name}</h2>
        <p>Preparing application...</p>
        <div class="progress-bar">
          <div class="fill" style="width: {launchProgress}%"></div>
        </div>
        <button on:click={cancelLaunch} variant="secondary">Cancel</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .launcher {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--bg-primary);
    color: var(--text-primary);
  }

  .top-bar {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 1rem 1.5rem;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .search-bar {
    flex: 1;
    position: relative;
    max-width: 400px;
  }

  .search-bar input {
    width: 100%;
    padding: 0.75rem 1rem;
    padding-right: 2.5rem;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    font-size: 1rem;
    outline: none;
    transition: all 0.2s ease;
  }

  .search-bar input:focus {
    border-color: var(--accent-color);
    box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.1);
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
  }

  .app-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 1.5rem;
    margin-top: 1rem;
  }

  .category-section {
    margin-bottom: 3rem;
  }

  .section-title {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: var(--text-secondary);
  }
</style>

<!-- AppCard Component -->
<script lang="ts">
  export let app: AppEntry;
  export let isFavorite = false;
  export let variant = 'normal';
  
  let isHovering = false;
</script>

<div class="app-card {variant}" on:mouseenter={() => isHovering = true} on:mouseleave={() => isHovering = false}>
  <div class="card-content">
    <div class="icon-wrapper">
      {@html app.icon}
    </div>
    
    <h3 class="app-name">{app.name}</h3>
    
    {#if isHovering && variant === 'normal'}
      <p class="app-description">{app.description}</p>
    {/if}
  </div>

  {#if isHovering}
    <div class="card-actions">
      <button on:click={() => dispatch('launch', app)} class="btn-primary">
        Launch
      </button>
      <button on:click={() => toggleFavorite(app)} class="btn-secondary">
        {isFavorite ? '⭐' : '☆'}
      </button>
    </div>
  {/if}
</div>
```

### Launcher File Structure

```
omnisystem-launcher/
├── src/
│   ├── lib.rs                    # Main module
│   ├── app_registry.rs           # App registry (700 LOC)
│   ├── launch_manager.rs         # Launch orchestration (600 LOC)
│   ├── monitor.rs                # System monitoring (500 LOC)
│   ├── process.rs                # Process management (400 LOC)
│   ├── search.rs                 # Smart search engine (500 LOC)
│   ├── config.rs                 # Configuration (300 LOC)
│   └── ipc.rs                    # IPC communication (300 LOC)
├── ui/
│   ├── launcher.svelte           # Main launcher UI (600 LOC)
│   ├── app-card.svelte           # App card component (200 LOC)
│   ├── search-bar.svelte         # Search component (250 LOC)
│   ├── system-stats.svelte       # Metrics display (200 LOC)
│   └── process-monitor.svelte    # Process viewer (300 LOC)
└── tests/
    ├── app_registry_tests.rs
    ├── launch_manager_tests.rs
    └── monitor_tests.rs
```

**Phase 2 LOC**: ~5,500 (Full launcher)

---

## PART 4: MODULAR UI WIDGET SYSTEM

### Purpose
Universal, reusable, accessible UI components for entire Omnisystem.

### Architecture

```
omnisystem-ui-widgets/
├── widgets/
│   ├── Button/
│   │   ├── Button.svelte
│   │   ├── ButtonGroup.svelte
│   │   └── styles.css
│   ├── Input/
│   │   ├── TextInput.svelte
│   │   ├── NumberInput.svelte
│   │   ├── SearchInput.svelte
│   │   └── FileInput.svelte
│   ├── Layout/
│   │   ├── Container.svelte
│   │   ├── Grid.svelte
│   │   ├── Flex.svelte
│   │   ├── Sidebar.svelte
│   │   └── Modal.svelte
│   ├── Navigation/
│   │   ├── Tabs.svelte
│   │   ├── Menu.svelte
│   │   ├── Breadcrumb.svelte
│   │   └── Pagination.svelte
│   ├── Display/
│   │   ├── Badge.svelte
│   │   ├── Card.svelte
│   │   ├── Alert.svelte
│   │   ├── Progress.svelte
│   │   └── Tooltip.svelte
│   ├── Form/
│   │   ├── Checkbox.svelte
│   │   ├── Radio.svelte
│   │   ├── Select.svelte
│   │   ├── Toggle.svelte
│   │   └── Slider.svelte
│   ├── Table/
│   │   ├── Table.svelte
│   │   ├── DataGrid.svelte
│   │   └── TreeView.svelte
│   ├── Chart/
│   │   ├── LineChart.svelte
│   │   ├── BarChart.svelte
│   │   ├── PieChart.svelte
│   │   └── AreaChart.svelte
│   └── Feedback/
│       ├── Spinner.svelte
│       ├── Skeleton.svelte
│       ├── Toast.svelte
│       └── Snackbar.svelte
├── theme/
│   ├── themes.ts                 # Theme definitions
│   ├── dark.css                  # Dark theme
│   ├── light.css                 # Light theme
│   ├── custom.ts                 # Custom theme builder
│   └── colors.ts                 # Color palettes
├── animations/
│   ├── transitions.ts            # 20+ transition types
│   ├── keyframes.ts              # 30+ keyframe animations
│   └── timing.ts                 # Easing functions
├── accessibility/
│   ├── aria.ts                   # ARIA helpers
│   ├── keyboard.ts               # Keyboard navigation
│   ├── focus-manager.ts          # Focus management
│   └── screen-reader.ts          # Screen reader support
├── utilities/
│   ├── colors.ts                 # Color utilities
│   ├── spacing.ts                # Spacing helpers
│   ├── typography.ts             # Font utilities
│   ├── responsive.ts             # Responsive design helpers
│   └── shadows.ts                # Shadow utilities
├── database/
│   ├── widgets.json              # Widget registry (1,000+ entries)
│   ├── components.json           # Component metadata
│   ├── patterns.json             # UI patterns library
│   └── examples.json             # Code examples
└── docs/
    ├── WIDGETS_DB.md             # Complete widget database
    ├── COMPONENT_GUIDE.md        # Component usage guide
    ├── THEME_GUIDE.md            # Theming guide
    └── ACCESSIBILITY.md          # Accessibility documentation
```

### 4.1 Core Widget Components

#### Button Widget

```svelte
<!-- Button.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  
  export let label: string = '';
  export let variant: 'primary' | 'secondary' | 'outline' | 'ghost' = 'primary';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let disabled: boolean = false;
  export let loading: boolean = false;
  export let icon: string | undefined = undefined;
  export let onClick: () => void = () => {};
  export let ariaLabel: string = '';

  let element: HTMLElement;

  const handleClick = () => {
    if (!disabled && !loading) {
      onClick();
    }
  };

  const handleKeydown = (e: KeyboardEvent) => {
    if ((e.key === 'Enter' || e.key === ' ') && !disabled && !loading) {
      handleClick();
    }
  };
</script>

<button
  {disabled}
  on:click={handleClick}
  on:keydown={handleKeydown}
  class="button button-{variant} button-{size}"
  aria-label={ariaLabel || label}
  aria-busy={loading}
  bind:this={element}
>
  {#if loading}
    <span class="spinner"></span>
  {:else if icon}
    <span class="icon">{@html icon}</span>
  {/if}
  
  {#if label}
    <span class="label">{label}</span>
  {/if}
  
  <slot></slot>
</button>

<style>
  .button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 1rem;
  }

  /* Variants */
  .button-primary {
    background: var(--color-primary);
    color: white;
  }

  .button-primary:hover:not(:disabled) {
    background: var(--color-primary-dark);
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
  }

  .button-secondary {
    background: var(--color-secondary);
    color: white;
  }

  .button-outline {
    background: transparent;
    color: var(--color-primary);
    border: 2px solid var(--color-primary);
  }

  .button-ghost {
    background: transparent;
    color: var(--text-primary);
  }

  /* Sizes */
  .button-sm {
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
  }

  .button-lg {
    padding: 1rem 1.5rem;
    font-size: 1.125rem;
  }

  /* States */
  .button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
```

#### Modal Dialog

```svelte
<!-- Modal.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { createEventDispatcher } from 'svelte';
  
  export let isOpen: boolean = false;
  export let title: string = '';
  export let size: 'sm' | 'md' | 'lg' | 'xl' = 'md';
  export let closeOnBackdrop: boolean = true;
  export let closeOnEscape: boolean = true;

  const dispatch = createEventDispatcher();

  let dialogElement: HTMLDialogElement;

  $: if (isOpen && dialogElement) {
    dialogElement.showModal();
  } else if (!isOpen && dialogElement) {
    dialogElement.close();
  }

  const handleBackdropClick = (e: MouseEvent) => {
    if (closeOnBackdrop && e.target === dialogElement) {
      dispatch('close');
    }
  };

  const handleKeydown = (e: KeyboardEvent) => {
    if (closeOnEscape && e.key === 'Escape') {
      dispatch('close');
    }
  };
</script>

<dialog
  bind:this={dialogElement}
  on:click={handleBackdropClick}
  on:keydown={handleKeydown}
  class="modal modal-{size}"
>
  <div class="modal-content">
    {#if title}
      <div class="modal-header">
        <h2 class="modal-title">{title}</h2>
        <button
          class="close-button"
          on:click={() => dispatch('close')}
          aria-label="Close dialog"
        >
          ✕
        </button>
      </div>
    {/if}

    <div class="modal-body">
      <slot></slot>
    </div>

    {#if $$slots.footer}
      <div class="modal-footer">
        <slot name="footer"></slot>
      </div>
    {/if}
  </div>
</dialog>

<style>
  .modal {
    border: none;
    border-radius: 12px;
    padding: 0;
    max-width: none;
    background: var(--bg-secondary);
    color: var(--text-primary);
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
  }

  .modal::backdrop {
    background: rgba(0, 0, 0, 0.5);
    animation: fadeIn 0.2s ease;
  }

  .modal-sm { max-width: 384px; }
  .modal-md { max-width: 512px; }
  .modal-lg { max-width: 768px; }
  .modal-xl { max-width: 1024px; }

  .modal-content {
    display: flex;
    flex-direction: column;
    max-height: 90vh;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .modal-title {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0;
  }

  .close-button {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    opacity: 0.6;
    transition: opacity 0.2s ease;
  }

  .close-button:hover {
    opacity: 1;
  }

  .modal-body {
    flex: 1;
    padding: 1.5rem;
    overflow-y: auto;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    padding: 1.5rem;
    border-top: 1px solid var(--border-color);
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
```

### 4.2 Theme System

```typescript
// theme/themes.ts

export interface Theme {
  name: string;
  colors: {
    primary: string;
    secondary: string;
    success: string;
    warning: string;
    error: string;
    info: string;
    background: {
      primary: string;
      secondary: string;
      tertiary: string;
    };
    text: {
      primary: string;
      secondary: string;
      tertiary: string;
    };
    border: string;
  };
  typography: {
    fontFamily: string;
    fontSize: {
      xs: string;
      sm: string;
      base: string;
      lg: string;
      xl: string;
      '2xl': string;
    };
    fontWeight: {
      light: number;
      regular: number;
      medium: number;
      semibold: number;
      bold: number;
    };
    lineHeight: {
      tight: number;
      normal: number;
      relaxed: number;
    };
  };
  spacing: {
    xs: string;
    sm: string;
    md: string;
    lg: string;
    xl: string;
  };
  shadows: {
    sm: string;
    md: string;
    lg: string;
    xl: string;
  };
  borderRadius: {
    sm: string;
    md: string;
    lg: string;
    full: string;
  };
  transitions: {
    fast: string;
    base: string;
    slow: string;
  };
}

export const darkTheme: Theme = {
  name: 'dark',
  colors: {
    primary: '#10b981',
    secondary: '#06b6d4',
    success: '#10b981',
    warning: '#f59e0b',
    error: '#ef4444',
    info: '#3b82f6',
    background: {
      primary: '#1a1a1a',
      secondary: '#2d2d2d',
      tertiary: '#3f3f3f',
    },
    text: {
      primary: '#ffffff',
      secondary: '#d1d5db',
      tertiary: '#9ca3af',
    },
    border: '#404040',
  },
  // ... rest of theme
};

export const lightTheme: Theme = {
  name: 'light',
  colors: {
    primary: '#059669',
    secondary: '#0891b2',
    success: '#059669',
    warning: '#d97706',
    error: '#dc2626',
    info: '#2563eb',
    background: {
      primary: '#ffffff',
      secondary: '#f9fafb',
      tertiary: '#f3f4f6',
    },
    text: {
      primary: '#1a1a1a',
      secondary: '#4b5563',
      tertiary: '#9ca3af',
    },
    border: '#e5e7eb',
  },
  // ... rest of theme
};

export class ThemeManager {
  private currentTheme: Theme = darkTheme;
  private themes: Map<string, Theme> = new Map([
    ['dark', darkTheme],
    ['light', lightTheme],
  ]);

  setTheme(name: string) {
    const theme = this.themes.get(name);
    if (theme) {
      this.currentTheme = theme;
      this.applyTheme(theme);
    }
  }

  private applyTheme(theme: Theme) {
    const root = document.documentElement;
    Object.entries(theme.colors).forEach(([key, value]) => {
      if (typeof value === 'string') {
        root.style.setProperty(`--color-${key}`, value);
      } else {
        Object.entries(value).forEach(([subKey, subValue]) => {
          root.style.setProperty(`--color-${key}-${subKey}`, subValue);
        });
      }
    });
  }

  registerTheme(theme: Theme) {
    this.themes.set(theme.name, theme);
  }

  getTheme(name: string): Theme | undefined {
    return this.themes.get(name);
  }
}
```

### 4.3 Modular UI Widgets Database

```json
{
  "widgets": {
    "button": {
      "name": "Button",
      "category": "Input Controls",
      "description": "Primary interactive element for triggering actions",
      "variants": ["primary", "secondary", "outline", "ghost"],
      "sizes": ["sm", "md", "lg"],
      "props": {
        "label": "string",
        "variant": "string",
        "size": "string",
        "disabled": "boolean",
        "loading": "boolean",
        "icon": "string"
      },
      "examples": [
        {
          "name": "Primary Button",
          "code": "<Button label=\"Click me\" variant=\"primary\" on:click={() => {}} />"
        }
      ],
      "accessibility": {
        "wcag_level": "AAA",
        "aria_attributes": ["aria-label", "aria-busy"],
        "keyboard_support": "Enter, Space"
      },
      "performance": {
        "bundle_size": "2.5KB",
        "render_time": "<1ms"
      }
    },
    "modal": {
      "name": "Modal Dialog",
      "category": "Layout",
      "description": "Overlay dialog for focused user interactions",
      "sizes": ["sm", "md", "lg", "xl"],
      "props": {
        "isOpen": "boolean",
        "title": "string",
        "size": "string",
        "closeOnBackdrop": "boolean",
        "closeOnEscape": "boolean"
      },
      "accessibility": {
        "wcag_level": "AAA",
        "aria_attributes": ["role=dialog", "aria-labelledby", "aria-hidden"],
        "keyboard_support": "Escape to close"
      }
    },
    // ... 50+ more widgets
  },
  "components": {
    "form": {
      "description": "Complete form component system",
      "widgets": ["TextInput", "Select", "Checkbox", "Radio", "Textarea"]
    },
    "navigation": {
      "description": "Navigation and menu components",
      "widgets": ["Menu", "Tabs", "Breadcrumb", "Pagination"]
    },
    // ... more component groups
  },
  "patterns": {
    "data-table": {
      "name": "Data Table",
      "description": "Sortable, filterable data display",
      "components": ["Table", "Sort", "Filter", "Pagination"]
    },
    "authentication": {
      "name": "Login Form",
      "description": "Complete login experience",
      "components": ["TextInput", "Button", "Alert"]
    }
    // ... 50+ UI patterns
  }
}
```

### Widget Database File Structure

```
omnisystem-ui-widgets/database/
├── widgets.json              # 50+ widget definitions
├── components.json           # 15+ component groups
├── patterns.json            # 50+ common UI patterns
├── examples.json            # 200+ code examples
├── accessibility.json       # WCAG compliance data
├── performance.json         # Bundle size & render metrics
└── index.md                 # Auto-generated widget database
```

---

## PART 5: IMPLEMENTATION ROADMAP

### Timeline: 12-16 Weeks

#### Week 1-2: Foundation Setup
- [ ] Set up project structure
- [ ] Create base types and interfaces
- [ ] Set up build pipeline
- [ ] Create 10 core widgets

#### Week 3-4: Pre-Launcher MVP
- [ ] System detection (3,500 LOC)
- [ ] Compilation orchestrator
- [ ] Basic UI
- [ ] BugHunter integration

#### Week 5-6: Launcher Core
- [ ] App registry system
- [ ] Process management
- [ ] Search functionality
- [ ] Launch manager

#### Week 7-8: Launcher UI
- [ ] Launcher interface (5,500 LOC)
- [ ] Real-time monitoring
- [ ] Keyboard navigation
- [ ] Theme support

#### Week 9-10: Widget Library Expansion
- [ ] Build 30+ widgets
- [ ] Theme system
- [ ] Animation engine
- [ ] Accessibility layer

#### Week 11-12: Widget Database
- [ ] Document all widgets (1,000+ entries)
- [ ] Create pattern library (50+ patterns)
- [ ] Generate examples (200+ examples)
- [ ] Build searchable database

#### Week 13-14: Feature Integration
- [ ] Integrate with Omnisystem features
- [ ] Build 20+ feature UIs
- [ ] Test across all modules
- [ ] Performance optimization

#### Week 15-16: Polishing & Launch
- [ ] Accessibility audit (WCAG AAA)
- [ ] Performance optimization
- [ ] Documentation
- [ ] Launch preparation

---

## PART 6: TECHNOLOGY STACK

### Frontend
- **Svelte 5** — Reactive UI framework
- **TypeScript** — Type safety
- **Vite** — Build tool (<1s rebuild)
- **TailwindCSS** — Utility CSS framework

### Backend
- **Rust** — Core system services
- **Tokio** — Async runtime
- **Axum** — Web framework (IPC)
- **Tauri 2** — Desktop integration

### Storage & Communication
- **SQLite** — Configuration storage
- **JSON** — Widget database
- **WebSocket** — Real-time updates
- **IPC** — Inter-process communication

### Tooling
- **Storybook** — Component documentation
- **Vitest** — Unit testing
- **Playwright** — E2E testing
- **ESLint + Prettier** — Code quality

---

## PART 7: CRITICAL SUCCESS FACTORS

### 1. Simplicity First
- Every widget must be usable by 5-year-old
- Maximum 2 clicks to launch any app
- Visual feedback for all interactions

### 2. Performance
- Launcher start: <1s
- App launch: <2s
- Widget render: <16ms (60fps)
- Memory: <100MB base

### 3. Accessibility
- WCAG AAA compliance
- Keyboard navigation everywhere
- Screen reader support
- High contrast modes

### 4. Security
- No arbitrary code execution
- Input validation everywhere
- Encrypted config storage
- Sandboxed app launches

### 5. Beauty
- Consistent design language
- Smooth animations (100ms duration)
- Proper spacing/typography
- Thoughtful color scheme

---

## PART 8: DEPLOYMENT & MAINTENANCE

### Release Strategy
- **Alpha (Week 14)**: Internal testing
- **Beta (Week 15)**: Community testing
- **GA (Week 16)**: Production launch

### Continuous Improvement
- Weekly widget additions
- Monthly theme releases
- Quarterly feature updates
- Annual major version

### Support
- Community forums
- Discord server
- Documentation wiki
- Issue tracker

---

## CONCLUSION

This three-part system creates a truly next-generation launcher experience:

1. **Pre-Launcher**: Removes friction from setup
2. **Launcher**: Makes all Omnisystem features accessible
3. **Widget System**: Makes every app beautiful

Together, they deliver an enterprise-grade experience that's childishly simple, beautiful, and fun.

**Total LOC**: ~9,000 (core systems)
**Total Components**: 50+ widgets, 15+ patterns
**Timeline**: 12-16 weeks
**Quality**: Enterprise-grade, WCAG AAA compliant

