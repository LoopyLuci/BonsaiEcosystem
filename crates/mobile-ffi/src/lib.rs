//! Bonsai Mobile FFI - Hardware-Accelerated Video Decoding for Android
//!
//! This crate provides production-grade JNI bindings for Android's MediaCodec API,
//! enabling zero-copy hardware-accelerated H.264/H.265 video decoding with <10ms latency.
//!
//! ## Architecture
//!
//! - **Decoder Management**: Safe JNI wrapping around MediaCodec decoder instances
//! - **Buffer Management**: Zero-copy frame buffers with lifetime-safe access patterns
//! - **Metrics Collection**: Real-time decode latency, FPS, and frame dimension tracking
//! - **Error Handling**: Comprehensive error reporting with meaningful diagnostics
//!
//! ## Performance Targets
//!
//! - Decode latency: <10ms per frame (target for Redmi Note 12 Pro 5G)
//! - Throughput: 60 FPS sustained at 1080p
//! - Zero frame drops under normal operation
//! - Memory overhead: <20MB per decoder instance

#![allow(non_snake_case)]
#![warn(missing_docs)]

pub mod codec;
pub mod decoder;
pub mod error;
pub mod ffi;
pub mod metrics;
pub mod llm_jni;

pub use codec::{CodecFormat, MediaFormat};
pub use decoder::{Decoder, DecoderConfig, DecodeResult, FrameBuffer};
pub use error::{Error, Result};
pub use metrics::{DecoderMetrics, FrameMetrics};

use jni::JNIEnv;

/// Global JNI environment handle for thread-safe access
pub struct JniContext {
    vm: Option<std::sync::Arc<jni::JavaVM>>,
}

impl JniContext {
    /// Initialize the JNI context from the current environment
    pub fn init(env: JNIEnv) -> Result<Self> {
        let vm = env.get_java_vm().map_err(|_| Error::JniInitFailed)?;
        Ok(JniContext {
            vm: Some(std::sync::Arc::new(vm)),
        })
    }

    /// Get the Java VM reference
    pub fn java_vm(&self) -> Option<std::sync::Arc<jni::JavaVM>> {
        self.vm.clone()
    }
}

/// FFI entry point: Initialize decoder with MediaCodec
///
/// # Safety
///
/// - `java_env` must be a valid, non-null JNI environment pointer
/// - `mime_type` must be a valid C string (null-terminated)
/// - `width` and `height` must be positive integers
///
/// # Returns
///
/// - Success: Opaque decoder pointer (to be cast to `*mut Decoder` on return)
/// - Failure: Null pointer with error logged
///
/// # Example
///
/// ```c
/// const char* mime = "video/avc";
/// void* decoder = initDecoder(env, mime, 1920, 1080);
/// if (!decoder) {
///     // initialization failed
/// }
/// ```
#[no_mangle]
pub extern "C" fn initDecoder(
    _java_env: *mut jni::sys::JNIEnv,
    mime_type: *const i8,
    width: i32,
    height: i32,
) -> *mut decoder::Decoder {
    if mime_type.is_null() || width <= 0 || height <= 0 {
        log::error!(
            "initDecoder: invalid arguments (mime_type={:?}, {}x{})",
            mime_type.is_null(),
            width,
            height
        );
        return std::ptr::null_mut();
    }

    let mime_str = match unsafe { std::ffi::CStr::from_ptr(mime_type).to_str() } {
        Ok(s) => s,
        Err(_) => {
            log::error!("initDecoder: invalid UTF-8 in mime_type");
            return std::ptr::null_mut();
        }
    };

    let config = match DecoderConfig::new(mime_str, width as u32, height as u32) {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("initDecoder: failed to create config: {}", e);
            return std::ptr::null_mut();
        }
    };

    match Decoder::new(config) {
        Ok(decoder) => {
            let boxed = Box::new(decoder);
            Box::into_raw(boxed)
        }
        Err(e) => {
            log::error!("initDecoder: failed to create decoder: {}", e);
            std::ptr::null_mut()
        }
    }
}

/// FFI entry point: Decode a frame from input buffer
///
/// # Arguments
///
/// - `decoder`: Opaque decoder pointer (cast from `*mut Decoder`)
/// - `input_data`: Input H.264/H.265 NAL unit data
/// - `input_size`: Size of input data in bytes
/// - `timestamp_us`: Presentation timestamp in microseconds
///
/// # Returns
///
/// - 0 on success (frame queued for decoding)
/// - Negative error code on failure
/// - Positive value indicates number of ready output buffers
///
/// # Safety
///
/// - `decoder` must have been created by `initDecoder`
/// - `input_data` must be valid for `input_size` bytes
///
/// # Example
///
/// ```c
/// int result = decodeFrame(decoder, nal_unit_data, 2048, 33333);
/// if (result < 0) {
///     // decoding error
/// } else if (result > 0) {
///     // result frames are ready to dequeue
/// }
/// ```
#[no_mangle]
pub extern "C" fn decodeFrame(
    decoder: *mut decoder::Decoder,
    input_data: *const u8,
    input_size: usize,
    timestamp_us: i64,
) -> i32 {
    if decoder.is_null() || input_data.is_null() || input_size == 0 {
        log::error!(
            "decodeFrame: invalid arguments (decoder={:?}, input={:?}, size={})",
            decoder.is_null(),
            input_data.is_null(),
            input_size
        );
        return -1;
    }

    let decoder = unsafe { &mut *decoder };

    let input_slice = unsafe { std::slice::from_raw_parts(input_data, input_size) };

    match decoder.decode_frame(input_slice, timestamp_us) {
        Ok(count) => count as i32,
        Err(e) => {
            log::error!("decodeFrame: {}", e);
            -1
        }
    }
}

