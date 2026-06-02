# Mobile Ecosystem Phase 2-3 Integration Checklist

## Pre-Build Verification

- [ ] **Rust Setup**
  - [ ] `rustup target add aarch64-linux-android` installed
  - [ ] `rustup target add x86_64-linux-android` installed
  - [ ] Android NDK installed (minimum r23)
  - [ ] `.cargo/config.toml` has correct Android linker paths

- [ ] **Code Review**
  - [ ] `crates/bonsai-mobile-ffi/src/llm_jni.rs` created (600+ LOC)
  - [ ] `crates/bonsai-mobile-ffi/src/lib.rs` exports `mod llm_jni`
  - [ ] `crates/bonsai-mobile-ffi/Cargo.toml` includes `uuid` and `lazy_static` deps
  - [ ] `BonsaiService.kt` updated with Phase 2-3 methods
  - [ ] `IBonsaiService.aidl` updated with new LLM methods
  - [ ] `IBonsaiCallback.aidl` unchanged (backward compatible)

## Compilation

### Step 1: Build Rust Library

```bash
cd z:\Projects\BonsaiWorkspace\crates\bonsai-mobile-ffi

# Debug build for testing
cargo build --target aarch64-linux-android

# Release build for production
cargo build --target aarch64-linux-android --release

# Check for warnings
cargo clippy --target aarch64-linux-android --all-targets
```

**Success Criteria:**
- [ ] No compilation errors
- [ ] No `unwrap()` in release build error messages
- [ ] `libBonsaiAndroidLLM.so` or similar generated in `target/aarch64-linux-android/release/`

### Step 2: Copy Native Libraries

```bash
# Create JNI directory structure
mkdir -p z:\Projects\BonsaiWorkspace\bonsai-buddy-android\library-bonsai-shared\src\main\jniLibs\arm64-v8a
mkdir -p z:\Projects\BonsaiWorkspace\bonsai-buddy-android\library-bonsai-shared\src\main\jniLibs\x86_64

# Copy ARM64 library (for actual devices)
cp z:\Projects\BonsaiWorkspace\crates\bonsai-mobile-ffi\target\aarch64-linux-android\release\libbonsai_mobile_ffi.so \
   z:\Projects\BonsaiWorkspace\bonsai-buddy-android\library-bonsai-shared\src\main\jniLibs\arm64-v8a\libbonsai_android_llm.so

# For x86_64 (emulator)
cargo build --target x86_64-linux-android --release
cp z:\Projects\BonsaiWorkspace\crates\bonsai-mobile-ffi\target\x86_64-linux-android\release\libbonsai_mobile_ffi.so \
   z:\Projects\BonsaiWorkspace\bonsai-buddy-android\library-bonsai-shared\src\main\jniLibs\x86_64\libbonsai_android_llm.so
```

**Success Criteria:**
- [ ] `.so` files exist in correct directories
- [ ] Files are not zero-length
- [ ] Library symbols are exported (`nm` shows Java_ai_bonsai_* symbols)

### Step 3: Gradle Build

```bash
cd z:\Projects\BonsaiWorkspace\bonsai-buddy-android

# Clean rebuild
./gradlew clean build

# Or build library only
./gradlew :library-bonsai-shared:build
```

**Success Criteria:**
- [ ] `gradlew build` completes without errors
- [ ] AIDL files compile (in build/generated/aidl)
- [ ] Kotlin compiles with no errors
- [ ] AAR artifact created

## Testing

### Unit Tests

```bash
# Test Rust code
cd crates/bonsai-mobile-ffi
cargo test llm_jni

# Test expected behavior
# ✓ Model state created on init
# ✓ Session tracking works
# ✓ Error responses valid JSON
# ✓ Thread safety with concurrent access
```

**Success Criteria:**
- [ ] All tests pass
- [ ] No panics or unwraps triggered
- [ ] Thread race conditions tested

### Integration Tests (Android)

```bash
# Deploy to emulator
adb install -r app-debug.apk

# Run service binding test
adb shell am startservice ai.bonsai.buddy/.MainActivity

# Check logcat
adb logcat | grep "BonsaiService\|BonsaiLLM"
```

**Success Criteria:**
- [ ] Service starts without crashes
- [ ] Log tag "BonsaiService" appears
- [ ] Log tag "BonsaiLLM" appears (from Rust)
- [ ] No UnsatisfiedLinkError

### Manual Testing

```kotlin
// In test Activity
val intent = Intent(this, BonsaiService::class.java)
bindService(intent, object : ServiceConnection {
    override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
        val bonsai = IBonsaiService.Stub.asInterface(service)
        
        // Test 1: Initialize model
        val response = bonsai.nativeInitModel("/sdcard/test.gguf")
        assert(response.contains("\"status\":\"ok\""))
        println("✓ nativeInitModel works")
        
        // Test 2: Get available models
        val models = bonsai.nativeGetAvailableModels()
        println("✓ nativeGetAvailableModels: ${models.size} models")
        
        // Test 3: Chat
        val messages = """[{"role":"user","content":"Hi"}]"""
        val chatResponse = bonsai.nativeChat("model-id", messages, 0.7f, 100)
        assert(chatResponse.contains("\"status\""))
        println("✓ nativeChat works")
    }
    
    override fun onServiceDisconnected(name: ComponentName?) {}
}, Context.BIND_AUTO_CREATE)
```

**Test Cases:**

