package ai.bonsai.buddy.data.remote_desktop

import android.media.MediaCodec
import android.media.MediaFormat
import android.view.Surface
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.nio.ByteBuffer
import java.util.concurrent.atomic.AtomicLong

/**
 * Wrapper around Android's MediaCodec for hardware-accelerated H.264/H.265 video decoding.
 *
 * Handles codec initialization, frame decoding to a Surface, and lifecycle management.
 * Tracks decode latency and frame metrics for performance monitoring.
 */
class MediaCodecDecoder(
    private val surface: Surface,
    private val codec: String = "video/avc" // H.264 by default
) {
    private var mediaCodec: MediaCodec? = null
    private var isConfigured = false
    private var isRunning = false

    private val decodeLatencyMs = AtomicLong(0)
    private val decodedFrameCount = AtomicLong(0)
    private val droppedFrameCount = AtomicLong(0)

    /**
     * Configure the decoder for a specific video format.
     *
     * @param width Frame width in pixels
     * @param height Frame height in pixels
     * @param colorFormat Android color format (optional)
     * @throws Exception if codec configuration fails
     */
    suspend fun configure(
        width: Int,
        height: Int,
        colorFormat: Int = MediaFormat.COLOR_FormatSurface
    ) = withContext(Dispatchers.Default) {
        if (isConfigured) {
            throw IllegalStateException("Decoder already configured")
        }

        try {
            mediaCodec = MediaCodec.createDecoderByType(codec)
            val mediaCodec = mediaCodec ?: throw IllegalStateException("Failed to create MediaCodec")

            val format = MediaFormat.createVideoFormat(codec, width, height).apply {
                setInteger(MediaFormat.KEY_COLOR_FORMAT, colorFormat)
            }

            mediaCodec.configure(format, surface, null, 0)
            isConfigured = true
        } catch (e: Exception) {
            throw DecoderException("Failed to configure decoder", e)
        }
    }

    /**
     * Start the decoder.
     *
     * Must be called after configure().
     */
    suspend fun start() = withContext(Dispatchers.Default) {
        if (!isConfigured) {
            throw IllegalStateException("Decoder not configured. Call configure() first")
        }
        if (isRunning) {
            return@withContext
        }

        try {
            mediaCodec?.start()
            isRunning = true
        } catch (e: Exception) {
            throw DecoderException("Failed to start decoder", e)
        }
    }

    /**
     * Decode a video frame from raw H.264/H.265 data.
     *
     * @param data Encoded frame data
     * @param offset Offset in data buffer
     * @param size Number of bytes to read
     * @param presentationTimeUs Presentation timestamp (microseconds)
     * @param isKeyFrame Whether this is a keyframe (I-frame)
     * @throws DecoderException if decode fails
     */
    suspend fun decodeFrame(
        data: ByteArray,
        offset: Int = 0,
        size: Int = data.size,
        presentationTimeUs: Long = System.currentTimeMillis() * 1000,
        isKeyFrame: Boolean = false
    ) = withContext(Dispatchers.Default) {
        if (!isRunning) {
            throw IllegalStateException("Decoder not running. Call start() first")
        }

        val codec = mediaCodec ?: throw IllegalStateException("MediaCodec released")
        val startTime = System.nanoTime()

        try {
            // Queue input
            val inputBufferIndex = codec.dequeueInputBuffer(TIMEOUT_US)
            if (inputBufferIndex < 0) {
                droppedFrameCount.incrementAndGet()
                return@withContext
            }

            val inputBuffer = codec.getInputBuffer(inputBufferIndex)
                ?: throw DecoderException("Failed to get input buffer")

            inputBuffer.put(data, offset, size)

            val flags = if (isKeyFrame) MediaCodec.BUFFER_FLAG_KEY_FRAME else 0
            codec.queueInputBuffer(inputBufferIndex, 0, size, presentationTimeUs, flags)

            // Process output
            val bufferInfo = MediaCodec.BufferInfo()
            val outputBufferIndex = codec.dequeueOutputBuffer(bufferInfo, TIMEOUT_US)

            when {
                outputBufferIndex >= 0 -> {
                    decodedFrameCount.incrementAndGet()
                    codec.releaseOutputBuffer(outputBufferIndex, true) // render to surface
                    updateDecodeLatency(startTime)
                }
                outputBufferIndex == MediaCodec.INFO_OUTPUT_FORMAT_CHANGED -> {
                    // Output format changed (resolution, color space, etc.)
                    val newFormat = codec.outputFormat
                    // Could be used for adaptive playback
                }
                outputBufferIndex == MediaCodec.INFO_TRY_AGAIN_LATER -> {
                    droppedFrameCount.incrementAndGet()
                }
            }
        } catch (e: Exception) {
            throw DecoderException("Frame decode error", e)
        }
    }

    /**
     * Stop the decoder and release resources.
     *
     * After calling this, the decoder cannot be reused.
     */
    suspend fun stop() = withContext(Dispatchers.Default) {
        if (!isRunning) {
            return@withContext
        }

        try {
            mediaCodec?.stop()
            isRunning = false
        } catch (e: Exception) {
            // Log but don't throw; we want to ensure release() is called
        }
    }

    /**
     * Release all resources.
     *
     * Must be called when done with the decoder.
     */
    suspend fun release() = withContext(Dispatchers.Default) {
        try {
            mediaCodec?.release()
        } catch (e: Exception) {
            // Ignore release errors
        } finally {
            mediaCodec = null
            isConfigured = false
            isRunning = false
        }
    }

    /**
     * Get the average decode latency in milliseconds.
     */
    fun getDecodeLatencyMs(): Long = decodeLatencyMs.get()

    /**
     * Get the total number of decoded frames.
     */
    fun getDecodedFrameCount(): Long = decodedFrameCount.get()

    /**
     * Get the total number of dropped frames.
     */
    fun getDroppedFrameCount(): Long = droppedFrameCount.get()

    /**
     * Check if decoder is currently running.
     */
    fun isDecoderRunning(): Boolean = isRunning

    private fun updateDecodeLatency(startNanoTime: Long) {
        val latencyNano = System.nanoTime() - startNanoTime
        val latencyMs = latencyNano / 1_000_000
        // Update with exponential moving average
        val currentLatency = decodeLatencyMs.get()
        val newLatency = if (currentLatency == 0L) {
            latencyMs
        } else {
            (currentLatency * 7 + latencyMs) / 8 // 87.5% old, 12.5% new
        }
        decodeLatencyMs.set(newLatency)
    }

    companion object {
        private const val TIMEOUT_US = 10_000L // 10ms timeout
    }
}

class DecoderException(message: String, cause: Throwable? = null) :
    Exception(message, cause)
