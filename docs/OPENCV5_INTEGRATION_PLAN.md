# OpenCV 5 Integration Plan - Omnisystem Architecture

**Status**: Ready for Implementation  
**Priority**: High  
**Scope**: 7 Phases, Multi-Language Implementation  
**Timeline**: 12-16 weeks (1 phase per 2 weeks, parallel work possible)

---

## Executive Summary

Transform OpenCV 5 from an external dependency into a **sovereign, formally verified, capability-secured native subsystem** of the Omnisystem. The integration spans:
- **Native C++ via UMS** (Foundation)
- **Rust Port** (Memory Safety)
- **Titan Rewrite** (Performance & Verification)
- **Sylva Scripting** (Developer Ergonomics)
- **Aether Distribution** (Planet-Scale)
- **Axiom Proof Engineering** (Mathematical Certainty)
- **Universal Validation Mesh** (Cross-Language Fidelity)

---

## Current Repository State Assessment

### ✅ Existing Foundation (228 crates, zero warnings)
- `p2p-*` transport layer (ready)
- `msg-*` messaging subsystem (ready)
- `omnisystem-*` language implementations (Titan, Sylva, Aether, Axiom available)
- `test-orchestrator` + `ubvm-mesh` validation fabric (ready)
- `sandbox` capability system (ready)
- `compile-cache` + `BACE` hot-reload (ready)

### 🎯 New Crates Required
```
crates/
├── cv/                          # Rust bindings (Phase 2)
├── cv-core/                     # Rust safe wrappers
├── cv-async/                    # Tokio/Rayon integration
├── cv-tests/                    # UVM test harness
│
├── opencv-native/               # C++ native via UMS (Phase 1)
├── opencv-manifest/             # UMS module manifests
├── opencv-build/                # CMake orchestration
│
├── titan-opencv/                # Titan rewrite (Phase 3)
├── titan-opencv-core/
├── titan-opencv-imgproc/
├── titan-opencv-dnn/
│
├── sylva-opencv/                # Sylva wrapper (Phase 4)
├── sylva-opencv-repl/           # Interactive REPL
│
├── aether-vision/               # Distributed pipeline (Phase 5)
├── aether-vision-actors/
│
├── axiom-opencv/                # Formal specs (Phase 6)
├── axiom-opencv-proofs/
│
└── cv-uvm/                      # Unified Validation Mesh
```

---

## Phase 0: Foundational Analysis & Specification

### 0.1 OpenCV 5 Header Analysis
**Deliverable**: `docs/opencv5-api-spec.json`

```bash
# Action: Parse OpenCV 5 headers from https://github.com/opencv/opencv/tree/5.x
# Tools: libclang, custom Python script
# Output: JSON spec with:
# - 50+ core functions (imread, imwrite, Mat::create, etc.)
# - Data types (Mat, UMat, Scalar, Point, Rect, etc.)
# - Pre/post-conditions, capabilities required
# - Target modules: core, imgproc, imgcodecs, videoio, dnn, features2d, objdetect
```

**Implementation**:
```python
# opencv5_extractor.py
import clang.cindex
import json

def extract_opencv_api(headers_dir):
    """Parse OpenCV 5 headers and generate canonical spec."""
    functions = []
    types = []
    
    # For each header file in opencv/include/
    for header in glob.glob(f"{headers_dir}/**/*.hpp", recursive=True):
        # Parse with libclang
        # Extract public functions, types, enums
        # Store capabilities, alignment, memory layout
    
    return {
        "version": "5.0.0",
        "timestamp": datetime.now().isoformat(),
        "functions": functions,
        "types": types,
        "hash": blake3(json.dumps(functions)).hexdigest()
    }
```

**Timeline**: 1 week

---

### 0.2 Canonical Test Suite Extraction
**Deliverable**: `crates/cv-uvm/tests/opencv_canonical_suite.yaml`

```yaml
version: 1.0
metadata:
  source: https://github.com/opencv/opencv/tree/5.x/modules/*/test
  total_tests: 2500+
  deterministic: true
  
test_groups:
  - name: core_matrix_operations
    tests:
      - id: test_mat_create
        inputs:
          rows: 100
          cols: 100
          depth: CV_8U
          channels: 3
        expected_output:
          shape: [100, 100, 3]
          memory_layout: row_major
          seed: 12345
          
  - name: imgproc_filters
    tests:
      - id: test_gaussian_blur
        inputs:
          radius: 5
          sigma: 1.0
        expected_output:
          # Bit-identical output across all implementations
          hash: blake3(output_pixels)
```

**Implementation**:
```rust
// crates/cv-uvm/src/canonical_suite.rs
pub struct CanonicalTest {
    pub id: String,
    pub seed: u64,
    pub inputs: TestInputs,
    pub expected: TestOutputs,
}

pub fn extract_opencv_tests() -> Vec<CanonicalTest> {
    // Load tests from opencv_canonical_suite.yaml
    // Each test becomes a deterministic fixture
    // For cross-language fidelity validation
}
```

**Timeline**: 1 week

---

## Phase 1: Universal Module System (UMS) Integration

### 1.1 Native OpenCV 5 Build System
**Deliverable**: `crates/opencv-build/build.ti` (Titan build orchestrator)

