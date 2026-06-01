package ai.bonsai.buddy.data.remote_desktop

import android.view.Surface
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import java.io.InputStream
import java.io.OutputStream
import java.util.concurrent.atomic.AtomicLong

/**
 * Core BRDF Mobile Client for remote desktop streaming.
 *
 * Manages:
 * - Connection to TransferDaemon streams (video/input)
 * - Hardware-accelerated video decoding (MediaCodec)
 * - Input event transmission
 * - Session statistics and metrics
 *
 * Usage:
 * ```kotlin
 * val client = BrdfMobileClient()
 * client.connect(peerId, token, surface)
 * client.injectTouch(x, y, TouchAction.DOWN)
 * // ... interact with remote desktop
 * client.disconnect()
 * ```
 */
class BrdfMobileClient(
    private val scope: CoroutineScope = CoroutineScope(Dispatchers.Default + Job())
) {
    private val _connectionState = MutableStateFlow(ConnectionState.DISCONNECTED)
    val connectionState: StateFlow<ConnectionState> = _connectionState.asStateFlow()

    private val _sessionStats = MutableStateFlow(SessionStats())
    val sessionStats: StateFlow<SessionStats> = _sessionStats.asStateFlow()

    private val _connectionError = MutableStateFlow<ConnectionError?>(null)
    val connectionError: StateFlow<ConnectionError?> = _connectionError.asStateFlow()

    private var decoder: MediaCodecDecoder? = null
    private var videoStreamJob: Job? = null
    private var statsCollectionJob: Job? = null

    private val videoBytesReceived = AtomicLong(0)
    private val inputEventsSent = AtomicLong(0)
    private val sessionStartTime = AtomicLong(0)

    private var videoInputStream: InputStream? = null
    private var inputOutputStream: OutputStream? = null
    private var lastStatsUpdateTime = 0L
    private var lastDecodedFrames = 0L
    private var lastVideoBytesReceived = 0L

    /**
     * Connect to a remote desktop via TransferDaemon.
     *
     * Opens two streams:
     * 1. Video stream: Receives H.264/H.265 encoded frames
     * 2. Input stream: Sends keyboard/mouse events
     *
     * @param peerId Peer identifier from discovery
     * @param token Capability token for access control
     * @param surface Target Surface for video rendering
     * @throws ConnectionException if connection fails
     */
    suspend fun connect(
        peerId: String,
        token: RemoteDesktopToken,
        surface: Surface
    ) = withContext(Dispatchers.Main) {
        if (_connectionState.value != ConnectionState.DISCONNECTED) {
            throw IllegalStateException("Already connected. Call disconnect() first")
        }

        try {
            _connectionState.value = ConnectionState.CONNECTING

            // Initialize decoder
            val desktopResolution = token.let {
                Resolution(1920, 1080, 60) // Default; will be updated by stream metadata
            }

            decoder = MediaCodecDecoder(surface, "video/avc")
            decoder?.configure(desktopResolution.width, desktopResolution.height)
            decoder?.start()

            // Simulate stream opening (in real impl, would use TransferDaemon)
            // val videoStream = transferDaemonClient.openStream(
            //     peerId = peerId,
            //     streamType = "remote_desktop_video",
            //     token = token.capabilityData
            // )
            // val inputStream = transferDaemonClient.openStream(
            //     peerId = peerId,
            //     streamType = "remote_desktop_input",
            //     token = token.capabilityData
            // )

            sessionStartTime.set(System.currentTimeMillis())
            _connectionState.value = ConnectionState.CONNECTED
            _connectionError.value = null

            // Start background processing jobs
            startVideoStreamProcessing()
            startStatsCollection()
        } catch (e: Exception) {
            _connectionState.value = ConnectionState.ERROR
            _connectionError.value = ConnectionError(
                code = "CONNECTION_FAILED",
                message = e.message ?: "Unknown error",
                cause = e
            )
            cleanup()
            throw ConnectionException("Failed to connect to peer: $peerId", e)
        }
    }

    /**
     * Disconnect from the remote desktop and clean up resources.
     */
    suspend fun disconnect() = withContext(Dispatchers.Main) {
        if (_connectionState.value == ConnectionState.DISCONNECTED) {
            return@withContext
        }

        _connectionState.value = ConnectionState.DISCONNECTED
        cleanup()
    }

    /**
     * Inject a touch event (tap, drag, etc.).
     *
     * @param x X coordinate on desktop (0 to resolution width)
     * @param y Y coordinate on desktop (0 to resolution height)
     * @param action Touch action type
     */
    suspend fun injectTouch(
        x: Float,
        y: Float,
        action: TouchAction
    ) = withContext(Dispatchers.IO) {
        if (_connectionState.value != ConnectionState.CONNECTED) {
            return@withContext
        }

        try {
            val event = InputEvent.TouchEvent(x, y, action)
            sendInputEvent(event)
            inputEventsSent.incrementAndGet()
        } catch (e: Exception) {
            handleInputError(e)
        }
    }

    /**
     * Inject a keyboard event.
     *
     * @param keycode Linux input event code (or Android keycode)
     * @param down True if key pressed, false if released
     * @param modifiers Bitwise OR of modifier flags (Ctrl, Alt, Shift, Meta)
     */
    suspend fun injectKey(
        keycode: Int,
        down: Boolean,
        modifiers: Int = 0
    ) = withContext(Dispatchers.IO) {
        if (_connectionState.value != ConnectionState.CONNECTED) {
            return@withContext
        }

        try {
            val event = InputEvent.KeyEvent(keycode, down, modifiers)
            sendInputEvent(event)
            inputEventsSent.incrementAndGet()
        } catch (e: Exception) {
            handleInputError(e)
        }
    }

    /**
     * Inject text input directly (for on-screen keyboard).
     *
     * @param text Text to send
     */
    suspend fun injectText(text: String) = withContext(Dispatchers.IO) {
        if (_connectionState.value != ConnectionState.CONNECTED) {
            return@withContext
        }

        try {
            val event = InputEvent.TextInputEvent(text)
            sendInputEvent(event)
            inputEventsSent.incrementAndGet()
        } catch (e: Exception) {
            handleInputError(e)
        }
    }

    /**
     * Inject scroll events.
     *
     * @param dx Horizontal scroll (negative = left, positive = right)
     * @param dy Vertical scroll (negative = up, positive = down)
     */
    suspend fun injectScroll(dx: Float, dy: Float) = withContext(Dispatchers.IO) {
        if (_connectionState.value != ConnectionState.CONNECTED) {
            return@withContext
        }

        try {
            val event = InputEvent.ScrollEvent(dx, dy)
            sendInputEvent(event)
            inputEventsSent.incrementAndGet()
        } catch (e: Exception) {
            handleInputError(e)
        }
    }

    private fun startVideoStreamProcessing() {
        videoStreamJob = scope.launch(Dispatchers.IO) {
            try {
                // Simulate reading frames from stream
                // In real implementation:
                // while (isActive && connectionState == CONNECTED) {
                //     val frame = videoStream.readFrame()
                //     decoder?.decodeFrame(frame.data, presentationTimeUs = frame.timestamp)
                //     videoBytesReceived.addAndGet(frame.data.size.toLong())
                // }

                // Temporary: receive from simulated stream
                withContext(Dispatchers.Default) {
                    while (isActive && _connectionState.value == ConnectionState.CONNECTED) {
                        delay(33) // Simulate 30 FPS
                    }
                }
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                _connectionState.value = ConnectionState.ERROR
                _connectionError.value = ConnectionError(
                    code = "VIDEO_STREAM_ERROR",
                    message = e.message ?: "Video stream error",
                    cause = e
                )
            }
        }
    }

    private fun startStatsCollection() {
        statsCollectionJob = scope.launch(Dispatchers.Default) {
            try {
                while (isActive && _connectionState.value == ConnectionState.CONNECTED) {
                    delay(STATS_UPDATE_INTERVAL_MS)
                    updateStats()
                }
            } catch (e: CancellationException) {
                throw e
            }
        }
    }

    private suspend fun updateStats() {
        val currentTime = System.currentTimeMillis()
        val sessionUptime = currentTime - sessionStartTime.get()

        val currentDecodedFrames = decoder?.getDecodedFrameCount() ?: 0L
        val framesDelta = currentDecodedFrames - lastDecodedFrames
        val fps = if (sessionUptime > 0) {
            (framesDelta * 1000f / STATS_UPDATE_INTERVAL_MS)
        } else {
            0f
        }

        val currentBytesReceived = videoBytesReceived.get()
        val bytesDelta = currentBytesReceived - lastVideoBytesReceived
        val bitrate = if (sessionUptime > 0) {
            (bytesDelta * 8L * 1000 / STATS_UPDATE_INTERVAL_MS)
        } else {
            0L
        }

        val decodeLatency = decoder?.getDecodeLatencyMs() ?: 0L
        val droppedFrames = decoder?.getDroppedFrameCount() ?: 0L
        val packetLoss = if (currentDecodedFrames + droppedFrames > 0) {
            (droppedFrames * 100f / (currentDecodedFrames + droppedFrames))
        } else {
            0f
        }

        _sessionStats.value = SessionStats(
            fps = fps,
            bitrate = bitrate,
            latency = decodeLatency,
            packetLoss = packetLoss,
            videoBytesReceived = currentBytesReceived,
            inputEventsSent = inputEventsSent.get(),
            decodedFrames = currentDecodedFrames,
            droppedFrames = droppedFrames,
            uptime = sessionUptime,
            codec = "H.264"
        )

        lastDecodedFrames = currentDecodedFrames
        lastVideoBytesReceived = currentBytesReceived
        lastStatsUpdateTime = currentTime
    }

    private suspend fun sendInputEvent(event: InputEvent) {
        // In real implementation, serialize and send via inputStream
        // outputStream?.write(serializeInputEvent(event))
        // outputStream?.flush()

        // For now, just acknowledge
    }

    private fun handleInputError(e: Exception) {
        if (_connectionState.value == ConnectionState.CONNECTED) {
            _connectionState.value = ConnectionState.ERROR
            _connectionError.value = ConnectionError(
                code = "INPUT_SEND_ERROR",
                message = e.message ?: "Failed to send input",
                cause = e
            )
        }
    }

    private suspend fun cleanup() {
        videoStreamJob?.cancelAndJoin()
        statsCollectionJob?.cancelAndJoin()

        withContext(Dispatchers.Default) {
            try {
                decoder?.stop()
                decoder?.release()
            } catch (e: Exception) {
                // Ignore cleanup errors
            }

            try {
                videoInputStream?.close()
                inputOutputStream?.close()
            } catch (e: Exception) {
                // Ignore close errors
            }
        }

        decoder = null
        videoInputStream = null
        inputOutputStream = null
    }

    /**
     * Release all resources and cancel jobs.
     */
    fun destroy() {
        scope.cancel()
    }

    companion object {
        private const val STATS_UPDATE_INTERVAL_MS = 1000L
    }
}

class ConnectionException(message: String, cause: Throwable? = null) :
    Exception(message, cause)
