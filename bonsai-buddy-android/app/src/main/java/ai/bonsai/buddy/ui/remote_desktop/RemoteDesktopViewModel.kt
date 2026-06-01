package ai.bonsai.buddy.ui.remote_desktop

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import ai.bonsai.buddy.data.remote_desktop.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

/**
 * ViewModel for remote desktop screen.
 *
 * Manages:
 * - BrdfMobileClient lifecycle
 * - UI state (connection, stats, keyboard visibility)
 * - Input event routing
 * - Proper coroutine scope management
 */
class RemoteDesktopViewModel : ViewModel() {
    private var client: BrdfMobileClient? = null

    private val _connectionState = MutableStateFlow(ConnectionState.DISCONNECTED)
    val connectionState: StateFlow<ConnectionState> = _connectionState.asStateFlow()

    private val _sessionStats = MutableStateFlow(SessionStats())
    val sessionStats: StateFlow<SessionStats> = _sessionStats.asStateFlow()

    private val _showKeyboard = MutableStateFlow(false)
    val showKeyboard: StateFlow<Boolean> = _showKeyboard.asStateFlow()

    private val _mouseMode = MutableStateFlow(InputMapper.MouseMode.ABSOLUTE)
    val mouseMode: StateFlow<InputMapper.MouseMode> = _mouseMode.asStateFlow()

    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error.asStateFlow()

    /**
     * Initialize connection to a remote desktop.
     *
     * @param peerId Peer identifier
     * @param token Capability token
     * @param surface Target Surface for video
     */
    fun connectToDesktop(
        peerId: String,
        token: RemoteDesktopToken,
        surface: android.view.Surface
    ) {
        viewModelScope.launch {
            try {
                client = BrdfMobileClient(viewModelScope)

                // Observe client state
                client?.connectionState?.let { state ->
                    launch { state.collect { _connectionState.value = it } }
                }

                client?.sessionStats?.let { stats ->
                    launch { stats.collect { _sessionStats.value = it } }
                }

                client?.connect(peerId, token, surface)
            } catch (e: Exception) {
                _error.value = "Connection failed: ${e.message}"
                _connectionState.value = ConnectionState.ERROR
            }
        }
    }

    /**
     * Disconnect from the remote desktop.
     */
    fun disconnect() {
        viewModelScope.launch {
            try {
                client?.disconnect()
            } finally {
                client?.destroy()
                client = null
                _connectionState.value = ConnectionState.DISCONNECTED
            }
        }
    }

    /**
     * Send a touch event to the remote desktop.
     */
    fun sendTouchEvent(x: Float, y: Float, action: TouchAction) {
        viewModelScope.launch {
            try {
                client?.injectTouch(x, y, action)
            } catch (e: Exception) {
                _error.value = "Failed to send touch: ${e.message}"
            }
        }
    }

    /**
     * Send a keyboard event.
     */
    fun sendKeyEvent(keycode: Int, down: Boolean, modifiers: Int = 0) {
        viewModelScope.launch {
            try {
                client?.injectKey(keycode, down, modifiers)
            } catch (e: Exception) {
                _error.value = "Failed to send key: ${e.message}"
            }
        }
    }

    /**
     * Send text input.
     */
    fun sendText(text: String) {
        viewModelScope.launch {
            try {
                client?.injectText(text)
            } catch (e: Exception) {
                _error.value = "Failed to send text: ${e.message}"
            }
        }
    }

    /**
     * Send scroll event.
     */
    fun sendScroll(dx: Float, dy: Float) {
        viewModelScope.launch {
            try {
                client?.injectScroll(dx, dy)
            } catch (e: Exception) {
                _error.value = "Failed to send scroll: ${e.message}"
            }
        }
    }

    /**
     * Toggle on-screen keyboard visibility.
     */
    fun toggleKeyboard() {
        _showKeyboard.value = !_showKeyboard.value
    }

    /**
     * Hide the on-screen keyboard.
     */
    fun hideKeyboard() {
        _showKeyboard.value = false
    }

    /**
     * Toggle mouse mode between absolute and relative.
     */
    fun toggleMouseMode() {
        _mouseMode.value = when (_mouseMode.value) {
            InputMapper.MouseMode.ABSOLUTE -> InputMapper.MouseMode.RELATIVE
            InputMapper.MouseMode.RELATIVE -> InputMapper.MouseMode.ABSOLUTE
        }
    }

    /**
     * Clear error message.
     */
    fun clearError() {
        _error.value = null
    }

    override fun onCleared() {
        super.onCleared()
        client?.destroy()
        client = null
    }
}