```titan
// crates/opencv-build/build.ti
module opencv_build

use std::process::{Command, Stdio}
use std::fs
use uce::compress

pub fn build_opencv_multiarch() -> Result<BuildArtifacts, Error> {
    let targets = vec![
        "x86_64-linux-gnu",
        "aarch64-linux-gnu",
        "riscv64-linux-gnu",
        "x86_64-macos",
        "aarch64-macos",
        "wasm32-wasi",
    ];
    
    let mut artifacts = Vec::new();
    
    for target in targets {
        // 1. Clone OpenCV 5 repository
        let opencv_src = fetch_opencv_5()?;
        
        // 2. Generate CMake presets for target
        let cmake_preset = generate_cmake_preset(target)?;
        
        // 3. Compile with optimizations
        let build_dir = format!("build/{}", target);
        fs::create_dir_all(&build_dir)?;
        
        let output = Command::new("cmake")
            .arg(&opencv_src)
            .arg("-DCMAKE_TOOLCHAIN_FILE")
            .arg(&cmake_preset)
            .arg("-DCMAKE_BUILD_TYPE=Release")
            .arg("-DBUILD_SHARED_LIBS=ON")
            .arg("-DENABLE_AVX512=ON")
            .arg("-DENABLE_SVE=ON")
            .arg("-DENABLE_NEON=ON")
            .current_dir(&build_dir)
            .output()?;
        
        if !output.status.success() {
            return Err(Error::BuildFailed(output.stderr));
        }
        
        // 4. Compress artifacts
        let libs = glob::glob(&format!("{}/**/*.so", build_dir))?
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        
        let compressed = compress(&libs)?;
        let hash = blake3::hash(&compressed);
        
        artifacts.push(BuildArtifact {
            target: target.to_string(),
            libs,
            hash: hash.to_hex().to_string(),
            size_bytes: compressed.len(),
        });
    }
    
    Ok(BuildArtifacts { artifacts })
}

fn generate_cmake_preset(target: &str) -> Result<String, Error> {
    // Generate target-specific CMake toolchain
    match target {
        "x86_64-linux-gnu" => Ok(CMAKE_X86_64_LINUX.to_string()),
        "aarch64-linux-gnu" => Ok(CMAKE_AARCH64_LINUX.to_string()),
        _ => Err(Error::UnsupportedTarget(target.to_string())),
    }
}
```

**Timeline**: 2 weeks (initial build, platform stabilization)

### 1.2 UMS Module Manifests
**Deliverable**: `crates/opencv-manifest/manifests/*.yaml`

```yaml
# crates/opencv-manifest/manifests/opencv-core-5.0.0-x86_64-linux.yaml
apiVersion: v1
kind: Module
metadata:
  name: opencv-core
  version: 5.0.0
  namespace: bonsai.omnisystem
  
spec:
  targets:
    - arch: x86_64
      os: linux
      
  exports:
    - name: cv_imread
      symbol: cv::imread
      signature: "fn(String, i32) -> Mat"
      
    - name: cv_imwrite
      symbol: cv::imwrite
      signature: "fn(String, &Mat) -> bool"
      
    - name: cv_create_mat
      symbol: cv::Mat::Mat
      signature: "fn(i32, i32, i32) -> Mat"
      
  capabilities:
    required:
      - memory: "256MB"
      - cpu: "avx2"
    optional:
      - gpu: "cuda_11.0"
      - cpu: "avx512f"
      
  dependencies:
    - module: libc
      version: "2.31"
      
  artifacts:
    - name: libopencv_core.so.5.0
      hash: "blake3:xxxxx..."
      size: 5242880
      compression: zstd
      
  signature:
    algorithm: bls_threshold_5of7
    signers:
      - council_member_1
      - council_member_2
      - council_member_3
      - council_member_4
      - council_member_5
    proof: "0x..."
```

**Implementation**:
```rust
// crates/opencv-manifest/src/lib.rs
use serde::{Deserialize, Serialize};
use blake3::Hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleManifest {
    pub metadata: Metadata,
    pub spec: ModuleSpec,
    pub signature: BLSSignature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleSpec {
    pub exports: Vec<ExportedSymbol>,
    pub capabilities: CapabilityRequirements,
    pub artifacts: Vec<Artifact>,
    pub dependencies: Vec<ModuleDependency>,
}

pub struct ExportedSymbol {
    pub name: String,
    pub symbol: String,
    pub signature: String,
}

pub struct CapabilityRequirements {
    pub required: Vec<Capability>,
    pub optional: Vec<Capability>,
}

#[derive(Clone)]
pub enum Capability {
    Memory { bytes: usize },
    CPU { features: Vec<String> }, // avx2, avx512f, sve, neon
    GPU { vendor: String, version: String },
}
```

**Timeline**: 1 week

### 1.3 Publishing & Council Signing
**Deliverable**: Published modules in global UMS mesh

```rust
// crates/opencv-manifest/src/publish.rs
pub async fn publish_opencv_modules(
    manifests: Vec<ModuleManifest>,
    council_signers: Vec<CouncilMember>,
) -> Result<PublishReceipt, Error> {
    
    // 1. Verify all manifests are valid
    for manifest in &manifests {
        manifest.validate()?;
    }
    
    // 2. Collect signatures from council (5-of-7 threshold)
    let mut signatures = Vec::new();
    for (i, signer) in council_signers.iter().enumerate() {
        if signatures.len() >= 5 { break; }
        
        let sig = signer.sign_manifest(&manifests).await?;
        signatures.push(sig);
    }
    
    if signatures.len() < 5 {
        return Err(Error::InsufficientSignatures);
    }
    
    // 3. Push to UMS mesh via TransferDaemon
    let receipt = transfer_daemon::publish_modules(
        manifests,
        signatures,
    ).await?;
    
    // 4. Record in observability (ledger)
    observability::log_module_publication(&receipt)?;
    
    Ok(receipt)
}
```

**Timeline**: 1 week (requires Council vote)

---

## Phase 2: Rust Port (`cv` Crate)

### 2.1 FFI Layer Generation
**Deliverable**: `crates/cv/src/sys/mod.rs` (auto-generated)

```bash
# Generate FFI bindings from OpenCV headers
bindgen \
    opencv/modules/core/include/opencv2/core.hpp \
    --output crates/cv/src/sys/core.rs \
    --whitelist-function "cv::.*" \
    --whitelist-type "cv::Mat" \
    --no-layout-tests
```