/// FFI entry point: Dequeue decoded output buffer
///
/// # Arguments
///
/// - `decoder`: Opaque decoder pointer
/// - `out_data`: Pointer to store output buffer pointer (YUV420 planar)
/// - `out_size`: Pointer to store output size in bytes
/// - `out_width`: Pointer to store actual frame width
/// - `out_height`: Pointer to store actual frame height
/// - `out_timestamp`: Pointer to store presentation timestamp
///
/// # Returns
///
/// - 0 on success (output buffer filled)
/// - -1 on error
/// - -2 if no output buffer is currently available
///
/// # Safety
///
/// - All output pointers must be valid for writing
/// - Returned buffer pointer is only valid until `releaseBuffer` is called
///
/// # Example
///
/// ```c
/// const uint8_t* frame_data;
/// size_t frame_size;
/// uint32_t width, height;
/// int64_t timestamp;
/// int result = getDecodedFrame(decoder, &frame_data, &frame_size, &width, &height, &timestamp);
/// if (result == 0) {
///     // use frame_data
///     releaseBuffer(decoder, frame_data);
/// }
/// ```
#[no_mangle]
pub extern "C" fn getDecodedFrame(
    decoder: *mut decoder::Decoder,
    out_data: *mut *const u8,
    out_size: *mut usize,
    out_width: *mut u32,
    out_height: *mut u32,
    out_timestamp: *mut i64,
) -> i32 {
    if decoder.is_null()
        || out_data.is_null()
        || out_size.is_null()
        || out_width.is_null()
        || out_height.is_null()
        || out_timestamp.is_null()
    {
        log::error!("getDecodedFrame: null output pointer");
        return -1;
    }

    let decoder = unsafe { &mut *decoder };

    match decoder.get_output_frame() {
        Ok(Some(frame)) => {
            unsafe {
                *out_data = frame.data.as_ptr();
                *out_size = frame.data.len();
                *out_width = frame.width;
                *out_height = frame.height;
                *out_timestamp = frame.timestamp_us;
            }
            0
        }
        Ok(None) => -2,
        Err(e) => {
            log::error!("getDecodedFrame: {}", e);
            -1
        }
    }
}

/// FFI entry point: Release decoded output buffer
///
/// # Arguments
///
/// - `decoder`: Opaque decoder pointer
/// - `buffer_ptr`: Buffer pointer returned from `getDecodedFrame`
///
/// # Returns
///
/// - 0 on success
/// - -1 on error
///
/// # Safety
///
/// - `buffer_ptr` must be a pointer previously returned by `getDecodedFrame`
/// - Must not call twice on the same buffer
///
/// # Example
///
/// ```c
/// releaseBuffer(decoder, frame_data);
/// ```
#[no_mangle]
pub extern "C" fn releaseBuffer(decoder: *mut decoder::Decoder, _buffer_ptr: *const u8) -> i32 {
    if decoder.is_null() {
        log::error!("releaseBuffer: null decoder");
        return -1;
    }

    let decoder = unsafe { &mut *decoder };

    match decoder.release_output_buffer() {
        Ok(_) => 0,
        Err(e) => {
            log::error!("releaseBuffer: {}", e);
            -1
        }
    }
}

/// FFI entry point: Destroy decoder and free resources
///
/// # Arguments
///
/// - `decoder`: Opaque decoder pointer (cast to void*)
///
/// # Safety
///
/// - `decoder` must have been created by `initDecoder`
/// - Must not be called twice on the same pointer
/// - Must not be accessed after calling this function
///
/// # Example
///
/// ```c
/// destroyDecoder((void*)decoder);
/// ```
#[no_mangle]
pub extern "C" fn destroyDecoder(decoder: *mut decoder::Decoder) {
    if decoder.is_null() {
        return;
    }

    log::info!("Destroying decoder");

    let decoder = unsafe { Box::from_raw(decoder) };
    drop(decoder);
}

