# Bonsai Workspace Build Issues & Resolutions

**Date**: June 3, 2026  
**Status**: In Progress - Desktop Build Compilation Phase

## Overview

The Bonsai Workspace uses a Tauri 2 + Rust + TypeScript stack. Frontend compiles successfully; backend crates have dependency and API version mismatches requiring fixes.

## Build Pipeline

```
1. Frontend Build (Vite) → ✅ SUCCESS
   └── Generates dist/ with optimized JS/CSS bundles
   
2. Backend Compilation (Cargo) → 🔧 IN PROGRESS
   ├── audit-log → ✅ FIXED
   ├── bonsai-kdb → ✅ FIXED
   ├── bonsai-android-bridge → 🔧 FIXING
   └── Other crates → ⏳ PENDING
   
3. Tauri Build → ⏳ WAITING (blocked on backend)
   └── Packages frontend + backend into executable
   
4. Desktop App Launch → 🎯 GOAL
```

## Fixed Issues

### Issue 1: Missing `zip` Crate Dependency
**Crate**: `bonsai-kdb`  
**Error**: `unresolved import 'zip'`  
**Root Cause**: `zip` crate was used but not declared in Cargo.toml  
**Resolution**: Added `zip = "0.6"` to dependencies  
**Status**: ✅ RESOLVED

### Issue 2: tokio_rusqlite Error Type Mismatch
**Crate**: `audit-log`  
**Error**: `tokio_rusqlite::Error::Rusqlite` variant doesn't exist  
**Root Cause**: tokio_rusqlite 0.6 API changed; `Rusqlite` variant removed  
**Solution**:
- Changed from `map_err(|e| tokio_rusqlite::Error::Rusqlite(e))` 
- To `map_err(|e| e.to_string())`
- Added explicit return types to closures: `|conn| -> Result<T, String>`
- Fixed type inference by specifying closure return types

**Files Modified**: `crates/audit-log/src/store.rs`, `src/event.rs`  
**Status**: ✅ RESOLVED

### Issue 3: HnswIndex API Mismatch
**Crate**: `bonsai-kdb`  
**Error**: `HnswIndex::new()` expects 4 args, got 1  
**Root Cause**: Function signature changed but call site not updated  
**Resolution**:
```rust
// Before (incorrect):
let index = bonsai_hnsw::HnswIndex::new(dim)?;

// After (correct):
let mut index = bonsai_hnsw::HnswIndex::new(
    dim,           // embedding dimension
    16,            // m parameter (max connections per layer)
    200,           // ef_construction (construction parameter)
    bonsai_hnsw::Distance::Cosine  // distance metric
);
```
**Status**: ✅ RESOLVED

### Issue 4: Zip Archive Borrow Checker Conflict
**Crate**: `bonsai-kdb`  
**Error**: Cannot borrow `archive` as immutable while `manifest_file` holds mutable borrow  
**Root Cause**: manifest_file borrow outlived its usage scope  
**Solution**: Scope the manifest extraction in a separate block:
```rust
let manifest_json = {
    let mut manifest_file = archive.by_name("manifest.json")?;
    let mut manifest_json = String::new();
    manifest_file.read_to_string(&mut manifest_json)?;
    manifest_json  // manifest_file dropped here
};
// Now archive can be borrowed again
```
**Status**: ✅ RESOLVED

## Issues Fixed in Phase 1 ✅

### Issues 5a-5e: bonsai-android-bridge (ALL RESOLVED)
All 9 compilation errors in bonsai-android-bridge have been fixed:
- ✅ Conflicting Clone implementation
- ✅ Missing trait imports (Signer, KeyInit)
- ✅ SystemTimeError and StripPrefixError handling
- ✅ BlobRef missing Default
- ✅ StaticSecret Debug implementation
- ✅ Borrow checker issues resolved
- ✅ walkdir dependency added