**Implementation**:
```rust
// crates/cv/Cargo.toml
[package]
name = "cv"
version = "5.0.0"

[dependencies]
opencv-sys = { path = "../opencv-sys" }  # Auto-generated FFI
tokio = { version = "1", features = ["full"] }
rayon = "1.7"
anyhow = "1"

[target.'cfg(any(target_os = "linux", target_os = "macos"))'.dependencies]
libloading = "0.8"  # For dynamic module loading from UMS

// crates/cv/src/lib.rs
pub mod sys;
pub mod mat;
pub mod imgproc;
pub mod imgcodecs;
pub mod dnn;
pub mod async_ops;

pub use mat::Mat;
pub use imgcodecs::{imread, imwrite};
```

**Timeline**: 2 weeks

### 2.2 Safe Wrapper Layer (RAII + Send/Sync)
**Deliverable**: `crates/cv/src/mat.rs` + tests

```rust
// crates/cv/src/mat.rs
use std::fmt;

pub struct Mat {
    inner: *mut opencv_sys::cv_Mat,
}

unsafe impl Send for Mat {}
unsafe impl Sync for Mat {}

impl Mat {
    /// Create a new matrix with given dimensions and type
    pub fn new(rows: i32, cols: i32, depth: i32) -> Result<Self, Error> {
        unsafe {
            let ptr = opencv_sys::cv_Mat_create(rows, cols, depth);
            if ptr.is_null() {
                Err(Error::AllocationFailed)
            } else {
                Ok(Mat { inner: ptr })
            }
        }
    }
    
    pub fn from_file(path: &str, flags: i32) -> Result<Self, Error> {
        let c_path = CString::new(path)?;
        unsafe {
            let ptr = opencv_sys::cv_imread(c_path.as_ptr(), flags);
            if ptr.is_null() {
                Err(Error::FileNotFound(path.to_string()))
            } else {
                Ok(Mat { inner: ptr })
            }
        }
    }
    
    pub fn rows(&self) -> i32 {
        unsafe { opencv_sys::cv_Mat_rows(self.inner) }
    }
    
    pub fn cols(&self) -> i32 {
        unsafe { opencv_sys::cv_Mat_cols(self.inner) }
    }
    
    /// Zero-copy pixel data access
    pub fn data(&self) -> &[u8] {
        unsafe {
            let ptr = opencv_sys::cv_Mat_data(self.inner);
            let total = (self.rows() * self.cols() * 3) as usize; // BGR
            std::slice::from_raw_parts(ptr, total)
        }
    }
    
    pub fn mutable_data(&mut self) -> &mut [u8] {
        unsafe {
            let ptr = opencv_sys::cv_Mat_data_mut(self.inner);
            let total = (self.rows() * self.cols() * 3) as usize;
            std::slice::from_raw_parts_mut(ptr, total)
        }
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            opencv_sys::cv_Mat_release(self.inner);
        }
    }
}

impl fmt::Debug for Mat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mat")
            .field("rows", &self.rows())
            .field("cols", &self.cols())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mat_creation() {
        let mat = Mat::new(100, 100, 0).expect("Failed to create Mat");
        assert_eq!(mat.rows(), 100);
        assert_eq!(mat.cols(), 100);
    }
    
    #[test]
    fn test_mat_drop() {
        {
            let _mat = Mat::new(50, 50, 0).expect("Failed to create Mat");
            // RAII should automatically release memory here
        }
        // Verify no memory leak via valgrind/asan
    }
}
```

**Timeline**: 2 weeks

### 2.3 Async Operations & Parallelism
**Deliverable**: `crates/cv-async/src/lib.rs`

```rust
// crates/cv-async/src/lib.rs
use tokio::task;
use rayon::prelude::*;

pub async fn blur_async(
    image: Mat,
    radius: u32,
) -> Result<Mat, Error> {
    // Offload blocking operation to tokio blocking pool
    task::block_in_place(|| {
        blur_sync(&image, radius)
    })
}

pub fn blur_parallel(
    image: &Mat,
    radius: u32,
) -> Result<Mat, Error> {
    // Use rayon for pixel-wise parallelization
    let mut output = Mat::new(image.rows(), image.cols(), image.depth())?;
    
    output.mutable_data()
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, pixel)| {
            // Each thread processes a chunk of pixels
            *pixel = compute_blur_value(image, i, radius);
        });
    
    Ok(output)
}

pub fn image_stream(
    video_path: &str,
) -> impl futures::Stream<Item = Result<Mat, Error>> {
    // Async iterator over video frames
    async_stream::stream! {
        let cap = video::VideoCapture::from_file(video_path)?;
        loop {
            match cap.read_frame().await {
                Ok(frame) => yield Ok(frame),
                Err(e) => {
                    if e.is_eof() { break; }
                    yield Err(e);
                }
            }
        }
    }
}
```

**Timeline**: 2 weeks

### 2.4 UVM Validation Tests
**Deliverable**: `crates/cv-tests/src/lib.rs` (cross-implementation validation)

```rust
// crates/cv-tests/src/lib.rs
use cv::Mat;
use blake3::Hash;

pub struct UvmTestCase {
    pub id: String,
    pub seed: u64,
    pub inputs: TestInputs,
    pub expected_hash: Hash,
}

pub async fn validate_against_canonical(
    test: &UvmTestCase,
) -> Result<ValidationReport, Error> {
    // 1. Load canonical test from UVM spec
    let canonical = load_canonical_test(&test.id)?;
    
    // 2. Run Rust implementation
    let rust_output = match test.id.as_str() {
        "test_imread" => cv::imread(&canonical.inputs.path, 1)?,
        "test_gaussian_blur" => cv::imgproc::gaussian_blur(&canonical.input_image, 5)?,
        _ => return Err(Error::UnknownTest),
    };
    
    // 3. Compute output hash
    let rust_hash = blake3::hash(rust_output.data());
    
    // 4. Compare against canonical
    let matches = rust_hash == test.expected_hash;
    
    Ok(ValidationReport {
        test_id: test.id.clone(),
        passed: matches,
        rust_hash,
        canonical_hash: test.expected_hash,
        runtime_ms: duration.as_millis(),
    })
}

#[tokio::test]
async fn test_rust_vs_canonical() {
    let suite = load_canonical_test_suite().await.unwrap();
    
    for test in suite.tests {
        let report = validate_against_canonical(&test)
            .await
            .expect(&format!("Test {} failed", test.id));
        
        assert!(report.passed, "Hash mismatch for {}", test.id);
    }
}
```