- [ ] **T1**: Service binds without error
- [ ] **T2**: nativeInitModel with valid path → JSON response
- [ ] **T3**: nativeInitModel with invalid path → error JSON
- [ ] **T4**: nativeInitModel returns valid model_id (UUID format)
- [ ] **T5**: nativeChat with valid model_id → response
- [ ] **T6**: nativeChat with invalid model_id → error JSON
- [ ] **T7**: nativeChatStream invokes callbacks
- [ ] **T8**: nativeUnloadModel removes model
- [ ] **T9**: nativeGetAvailableModels lists .gguf files
- [ ] **T10**: Service survives configuration changes
- [ ] **T11**: Multiple concurrent calls don't crash
- [ ] **T12**: Unloaded model cannot be used

## Code Quality

### Rust Linting

```bash
cd crates/bonsai-mobile-ffi

# Check for warnings
cargo clippy --target aarch64-linux-android --all-targets -- -D warnings

# Format check
cargo fmt -- --check

# Comprehensive review
cargo clippy --all-targets --all-features -- -D warnings
```

**Must Fix:**
- [ ] No `unwrap()` in error paths
- [ ] No `panic!()` in FFI code
- [ ] All logging statements present
- [ ] Docstring comments on all public functions

### Kotlin Linting

```bash
# Detekt linting
./gradlew detekt

# Lint checks
./gradlew lint

# Format check (ktlint)
./gradlew ktlintCheck
```

**Must Fix:**
- [ ] No bare `try {} catch { }` without logging
- [ ] All callback methods try-catch wrapped
- [ ] Proper null safety (no !! unless justified)
- [ ] Resource cleanup in onDestroy()

## Documentation

- [ ] `docs/21-MOBILE-ECOSYSTEM-PHASE-2-3.md` created
  - [ ] Architecture diagram present
  - [ ] All JNI function signatures documented
  - [ ] Error handling patterns explained
  - [ ] Integration guide complete
  - [ ] Troubleshooting section included

- [ ] Code comments
  - [ ] Each JNI function has `#[doc]` and `///` comments
  - [ ] Complex logic explained
  - [ ] Safety preconditions noted
  - [ ] Example usage in comments

- [ ] README updates
  - [ ] Phase 2-3 mentioned in project overview
  - [ ] Link to integration guide
  - [ ] Quick start example

## Performance Baseline

- [ ] Model initialization latency measured
- [ ] First token latency measured
- [ ] Token generation rate measured
- [ ] Memory usage profiled
- [ ] No memory leaks detected (valgrind/AddressSanitizer)

### Baseline Targets

```
Model Init:     < 2000ms
First Token:    < 500ms
Token Rate:     > 5 tokens/sec
Memory (Model): < 500MB
Memory (Session): < 10MB
```

## Deployment

### Pre-Release Checklist

- [ ] All tests passing
- [ ] No known crashes
- [ ] Memory leak testing complete
- [ ] Performance benchmarks recorded
- [ ] Documentation reviewed
- [ ] Security audit complete (no buffer overflows in JNI)

### Release Commit

```bash
git add crates/bonsai-mobile-ffi/src/llm_jni.rs
git add crates/bonsai-mobile-ffi/src/lib.rs
git add crates/bonsai-mobile-ffi/Cargo.toml
git add bonsai-buddy-android/library-bonsai-shared/src/main/java/ai/bonsai/shared/service/BonsaiService.kt
git add bonsai-buddy-android/library-bonsai-shared/src/main/aidl/ai/bonsai/shared/IBonsaiService.aidl
git add docs/21-MOBILE-ECOSYSTEM-PHASE-2-3.md
git add docs/22-MOBILE-INTEGRATION-CHECKLIST.md

git commit -m "feat: Complete Phase 2-3 Mobile Ecosystem with Rust JNI + production BonsaiService"
```

### Version Tagging

```bash
git tag -a v0.2.3-mobile-ecosystem \
  -m "Mobile Ecosystem Phase 2-3: LLM inference + streaming chat + model management"
git push origin v0.2.3-mobile-ecosystem
```

## Post-Deployment

- [ ] Monitor crashes via Firebase Crashlytics
- [ ] Monitor performance via Firebase Performance
- [ ] Collect user feedback
- [ ] Patch security issues immediately
- [ ] Plan Phase 4 enhancements (real llama.cpp integration)

## Known Limitations

### Phase 2-3 Current Status

- [x] JNI layer complete
- [x] Service binding complete
- [x] Error handling complete
- [x] Thread safety complete
- [ ] Real inference engine (placeholder only)
- [ ] Knowledge base integration (future)
- [ ] Model quantization (future)
- [ ] GPU acceleration (future)

### Placeholder Behavior

Currently, `nativeChat` and `nativeChatStream` return placeholder responses:

```rust
// In llm_jni.rs
fn generate_response(model_id: &str, ...) -> String {
    format!("I can help you with the Bonsai Ecosystem. [LLM inference placeholder]")
}

fn generate_token_stream(prompt: &str, ...) -> Vec<String> {
    vec!["I", " can", " help", " you", ...]
}
```

**To Enable Real Inference (Phase 4):**

1. Add `llama-cpp` crate to Cargo.toml
2. Replace `generate_response()` with actual model inference
3. Update `generate_token_stream()` to stream from real model
4. Benchmark and optimize for target devices

## Support Contacts

- **Architecture**: See CLAUDE.md
- **Android Issues**: Check bonsai-buddy-android README
- **JNI Binding Issues**: See crates/bonsai-mobile-ffi docs
- **Performance**: Profile with Android Studio Profiler