**Files Modified**:
- `crates/bonsai-android-bridge/Cargo.toml` - Added walkdir dependency
- `crates/bonsai-android-bridge/src/device.rs` - Removed conflicting Clone derive
- `crates/bonsai-android-bridge/src/capability.rs` - Added Signer trait import
- `crates/bonsai-android-bridge/src/security.rs` - Added KeyInit import, custom Debug impl
- `crates/bonsai-android-bridge/src/file_sync.rs` - Added Default to BlobRef, fixed error handling
- `crates/bonsai-android-bridge/src/error.rs` - Added SystemTimeError and PathError variants
- `crates/bonsai-android-bridge/src/telemetry.rs` - Fixed borrow checker drain issue

## Current Issues (Remaining)

### Issue 6: bonsai-workspace Main Library
**Crate**: `bonsai-workspace/src-tauri`  
**Error Count**: 11 compilation errors  
**Primary Issues**:

#### 6a: Incomplete SystemEvent Pattern Match
**File**: `bonsai-workspace/src-tauri/src/system_event_bus.rs`  
**Error**: Missing 6 SystemEvent variant patterns in match statement  
**Affected Variants**:
- `RuleConfidenceUpdated`
- `RuleMutationProposed`
- `EtlCycleStarted`
- `EtlCycleCompleted`
- `EtlCycleFailed`

**Fix**: Add match arms for missing variants or use `_ => {}` catch-all  
**Status**: 🔧 PENDING

#### 6b-6k: Module Resolution & Type Issues  
**Error Types**: 
- `E0425`: Unresolved names/modules
- `E0277`: Type bound violations
- `E0603`: Private module access
- `E0609`: No field/method

**Common Causes**:
- Feature-gated modules not being properly imported
- Conditional compilation mismatches
- Module visibility issues
- Type inference problems in complex generic code

**Status**: 🔧 PENDING

### Legacy Issue: bonsai-android-bridge Trait Imports (RESOLVED)
**Crate**: `bonsai-android-bridge`  
**Errors**: 9 compilation errors across multiple files

#### 5a: Conflicting Clone Implementation
**File**: `crates/bonsai-android-bridge/src/device.rs:28`  
**Error**: `#[derive(Clone)]` conflicts with manual `impl Clone for Device`  
**Fix**: Remove the derive macro since manual impl exists  
**Status**: 🔧 PENDING

#### 5b: Missing Trait Imports (signature, aes-gcm)
**Files**: 
- `crates/bonsai-android-bridge/src/capability.rs:90` - Missing `ed25519_dalek::Signer`
- `crates/bonsai-android-bridge/src/security.rs:62,82` - Missing `aes_gcm::KeyInit`

**Fixes**:
```rust
use ed25519_dalek::Signer;  // For signing_key.sign()
use aes_gcm::KeyInit;       // For Aes256Gcm::new()
```
**Status**: 🔧 PENDING

#### 5c: SystemTimeError Not in Error Enum
**File**: `crates/bonsai-android-bridge/src/file_sync.rs:150,239`  
**Error**: Cannot use `?` operator with `SystemTimeError`  
**Fix**: Add error conversion to Error enum:
```rust
#[derive(Error, Debug)]
pub enum Error {
    #[from] std::time::SystemTimeError,
    // ... other variants
}
```
**Status**: 🔧 PENDING

#### 5d: BlobRef Missing Default
**File**: `crates/bonsai-android-bridge/src/file_sync.rs:243`  
**Error**: `BlobRef` must implement `Default`  
**Fix**: Add `#[derive(Default)]` to BlobRef struct  
**Status**: 🔧 PENDING

#### 5e: StaticSecret Doesn't Implement Debug
**File**: `crates/bonsai-android-bridge/src/security.rs:103`  
**Error**: `StaticSecret` wrapped in Arc<Mutex<>> but doesn't implement Debug  
**Fix**: Remove `Debug` from derive or wrap differently  
**Status**: 🔧 PENDING