**Timeline**: 2 weeks

---

## Phase 3: Titan Rewrite (Systems Language)

### 3.1 Titan Core Module
**Deliverable**: `crates/titan-opencv/src/core.ti`

```titan
// crates/titan-opencv/src/core.ti
module titan_opencv

use std::simd::{f32x8, u8x32}
use std::alloc::Capability
use uce::compress

pub struct Mat {
    pub width: usize,
    pub height: usize,
    pub channels: u8,
    pub depth: u8,  // CV_8U, CV_32F, etc.
    pub data: Capability<[u8]>,  // Linear pixel buffer
    pub step: usize,  // Row stride in bytes
}

impl Mat {
    pub fn create(
        height: usize,
        width: usize,
        channels: u8,
        depth: u8,
        mem_cap: Capability,
    ) -> Result<Self, Error> {
        let bytes_per_pixel = match depth {
            8 => 1,  // CV_8U
            32 => 4, // CV_32F
            _ => return Err(Error::UnsupportedDepth),
        };
        
        let total_bytes = height * width * channels as usize * bytes_per_pixel;
        
        // Allocate with memory capability
        let data = mem_cap.allocate::<[u8]>(total_bytes)?;
        
        Ok(Mat {
            height,
            width,
            channels,
            depth,
            data,
            step: width * channels as usize * bytes_per_pixel,
        })
    }
    
    pub fn at(&self, row: usize, col: usize, ch: u8) -> u8 {
        let idx = row * self.step + col * self.channels as usize + ch as usize;
        self.data[idx]
    }
    
    pub fn set(&mut self, row: usize, col: usize, ch: u8, val: u8) {
        let idx = row * self.step + col * self.channels as usize + ch as usize;
        self.data[idx] = val;
    }
}

// Export via Universal ABI for cross-language calls
extern "uabi" {
    pub fn cv_create_mat(
        height: usize,
        width: usize,
        channels: u8,
        depth: u8,
    ) -> Result<*mut Mat, i32> {
        let mat = Box::new(Mat::create(height, width, channels, depth, Capability::current())?);
        Ok(Box::into_raw(mat))
    }
    
    pub fn cv_release_mat(mat: *mut Mat) {
        if !mat.is_null() {
            let _ = unsafe { Box::from_raw(mat) };
        }
    }
}
```

**Timeline**: 3 weeks

### 3.2 Titan imgproc Module (SIMD-Optimized)
**Deliverable**: `crates/titan-opencv-imgproc/src/filters.ti`

```titan
// crates/titan-opencv-imgproc/src/filters.ti
module titan_opencv_imgproc

use titan_opencv::Mat
use std::simd::f32x8

/// Gaussian blur with SIMD vectorization
/// Capability requirement: CPU feature AVX2 or SVE
pub fn gaussian_blur(
    input: &Mat,
    kernel_size: usize,
    sigma: f32,
) -> Result<Mat, Error> {
    // 1. Validate inputs
    if kernel_size % 2 == 0 {
        return Err(Error::InvalidKernelSize);
    }
    
    let radius = kernel_size / 2;
    
    // 2. Compute Gaussian kernel
    let kernel = compute_gaussian_kernel(radius, sigma)?;
    
    // 3. Apply convolution with SIMD
    let output = Mat::create(
        input.height,
        input.width,
        input.channels,
        input.depth,
    )?;
    
    // SIMD loop: process 8 pixels at a time
    for y in radius..input.height - radius {
        for x in (radius..input.width - radius).step_by(8) {
            // Load 8 pixels + neighborhood
            let pixels = simd_load_neighborhood(input, x, y, radius)?;
            
            // Apply convolution
            let result = simd_convolve(pixels, &kernel)?;
            
            // Store result
            simd_store(&output, x, y, result)?;
        }
    }
    
    Ok(output)
}

fn simd_load_neighborhood(
    img: &Mat,
    x: usize,
    y: usize,
    radius: usize,
) -> Result<f32x8, Error> {
    // Vectorized neighborhood load
    let mut values = Vec::new();
    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            let px = (x as i32 + dx) as usize;
            let py = (y as i32 + dy) as usize;
            values.push(img.at(py, px, 0) as f32);
        }
    }
    Ok(f32x8::from_array(values.try_into()?))
}

fn simd_convolve(
    pixels: f32x8,
    kernel: &[f32],
) -> Result<f32x8, Error> {
    // Multiply-accumulate: pixels * kernel weights
    let result = f32x8::splat(0.0);
    for (i, &w) in kernel.iter().enumerate() {
        result += pixels[i] * f32x8::splat(w);
    }
    Ok(result)
}

// Formal verification spec (Axiom)
// theorem gaussian_blur_smooth:
//   forall img: Mat, kernel_size: usize, sigma: f32,
//     let result = gaussian_blur(img, kernel_size, sigma) in
//     result.width == img.width ∧ result.height == img.height
```

**Timeline**: 3 weeks

### 3.3 Capability-Based Hardware Access
**Deliverable**: `crates/titan-opencv/src/capabilities.ti`