/// FFI entry point: Get decoder performance metrics
///
/// # Arguments
///
/// - `decoder`: Opaque decoder pointer
/// - `out_metrics`: Pointer to metrics structure (5 x i64)
///   - [0]: avg_decode_latency_us (microseconds)
///   - [1]: max_decode_latency_us (microseconds)
///   - [2]: frames_decoded (total count)
///   - [3]: frames_dropped (total count)
///   - [4]: last_timestamp_us
///
/// # Returns
///
/// - 0 on success
/// - -1 on error
///
/// # Example
///
/// ```c
/// int64_t metrics[5];
/// int result = getMetrics(decoder, metrics);
/// if (result == 0) {
///     printf("Avg latency: %ld us\n", metrics[0]);
///     printf("Frames decoded: %ld\n", metrics[2]);
/// }
/// ```
#[no_mangle]
pub extern "C" fn getMetrics(decoder: *mut decoder::Decoder, out_metrics: *mut i64) -> i32 {
    if decoder.is_null() || out_metrics.is_null() {
        log::error!("getMetrics: null pointer");
        return -1;
    }

    let decoder = unsafe { &*decoder };
    let metrics = decoder.metrics();

    unsafe {
        *out_metrics.offset(0) = metrics.avg_decode_latency_us;
        *out_metrics.offset(1) = metrics.max_decode_latency_us;
        *out_metrics.offset(2) = metrics.frames_decoded as i64;
        *out_metrics.offset(3) = metrics.frames_dropped as i64;
        *out_metrics.offset(4) = metrics.last_timestamp_us.unwrap_or(-1);
    }

    0
}

/// FFI entry point: Get formatted metrics as JSON string
///
/// # Arguments
///
/// - `decoder`: Opaque decoder pointer
/// - `out_json`: Pointer to store JSON string pointer
///
/// # Returns
///
/// - Length of JSON string on success
/// - -1 on error
///
/// # Safety
///
/// - Returned string pointer remains valid until `freeString` is called
/// - String is null-terminated
///
/// # Example
///
/// ```c
/// const char* json_str;
/// int len = getMetricsJson(decoder, &json_str);
/// if (len > 0) {
///     printf("Metrics: %s\n", json_str);
///     freeString(json_str);
/// }
/// ```
#[no_mangle]
pub extern "C" fn getMetricsJson(
    decoder: *mut decoder::Decoder,
    out_json: *mut *const i8,
) -> i32 {
    if decoder.is_null() || out_json.is_null() {
        log::error!("getMetricsJson: null pointer");
        return -1;
    }

    let decoder = unsafe { &*decoder };
    let metrics = decoder.metrics();

    match serde_json::to_string(&metrics) {
        Ok(json) => {
            let c_str = std::ffi::CString::new(json).expect("JSON string contained null byte");
            let ptr = c_str.into_raw();
            unsafe {
                *out_json = ptr as *const i8;
            }
            std::mem::size_of_val(&ptr) as i32
        }
        Err(e) => {
            log::error!("getMetricsJson: serialization error: {}", e);
            -1
        }
    }
}

/// FFI entry point: Free a string allocated by the FFI layer
///
/// # Safety
///
/// - `ptr` must be a valid pointer previously returned by an FFI function
/// - Must not be called twice on the same pointer
///
/// # Example
///
/// ```c
/// freeString(json_str);
/// ```
#[no_mangle]
pub extern "C" fn freeString(ptr: *const i8) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let _ = std::ffi::CString::from_raw(ptr as *mut i8);
    }
}

/// Configure decoder for low-latency operation
///
/// # Arguments
///
/// - `decoder`: Opaque decoder pointer
/// - `enable`: 1 to enable low-latency mode, 0 to disable
///
/// # Returns
///
/// - 0 on success
/// - -1 on error
///
/// # Example
///
/// ```c
/// setLowLatencyMode(decoder, 1);
/// ```
#[no_mangle]
pub extern "C" fn setLowLatencyMode(decoder: *mut decoder::Decoder, enable: i32) -> i32 {
    if decoder.is_null() {
        log::error!("setLowLatencyMode: null decoder");
        return -1;
    }

    let decoder = unsafe { &mut *decoder };
    match decoder.set_low_latency_mode(enable != 0) {
        Ok(_) => 0,
        Err(e) => {
            log::error!("setLowLatencyMode: {}", e);
            -1
        }
    }
}

/// Reset decoder state for seeking or discontinuity
///
/// # Arguments
///
/// - `decoder`: Opaque decoder pointer
///
/// # Returns
///
/// - 0 on success
/// - -1 on error
///
/// # Example
///
/// ```c
/// resetDecoder(decoder);
/// ```
#[no_mangle]
pub extern "C" fn resetDecoder(decoder: *mut decoder::Decoder) -> i32 {
    if decoder.is_null() {
        log::error!("resetDecoder: null decoder");
        return -1;
    }

    let decoder = unsafe { &mut *decoder };
    match decoder.reset() {
        Ok(_) => {
            log::info!("Decoder reset successfully");
            0
        }
        Err(e) => {
            log::error!("resetDecoder: {}", e);
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_with_invalid_args() {
        let result = unsafe { initDecoder(std::ptr::null_mut(), std::ptr::null(), -1, -1) };
        assert!(result.is_null());
    }

    #[test]
    fn test_getmetrics_with_null() {
        let result = unsafe { getMetrics(std::ptr::null_mut(), std::ptr::null_mut()) };
        assert_eq!(result, -1);
    }
}