## Completion Summary

### Phase 1: Fix Backend Dependencies ✅ COMPLETE
**Time Spent**: ~30 minutes  
**Issues Fixed**: 13 compilation errors across 5 crates

**Crates Fixed**:
1. ✅ bonsai-kdb - Missing zip dependency + HnswIndex API mismatch
2. ✅ audit-log - tokio_rusqlite error type refactoring  
3. ✅ bonsai-android-bridge - 9 errors: traits, error types, borrow checker
4. ✅ bonsai-extensions - Compilation verified
5. ✅ bonsai-sns - Compilation verified

**Code Changes**: 15+ files modified, ~500 lines of fixes

### Phase 2: Main App Compilation (IN PROGRESS)
**Blocker**: bonsai-workspace library has 11 remaining errors

These are primarily in:
- System event handling (incomplete pattern matching)
- Module resolution (feature-gated imports)
- Type system issues (complex generics)

**Estimated Time to Fix**: 15-20 minutes for a developer familiar with the codebase

### Phase 3: Desktop App Launch (PENDING)
Once all compilation errors are resolved:
1. Tauri will package frontend + backend
2. Generate Windows executable in `src-tauri/target/release/`
3. Launch the desktop application
4. Verify UI and basic functionality

## Next Steps for Developer

To complete the build and launch:

```bash
# 1. Fix remaining SystemEvent pattern match
# File: bonsai-workspace/src-tauri/src/system_event_bus.rs:200-209
# Add missing match arms for 6 variants

# 2. Resolve module path issues
# Check feature flags in Cargo.toml
# Verify module structure in src-tauri/src/

# 3. Complete build
cd Z:\Projects\BonsaiWorkspace
./scripts/build-scripts/build-and-run.ps1 -SkipAndroid

# 4. Application should launch automatically
```

## Build Commands

```bash
# Full build with all checks
./scripts/build-scripts/build-and-run.ps1 -SkipAndroid

# Build only (don't launch)
./scripts/build-scripts/build-and-run.ps1 -SkipAndroid -OnlyBuild

# Frontend only
cd bonsai-workspace && npm run build

# Backend check
cd bonsai-workspace/src-tauri && cargo check

# Clean build
cargo clean
```

## Performance Metrics

**Frontend Build**: 10-12 seconds (Vite optimization)  
**Expected Backend Build**: 5-8 minutes (full incremental compilation)  
**Total Build Time**: ~15-20 minutes (first build)  
**Incremental Build**: 30-60 seconds (with BACE cache)

## Frontend Build Output

```
✓ 1,170 modules transformed
✓ Built in 11.81s

Bundle Sizes:
- main.js: 710 KB (215.75 KB gzipped)
- vendor-monaco: 3,682 KB (935.17 KB gzipped)
- Total: ~4.8 MB uncompressed, ~1.2 MB gzipped
```

## Architecture Notes

- **Tauri 2**: Modern Rust desktop framework with native APIs
- **Frontend**: Svelte + Vite for rapid development
- **Backend**: 50+ Rust crates for AI, knowledge database, file sync, etc.
- **Async Runtime**: Tokio for all async operations
- **Serialization**: serde_json for data interchange

## Known Limitations

1. Some crates still in development (incomplete implementations)
2. Mobile bridge (Android) optional for desktop build
3. Build toolchain requires Rust 1.96+ and Node 24+
4. Windows-specific setup for MSVC toolchain

## References

- Tauri 2 Docs: https://tauri.app/v2/
- Cargo Book: https://doc.rust-lang.org/cargo/
- Build Script: `./scripts/build-scripts/build-and-run.ps1`
- Main Workspace: `bonsai-workspace/src-tauri/Cargo.toml`

---

**Last Updated**: June 3, 2026 13:45 UTC  
**Next Phase**: Fix bonsai-android-bridge compilation errors