```titan
module titan_opencv_capabilities

pub enum HardwareCapability {
    CPU { features: [String] },  // avx2, avx512f, sve, neon
    GPU { vendor: String, compute_capability: String },
    TPU { model: String },
}

pub struct GpuKernel {
    pub code: String,  // CUDA/HIP kernel
    pub capability: HardwareCapability,
}

pub fn gaussian_blur_gpu(
    input: &Mat,
    kernel_size: usize,
    sigma: f32,
    gpu_cap: &Capability<GPU>,
) -> Result<Mat, Error> {
    // Only callable if GPU capability is granted
    let kernel = GpuKernel {
        code: CUDA_GAUSSIAN_KERNEL.to_string(),
        capability: HardwareCapability::GPU {
            vendor: "nvidia".to_string(),
            compute_capability: "7.0".to_string(),
        },
    };
    
    // Dispatch to GPU via capability
    gpu_cap.execute_kernel(kernel, input)
}

pub fn gaussian_blur_dispatch(
    input: &Mat,
    kernel_size: usize,
    sigma: f32,
    gpu_opt: Option<&Capability<GPU>>,
) -> Result<Mat, Error> {
    // Smart dispatch: GPU if available, else CPU SIMD
    if let Some(gpu) = gpu_opt {
        gaussian_blur_gpu(input, kernel_size, sigma, gpu)
    } else {
        gaussian_blur(input, kernel_size, sigma)
    }
}
```

**Timeline**: 2 weeks

### 3.4 Axiom Formal Verification
**Deliverable**: `crates/axiom-opencv/proofs/gaussian_blur.ax`

```axiom
-- crates/axiom-opencv/proofs/gaussian_blur.ax
module opencv_proofs

-- Specification of Gaussian blur
theorem gaussian_blur_correctness:
  forall (img : Mat) (kernel_size : Nat) (sigma : Float),
    kernel_size % 2 = 1 ->
    sigma > 0 ->
    let result = gaussian_blur(img, kernel_size, sigma)
    in
    result.width = img.width ∧
    result.height = img.height ∧
    result.channels = img.channels ∧
    -- All output pixels are weighted combinations of input
    (forall (x y : Nat),
      x < result.width -> y < result.height ->
      result.at(x, y) = weighted_average(img, x, y, kernel_size, sigma))
  
proof by induction on kernel_size:
  case 1:
    -- Base case: 1x1 kernel is identity
    skip
  case n:
    -- Inductive case: assume true for n-1, prove for n
    -- Use convolution properties
    sorry
```

**Timeline**: 2 weeks

---

## Phase 4: Sylva Scripting Layer

### 4.1 Sylva Wrapper Module
**Deliverable**: `crates/sylva-opencv/sylva/opencv.sv`

```sylva
// crates/sylva-opencv/sylva/opencv.sv
module opencv

extern "uabi" fn cv_imread(path: String, flags: Int) -> Mat 
  from "titan_opencv::cv_imread"

extern "uabi" fn cv_imwrite(path: String, img: Mat) -> Bool 
  from "titan_opencv::cv_imwrite"

extern "uabi" fn cv_gaussian_blur(img: Mat, radius: Int, sigma: Float) -> Mat 
  from "titan_opencv_imgproc::gaussian_blur"

extern "uabi" fn cv_canny(img: Mat, threshold1: Int, threshold2: Int) -> Mat 
  from "titan_opencv_imgproc::canny"

// High-level Pythonic wrapper
pub type Image = Mat

pub fn imread(path: String) -> Result<Image> {
  match cv_imread(path, 1) {
    null -> Error("Failed to read image")
    img -> Ok(img)
  }
}

pub fn imwrite(path: String, img: Image) -> Bool {
  cv_imwrite(path, img)
}

pub impl Image {
  pub fn blur(self, radius: Int) -> Image {
    cv_gaussian_blur(self, radius, 1.0)
  }
  
  pub fn edges(self, t1: Int, t2: Int) -> Image {
    self.cvt_color(COLOR_BGR2GRAY).canny(t1, t2)
  }
  
  pub fn width(self) -> Int {
    // Query Mat metadata via UABI
    cv_get_mat_width(self)
  }
  
  pub fn height(self) -> Int {
    cv_get_mat_height(self)
  }
}
```

**Timeline**: 2 weeks

### 4.2 Interactive REPL with Time-Travel Debugging
**Deliverable**: `crates/sylva-opencv-repl/src/main.rs`

```rust
// crates/sylva-opencv-repl/src/main.rs
use sylva_runtime::Interpreter;
use std::collections::VecDeque;

pub struct OpenCVRepl {
    interpreter: Interpreter,
    history: VecDeque<ExecutionStep>,
    time_travel_pos: usize,
}

pub struct ExecutionStep {
    pub code: String,
    pub bindings: HashMap<String, Value>,
    pub duration_ms: u64,
}

impl OpenCVRepl {
    pub async fn run(&mut self) -> Result<()> {
        loop {
            print!("cv> ");
            io::stdout().flush()?;
            
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            
            match line.trim() {
                "help" => self.show_help(),
                "back" => self.time_travel_back(),
                "forward" => self.time_travel_forward(),
                code => {
                    let start = Instant::now();
                    
                    match self.interpreter.eval(code).await {
                        Ok(result) => {
                            println!("=> {}", result);
                            self.history.push_back(ExecutionStep {
                                code: code.to_string(),
                                bindings: self.interpreter.bindings(),
                                duration_ms: start.elapsed().as_millis() as u64,
                            });
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        }
    }
    
    fn time_travel_back(&mut self) {
        if self.time_travel_pos > 0 {
            self.time_travel_pos -= 1;
            let step = &self.history[self.time_travel_pos];
            self.interpreter.restore(&step.bindings);
            println!("✓ Rewound to: {}", step.code);
        }
    }
    
    fn show_help(&self) {
        println!("OpenCV Sylva REPL");
        println!("  imread(path)          - Load image");
        println!("  img.blur(5)           - Apply Gaussian blur");
        println!("  img.edges(100, 200)   - Detect edges (Canny)");
        println!("  imwrite('out.jpg', img) - Save image");
        println!("  back                  - Time-travel to previous step");
        println!("  forward               - Time-travel to next step");
    }
}
```

**Timeline**: 2 weeks

---

## Phase 5: Aether Distributed Vision Pipelines

