package ai.bonsai.buddy.ui.remote_desktop

import android.view.Surface
import android.view.SurfaceHolder
import android.view.SurfaceView
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Keyboard
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.hilt.navigation.compose.hiltViewModel
import ai.bonsai.buddy.data.remote_desktop.*

/**
 * Main screen for remote desktop streaming.
 *
 * Layout:
 * - Video view (SurfaceView for hardware decoding)
 * - Connection bar (top) with stats
 * - Gesture overlay layer (for touch input)
 * - Floating toolbar (bottom) with controls
 * - On-screen keyboard (when toggled)
 */
@Composable
fun RemoteDesktopScreen(
    peerId: String,
    tokenBase64: String,
    onNavigateBack: () -> Unit,
    modifier: Modifier = Modifier,
    viewModel: RemoteDesktopViewModel = hiltViewModel()
) {
    val context = LocalContext.current
    val connectionState by viewModel.connectionState.collectAsState()
    val sessionStats by viewModel.sessionStats.collectAsState()
    val showKeyboard by viewModel.showKeyboard.collectAsState()
    val mouseMode by viewModel.mouseMode.collectAsState()
    val error by viewModel.error.collectAsState()

    var surfaceView by remember { mutableStateOf<SurfaceView?>(null) }
    var snackbarVisible by remember { mutableStateOf(false) }
    var snackbarMessage by remember { mutableStateOf("") }

    // Initialize connection on first launch
    LaunchedEffect(peerId, tokenBase64) {
        try {
            val token = RemoteDesktopToken(
                peerId = peerId,
                capabilityData = tokenBase64
            )

            surfaceView?.holder?.addCallback(object : SurfaceHolder.Callback {
                override fun surfaceCreated(holder: SurfaceHolder) {
                    holder.surface?.let { surface ->
                        viewModel.connectToDesktop(peerId, token, surface)
                    }
                }

                override fun surfaceChanged(
                    holder: SurfaceHolder,
                    format: Int,
                    width: Int,
                    height: Int
                ) {
                    // Surface dimensions changed
                }

                override fun surfaceDestroyed(holder: SurfaceHolder) {
                    viewModel.disconnect()
                }
            })
        } catch (e: Exception) {
            snackbarMessage = "Failed to initialize: ${e.message}"
            snackbarVisible = true
        }
    }

    // Show errors as snackbars
    LaunchedEffect(error) {
        if (error != null) {
            snackbarMessage = error!!
            snackbarVisible = true
        }
    }

    Box(
        modifier = modifier
            .fillMaxSize()
            .background(Color.Black)
    ) {
        // Video view
        AndroidView(
            factory = { context ->
                SurfaceView(context).apply {
                    surfaceView = this
                    setBackgroundColor(android.graphics.Color.BLACK)
                }
            },
            modifier = Modifier.fillMaxSize()
        )

        // Connection status bar (top)
        ConnectionBar(
            peerName = peerId,
            connectionState = connectionState,
            stats = sessionStats,
            modifier = Modifier
                .align(Alignment.TopCenter)
                .fillMaxWidth()
        )

        // Gesture input overlay
        GestureInputOverlay(
            modifier = Modifier
                .fillMaxSize()
                .align(Alignment.Center),
            onTouchEvent = { x, y, action ->
                viewModel.sendTouchEvent(x, y, action)
            },
            onKeyEvent = { keycode, down, modifiers ->
                viewModel.sendKeyEvent(keycode, down, modifiers)
            }
        )

        // Floating toolbar (bottom)
        FloatingToolbar(
            mouseMode = mouseMode,
            onKeyboardClick = { viewModel.toggleKeyboard() },
            onMouseModeClick = { viewModel.toggleMouseMode() },
            onDisconnect = {
                viewModel.disconnect()
                onNavigateBack()
            },
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .fillMaxWidth()
                .padding(16.dp)
        )

        // On-screen keyboard
        OnScreenKeyboard(
            visible = showKeyboard,
            onKeyPress = { keycode, down, modifiers ->
                viewModel.sendKeyEvent(keycode, down, modifiers)
            },
            onTextInput = { text ->
                viewModel.sendText(text)
            },
            onClose = { viewModel.hideKeyboard() },
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .fillMaxWidth()
        )

        // Error snackbar
        if (snackbarVisible) {
            Snackbar(
                modifier = Modifier
                    .align(Alignment.BottomCenter)
                    .padding(16.dp),
                action = {
                    TextButton(onClick = { snackbarVisible = false }) {
                        Text("Dismiss")
                    }
                }
            ) {
                Text(snackbarMessage)
            }
        }
    }
}

/**
 * Top connection bar showing peer name, status, and statistics.
 */