### 5.1 Actor-Based Pipeline
**Deliverable**: `crates/aether-vision/src/actors.ae`

```aether
// crates/aether-vision/src/actors.ae
module vision_pipeline

use aether::actor
use aether::crdt::{Set, Map}
use titan_opencv::{Mat, gaussian_blur}

actor FrameGrabber {
  camera_id: String,
  frame_queue: Queue<Mat>,
  
  on_init(camera_id: String) {
    self.camera_id = camera_id
    self.frame_queue = Queue::new()
  }
  
  msg capture_frame() {
    let frame = open_camera(self.camera_id).read_frame()
    self.frame_queue.push(frame)
    self.send(Preprocessor, process_frame(frame))
  }
}

actor Preprocessor {
  target_width: usize,
  target_height: usize,
  
  msg process_frame(frame: Mat) {
    let resized = frame.resize(self.target_width, self.target_height)
    let normalized = resized.normalize(0.0, 1.0)
    self.send(Detector, detect_objects(normalized))
  }
}

actor Detector {
  model: Model,
  detector_index: Map<ObjectId, DetectionResult>,
  gpu_capability: Option<Capability<GPU>>,
  
  msg detect_objects(frame: Mat) {
    let detections = self.model.infer(&frame, self.gpu_capability)?
    
    for det in detections {
      self.detector_index.insert(det.id, det.clone())
    }
    
    self.send(Tracker, track_objects(detections, frame))
  }
}

actor Tracker {
  active_tracks: CRDT<Set<Track>>,
  
  msg track_objects(detections: Vec<Detection>, frame: Mat) {
    // Update CRDT-based state: merges concurrent updates automatically
    for det in detections {
      let track = Track::from_detection(det)
      self.active_tracks.add(track)
    }
    
    self.send(Logger, log_tracks(self.active_tracks.snapshot()))
  }
}

actor Supervisor {
  supervised_actors: Map<ActorId, ActorStatus>,
  
  on_actor_crash(actor_id: ActorId, error: Error) {
    println!("Actor {} crashed: {}", actor_id, error)
    
    // Restart on different node if possible
    let new_location = self.find_node_with_capability(
      Capability::GPU { 
        vendor: "nvidia", 
        compute_capability: "8.0" 
      }
    )
    
    self.restart_actor(actor_id, new_location)
  }
}

// Orchestration YAML
// crates/aether-vision/pipelines/vision_pipeline.yaml
pipeline:
  name: "real-time-detection"
  actors:
    - name: frame_grabber
      module: FrameGrabber
      config:
        camera_id: "usb:0"
      replicas: 2  # Load-balance across 2 nodes
      
    - name: preprocessor
      module: Preprocessor
      config:
        target_width: 640
        target_height: 480
      depends_on: frame_grabber
      
    - name: detector
      module: Detector
      config:
        model: "yolov8n.onnx"
      depends_on: preprocessor
      capabilities:
        - gpu:cuda_11.0
      
    - name: tracker
      module: Tracker
      depends_on: detector
      crdt_state:
        - field: active_tracks
          type: Set<Track>
      
    - name: supervisor
      module: Supervisor
      health_check:
        interval: 10s
        timeout: 5s
```

**Timeline**: 3 weeks

### 5.2 Integration with TransferDaemon P2P Mesh
**Deliverable**: `crates/aether-vision/src/distribution.rs`

```rust
// crates/aether-vision/src/distribution.rs
use transfer_daemon::mesh::MeshNode;

pub async fn deploy_vision_pipeline(
    pipeline_spec: &PipelineYaml,
    available_nodes: Vec<MeshNode>,
) -> Result<DeploymentHandle, Error> {
    
    // 1. Analyze resource requirements
    let resource_plan = analyze_pipeline(&pipeline_spec)?;
    
    // 2. Allocate to nodes based on capabilities
    let allocation = allocate_actors_to_nodes(&available_nodes, &resource_plan)?;
    
    // 3. Start actors on assigned nodes via TransferDaemon
    for (actor_spec, node) in allocation {
        let code = fetch_actor_code(&actor_spec.module)?;
        node.spawn_actor(code, actor_spec.config).await?;
    }
    
    // 4. Wire actor communication through TransferDaemon mesh
    for (src, dst) in pipeline_spec.actor_connections() {
        let src_node = allocation.get_node(&src)?;
        let dst_node = allocation.get_node(&dst)?;
        
        src_node.register_route(&src, &dst, dst_node).await?;
    }
    
    Ok(DeploymentHandle {
        pipeline_id: pipeline_spec.name.clone(),
        nodes: available_nodes,
    })
}
```

**Timeline**: 2 weeks

---

## Phase 6: Axiom Proof Engineering

### 6.1 Formal Specifications of Image Processing Functions
**Deliverable**: `crates/axiom-opencv-proofs/specs/image_ops.ax`

```axiom
-- Specification of image resize operation
theorem resize_preserves_aspect:
  forall (img : Mat) (new_width new_height : Nat),
    new_width > 0 -> new_height > 0 ->
    let result = resize(img, new_width, new_height)
    in
    result.width = new_width ∧
    result.height = new_height ∧
    (aspect_ratio(result) = aspect_ratio(img) ∨ img.width = 0)

-- Specification of color space conversion
theorem cvt_color_bgr2gray_spec:
  forall (img : Mat),
    img.channels = 3 ->
    let result = cvt_color(img, COLOR_BGR2GRAY)
    in
    result.channels = 1 ∧
    result.width = img.width ∧
    result.height = img.height ∧
    forall (x y : Nat),
      x < result.width -> y < result.height ->
      result.at(x, y) = 
        (0.114 * img.at(x, y, BLUE) +
         0.587 * img.at(x, y, GREEN) +
         0.299 * img.at(x, y, RED))

-- Verified histogram computation
def histogram_verified(img : Mat) : Vec<Int> :=
  let counts : Vec<Int> := vec_of_zeros(256)
  for (x : Nat) (y : Nat) in img:
    let pixel := img.at(x, y)
    counts[pixel] := counts[pixel] + 1
  counts

theorem histogram_completeness:
  forall (img : Mat),
    let hist := histogram_verified(img)
    in
    (fold (fun sum count => sum + count) 0 hist) = 
    (img.width * img.height)
```

**Timeline**: 2 weeks

### 6.2 Proof-Carrying Code Extraction
**Deliverable**: Verified binaries with proof certificates

```rust
// crates/axiom-opencv/src/extract.rs
pub struct ProofCarryingBinary {
    pub code: Vec<u8>,
    pub proof_certificate: Hash,
    pub theorem_name: String,
}

pub fn extract_to_titan(
    axiom_proof: &AxiomProof,
) -> Result<ProofCarryingBinary, Error> {
    // 1. Compile Axiom proof to Titan IR
    let titan_ir = axiom_proof.compile_to_titan()?;
    
    // 2. Compile Titan IR to native code
    let code = titan_compiler::compile(titan_ir)?;
    
    // 3. Generate proof certificate
    let cert_hash = blake3::hash(&axiom_proof.proof_text);
    
    Ok(ProofCarryingBinary {
        code,
        proof_certificate: cert_hash,
        theorem_name: axiom_proof.theorem_name.clone(),
    })
}
```

**Timeline**: 2 weeks

---

## Phase 7: Cross-Language Validation & Ecosystem Rollout

### 7.1 Universal Validation Mesh (UVM) Integration
**Deliverable**: Unified test harness with cross-language comparison

```rust
// crates/cv-uvm/src/harness.rs
pub struct UvmHarness {
    canonical_suite: Vec<CanonicalTest>,
    implementations: HashMap<String, Box<dyn CvImplementation>>,
}

#[async_trait::async_trait]
pub trait CvImplementation: Send + Sync {
    async fn blur(&self, img: &[u8], radius: u32) -> Result<Vec<u8>>;
    async fn resize(&self, img: &[u8], width: u32, height: u32) -> Result<Vec<u8>>;
}

impl UvmHarness {
    pub async fn validate_all(
        &self,
    ) -> Result<ValidationReport, Error> {
        let mut results = Vec::new();
        
        for test in &self.canonical_suite {
            // Run all implementations
            let mut impl_outputs = Vec::new();
            
            for (name, impl_) in &self.implementations {
                let output = match test.operation.as_str() {
                    "blur" => impl_.blur(&test.inputs.image, test.inputs.radius).await?,
                    "resize" => impl_.resize(&test.inputs.image, test.inputs.width, test.inputs.height).await?,
                    _ => continue,
                };
                
                let hash = blake3::hash(&output);
                impl_outputs.push((name.clone(), hash));
            }
            
            // Verify all hashes match canonical
            let canonical_hash = test.expected_hash;
            let mut all_match = true;
            
            for (name, hash) in &impl_outputs {
                if *hash != canonical_hash {
                    all_match = false;
                    eprintln!("❌ {} failed for test {}", name, test.id);
                }
            }
            
            results.push(TestResult {
                test_id: test.id.clone(),
                passed: all_match,
                impl_hashes: impl_outputs,
            });
        }
        
        Ok(ValidationReport {
            total: results.len(),
            passed: results.iter().filter(|r| r.passed).count(),
            results,
        })
    }
}

#[tokio::test]
async fn test_cross_language_fidelity() {
    let mut harness = UvmHarness::new();
    
    // Register all implementations
    harness.register("cpp-native", Box::new(CppNativeImpl::new())?);
    harness.register("rust", Box::new(RustImpl::new())?);
    harness.register("titan", Box::new(TitanImpl::new())?);
    harness.register("sylva", Box::new(SylvaImpl::new())?);
    harness.register("axiom-extracted", Box::new(AxiomImpl::new())?);
    
    let report = harness.validate_all().await.unwrap();
    
    assert_eq!(report.passed, report.total, 
               "Some implementations failed cross-language validation");
}
```

**Timeline**: 3 weeks

### 7.2 Performance Benchmarking Dashboard
**Deliverable**: `observability` dashboard showing performance across all implementations

```yaml
# crates/cv-uvm/benchmarks/benchmark_suite.yaml
benchmarks:
  - name: "Image Classification (ImageNet)"
    image: "imagenet_sample.jpg"
    models:
      - "resnet50.onnx"
      - "mobilenet_v2.onnx"
    metrics:
      - latency_ms
      - throughput_fps
      - peak_memory_mb
    targets:
      - impl: "cpp-native"
        expected_baseline: 1.0
      - impl: "rust"
        expected_vs_baseline: 0.95
      - impl: "titan"
        expected_vs_baseline: 1.0  # Same or better
      - impl: "sylva"
        expected_vs_baseline: 0.85
      
  - name: "Object Detection (COCO YOLO)"
    models:
      - "yolov8n.onnx"
      - "yolov8m.onnx"
    metrics:
      - fps_on_rtx3090
      - fps_on_jetson_orin
      
  - name: "Video Stabilization (Real-time)"
    duration_seconds: 60
    metrics:
      - latency_ms
      - throughput_fps

# Automatically published to observability dashboard
# Each implementation tested in parallel
# Regressions >1% are flagged for investigation
```

**Timeline**: 2 weeks

### 7.3 Documentation & Developer Guides
**Deliverable**: Comprehensive guides for each language