@Composable
private fun ConnectionBar(
    peerName: String,
    connectionState: ConnectionState,
    stats: SessionStats,
    modifier: Modifier = Modifier
) {
    Surface(
        color = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.9f),
        modifier = modifier.height(56.dp)
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 12.dp, vertical = 8.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text(
                    text = peerName,
                    style = MaterialTheme.typography.labelLarge,
                    maxLines = 1
                )
                Text(
                    text = when (connectionState) {
                        ConnectionState.CONNECTED -> "Connected"
                        ConnectionState.CONNECTING -> "Connecting..."
                        ConnectionState.RECONNECTING -> "Reconnecting..."
                        ConnectionState.DISCONNECTED -> "Disconnected"
                        ConnectionState.ERROR -> "Error"
                        ConnectionState.SUSPENDED -> "Suspended"
                    },
                    style = MaterialTheme.typography.labelSmall,
                    color = when (connectionState) {
                        ConnectionState.CONNECTED -> Color.Green
                        ConnectionState.ERROR -> Color.Red
                        else -> MaterialTheme.colorScheme.onSurfaceVariant
                    },
                    maxLines = 1
                )
            }

            if (connectionState == ConnectionState.CONNECTED) {
                Row(
                    horizontalArrangement = Arrangement.spacedBy(12.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.height(48.dp)
                ) {
                    StatBadge("FPS", String.format("%.0f", stats.fps))
                    StatBadge("Kbps", String.format("%.0f", stats.bitrate / 1000))
                    StatBadge("Ms", "${stats.latency}")
                    StatBadge("Loss", String.format("%.1f%%", stats.packetLoss))
                }
            }
        }
    }
}

@Composable
private fun StatBadge(label: String, value: String) {
    Surface(
        color = MaterialTheme.colorScheme.tertiary.copy(alpha = 0.8f),
        shape = MaterialTheme.shapes.extraSmall,
        modifier = Modifier.padding(horizontal = 4.dp)
    ) {
        Row(
            modifier = Modifier.padding(horizontal = 6.dp, vertical = 2.dp),
            horizontalArrangement = Arrangement.spacedBy(2.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(label, fontSize = 8.sp, color = MaterialTheme.colorScheme.onTertiary)
            Text(value, fontSize = 9.sp, color = MaterialTheme.colorScheme.onTertiary)
        }
    }
}

/**
 * Gesture input overlay that detects touch events and converts them to input.
 */
@Composable
private fun GestureInputOverlay(
    modifier: Modifier = Modifier,
    onTouchEvent: (Float, Float, TouchAction) -> Unit,
    onKeyEvent: (Int, Boolean, Int) -> Unit
) {
    Box(
        modifier = modifier
            .pointerInput(Unit) {
                detectDragGestures(
                    onDragStart = { offset ->
                        onTouchEvent(offset.x, offset.y, TouchAction.DOWN)
                    },
                    onDrag = { change, _ ->
                        onTouchEvent(change.position.x, change.position.y, TouchAction.MOVE)
                    },
                    onDragEnd = {
                        // Get the last known position from the drag (ideally we'd pass it)
                        // For now, we'll handle this in a real implementation
                        onKeyEvent(0, false, 0) // Placeholder
                    }
                )
            }
            .clickable(enabled = false) {}
    )
}

/**
 * Floating toolbar with keyboard, mouse mode, and disconnect buttons.
 */
@Composable
private fun FloatingToolbar(
    mouseMode: InputMapper.MouseMode,
    onKeyboardClick: () -> Unit,
    onMouseModeClick: () -> Unit,
    onDisconnect: () -> Unit,
    modifier: Modifier = Modifier
) {
    Surface(
        color = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.95f),
        shape = MaterialTheme.shapes.large,
        modifier = modifier
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(8.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            IconButton(
                onClick = onKeyboardClick,
                modifier = Modifier.size(40.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.Keyboard,
                    contentDescription = "Toggle Keyboard",
                    tint = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }

            TextButton(
                onClick = onMouseModeClick,
                modifier = Modifier
                    .weight(1f)
                    .height(40.dp),
                colors = ButtonDefaults.textButtonColors(
                    contentColor = MaterialTheme.colorScheme.onSurfaceVariant
                )
            ) {
                Text(
                    text = when (mouseMode) {
                        InputMapper.MouseMode.ABSOLUTE -> "Absolute"
                        InputMapper.MouseMode.RELATIVE -> "Relative"
                    },
                    fontSize = 12.sp
                )
            }

            IconButton(
                onClick = onDisconnect,
                modifier = Modifier.size(40.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.Close,
                    contentDescription = "Disconnect",
                    tint = MaterialTheme.colorScheme.error
                )
            }
        }
    }
}