```markdown
# docs/OPENCV5_USAGE_GUIDE.md

## Using OpenCV 5 from Rust

### Installation
\`\`\`bash
build module install opencv-core:5.0.0 --lang rust
\`\`\`

### Basic Usage
\`\`\`rust
use cv::Mat;
use cv::imgproc;

fn main() {
    let img = cv::imread("photo.jpg").expect("Failed to load image");
    let blurred = imgproc::gaussian_blur(&img, 5).expect("Blur failed");
    cv::imwrite("blurred.jpg", &blurred).ok();
}
\`\`\`

---

## Using OpenCV 5 from Sylva

### Interactive REPL
\`\`\`bash
sylva-opencv-repl
cv> let img = opencv.imread("photo.jpg")
cv> let blurred = img.blur(5)
cv> opencv.imwrite("output.jpg", blurred)
\`\`\`

---

## Using OpenCV 5 from Aether (Distributed)

### Deployment YAML
\`\`\`yaml
pipeline:
  name: object-detection-cluster
  actors:
    - name: camera-feed
      replicas: 4
    - name: detector
      capabilities: [gpu:cuda_11.0]
      replicas: 8
\`\`\`

### Deploy
\`\`\`bash
build pipeline deploy object-detection-cluster.yaml
\`\`\`

---

## Security: Capability-Based Access

### Example: GPU-Optional Processing
\`\`\`titan
pub fn process_with_optional_gpu(
    img: &Mat,
    gpu_cap: Option<&Capability<GPU>>,
) -> Result<Mat, Error> {
    if let Some(gpu) = gpu_cap {
        // Only executes if caller has GPU capability
        gaussian_blur_gpu(img, 5, gpu)
    } else {
        // Fallback to CPU SIMD
        gaussian_blur_cpu(img, 5)
    }
}
\`\`\`

The `Capability` system ensures:
- ✅ All hardware access is explicit and auditable
- ✅ No hidden device allocation or overallocation
- ✅ Safe privilege escalation via formal capabilities
- ✅ Provable resource bounds via Axiom theorems
```

**Timeline**: 3 weeks

### 7.4 UMS Discovery & Auto-Installation
**Deliverable**: `build` CLI integration

```bash
# Search for OpenCV modules
$ build module search opencv
  opencv-core:5.0.0       [lang: rust, titan, sylva, aether]
  opencv-dnn:5.0.0        [lang: rust, titan, sylva, aether]
  opencv-imgproc:5.0.0    [lang: rust, titan, sylva, aether]

# Install for Rust
$ build module install opencv-core:5.0.0 --lang rust
  ✓ Fetching opencv-core from UMS...
  ✓ Verifying signature (5-of-7 Council)...
  ✓ Unpacking (zstd decompression)...
  ✓ Installing to ~/.bonsai/modules/opencv-core/
  ✓ Available for import in your code

# Install for Titan
$ build module install opencv-core:5.0.0 --lang titan
  ✓ Fetching opencv-core...
  ✓ Installed to crates/cv-deps/opencv-core/

# Use in code
# Rust: use opencv::{imread, imwrite};
# Titan: extern "uabi" fn cv_imread(...) from "opencv-core::cv_imread"
# Sylva: let img = opencv.imread("path")
```

**Timeline**: 2 weeks

---

## Implementation Timeline Summary

| Phase | Duration | Dependencies | Key Deliverables |
|-------|----------|--------------|------------------|
| **0** | 2 weeks | — | API spec, canonical tests |
| **1** | 4 weeks | Phase 0 | Native UMS modules, multi-arch builds |
| **2** | 6 weeks | Phase 1 | Rust crate, RAII wrappers, async ops |
| **3** | 5 weeks | Phase 1, 2 | Titan rewrite, SIMD, capabilities |
| **4** | 4 weeks | Phase 3 | Sylva wrapper, REPL, JIT |
| **5** | 5 weeks | Phase 3, 4 | Aether actors, P2P distribution |
| **6** | 4 weeks | Phase 3, 5 | Axiom proofs, proof-carrying code |
| **7** | 5 weeks | All phases | UVM validation, benchmarks, docs |

**Total: 35 weeks (~8 months)** with sequential phases.  
**Parallel opportunities**: Phases 4-6 can partially overlap once Phase 3 is stable.

---

## Risk Mitigation

### Technical Risks
| Risk | Probability | Mitigation |
|------|-------------|-----------|
| OpenCV 5 API instability | Low | Track upstream repo, pin to release tag |
| SIMD performance variance across targets | Medium | Maintain SIMD fallback paths, benchmark early |
| Proof verification complexity (Axiom) | Medium | Start with simple specs (histogram, resize), scale up |
| GPU kernel compatibility | High | Focus on CUDA first, then HIP/OpenCL |

### Timeline Risks
| Risk | Mitigation |
|------|-----------|
| Titan compiler bugs | Fallback to C++ shim layer during Phase 3 |
| Council signing delays | Parallelize Phase 1 with unsigned pre-releases |
| Cross-language test failures | Implement UVM harness early (Week 3) |

---

## Success Criteria

✅ **Phase-by-phase validation**:
- Phase 1: Native modules published to UMS, signed by Council
- Phase 2: Rust tests pass canonical UVM suite with 0 hash mismatches
- Phase 3: Titan implementations match C++ output exactly (via UVM)
- Phase 4: Sylva REPL runs example scripts at 90%+ of Rust speed
- Phase 5: Actor pipeline distributes across 10+ nodes without errors
- Phase 6: Axiom proofs verified by independent checker
- Phase 7: All implementations show <5% performance variance

✅ **Production readiness**:
- Zero undefined behavior (verified by Miri + ASan/TSan)
- 100% capability-based access (auditable via Axiom)
- <1% regression from C++ baseline on standard benchmarks
- <100ms module hot-reload latency
- Deterministic cross-language outputs (bit-identical hashes)

---

## Next Steps (Immediate)

1. **Week 1**: Create GitHub issue for OpenCV 5 integration, assign phases to team
2. **Week 1-2**: Run Phase 0 (API audit, test extraction)
3. **Week 3**: Begin Phase 1 (CMake build orchestration)
4. **Ongoing**: Set up UVM test harness in parallel (Phase 7 preparation)
5. **Bi-weekly reviews**: Demo phase completions to architecture council

---

**Prepared by**: Claude Code AI  
**Date**: 2026-06-06  
**Status**: Ready for architecture review and team assignment
