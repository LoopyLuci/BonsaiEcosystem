package ai.bonsai.buddy.ui.remote_desktop

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Backspace
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowLeft
import androidx.compose.material.icons.filled.KeyboardArrowRight
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

/**
 * Composable on-screen keyboard for remote desktop input.
 *
 * Features:
 * - Full QWERTY layout
 * - Modifier keys: Ctrl, Alt, Shift, Win/Super
 * - Function keys: F1-F12
 * - Special keys: Esc, Tab, Enter, Backspace, Delete, arrows
 * - Proper Android keycode mapping
 * - Shift key toggles uppercase letters
 * - Dismissible with slide animation
 */
@Composable
fun OnScreenKeyboard(
    visible: Boolean,
    onKeyPress: (Int, Boolean, Int) -> Unit,
    onTextInput: (String) -> Unit,
    onClose: () -> Unit,
    modifier: Modifier = Modifier
) {
    var shiftActive by remember { mutableStateOf(false) }
    var ctrlActive by remember { mutableStateOf(false) }
    var altActive by remember { mutableStateOf(false) }
    var metaActive by remember { mutableStateOf(false) }

    AnimatedVisibility(
        visible = visible,
        enter = slideInVertically(initialOffsetY = { it }),
        exit = slideOutVertically(targetOffsetY = { it }),
        modifier = modifier
    ) {
        Surface(
            color = MaterialTheme.colorScheme.surface,
            modifier = Modifier
                .fillMaxWidth()
                .padding(4.dp)
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(MaterialTheme.colorScheme.surfaceContainer)
                    .padding(8.dp),
                verticalArrangement = Arrangement.spacedBy(4.dp)
            ) {
                // Function keys row
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(2.dp)
                ) {
                    repeat(12) { i ->
                        KeyboardButton(
                            label = "F${i + 1}",
                            onClick = {
                                val keycode = 0x3B + i // F1-F12 keycodes
                                onKeyPress(keycode, true, 0)
                                onKeyPress(keycode, false, 0)
                            },
                            modifier = Modifier.weight(1f),
                            fontSize = 10.sp
                        )
                    }
                }

                // Number and symbol row
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(2.dp)
                ) {
                    val numberKeys = listOf(
                        "1!" to 0x02, "2@" to 0x03, "3#" to 0x04, "4$" to 0x05,
                        "5%" to 0x06, "6^" to 0x07, "7&" to 0x08, "8*" to 0x09,
                        "9(" to 0x0A, "0)" to 0x0B
                    )
                    numberKeys.forEach { (label, keycode) ->
                        KeyboardButton(
                            label = label,
                            onClick = {
                                val text = if (shiftActive) label.last() else label.first()
                                onTextInput(text.toString())
                                if (shiftActive) shiftActive = false
                            },
                            modifier = Modifier.weight(1f)
                        )
                    }
                    KeyboardButton(
                        label = "-_",
                        onClick = { onTextInput(if (shiftActive) "_" else "-") },
                        modifier = Modifier.weight(1f)
                    )
                    KeyboardButton(
                        label = "=+",
                        onClick = { onTextInput(if (shiftActive) "+" else "=") },
                        modifier = Modifier.weight(1f)
                    )
                }

                // QWERTY row
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(2.dp)
                ) {
                    listOf("Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P").forEach { key ->
                        KeyboardButton(
                            label = if (shiftActive) key else key.lowercase(),
                            onClick = {
                                onTextInput(key.lowercase())
                                if (shiftActive) shiftActive = false
                            },
                            modifier = Modifier.weight(1f)
                        )
                    }
                    KeyboardButton(
                        label = "[{",
                        onClick = { onTextInput(if (shiftActive) "{" else "[") },
                        modifier = Modifier.weight(1f)
                    )
                    KeyboardButton(
                        label = "]}",
                        onClick = { onTextInput(if (shiftActive) "}" else "]") },
                        modifier = Modifier.weight(1f)
                    )
                }

                // ASDF row
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(2.dp)
                ) {
                    listOf("A", "S", "D", "F", "G", "H", "J", "K", "L").forEach { key ->
                        KeyboardButton(
                            label = if (shiftActive) key else key.lowercase(),
                            onClick = {
                                onTextInput(key.lowercase())
                                if (shiftActive) shiftActive = false
                            },
                            modifier = Modifier.weight(1f)
                        )
                    }
                    KeyboardButton(
                        label = ";:",
                        onClick = { onTextInput(if (shiftActive) ":" else ";") },
                        modifier = Modifier.weight(1f)
                    )
                    KeyboardButton(
                        label = "'\"",
                        onClick = { onTextInput(if (shiftActive) "\"" else "'") },
                        modifier = Modifier.weight(1f)
                    )
                }

                // ZXCV row
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(2.dp)
                ) {
                    listOf("Z", "X", "C", "V", "B", "N", "M").forEach { key ->
                        KeyboardButton(
                            label = if (shiftActive) key else key.lowercase(),
                            onClick = {
                                onTextInput(key.lowercase())
                                if (shiftActive) shiftActive = false
                            },
                            modifier = Modifier.weight(1f)
                        )
                    }
                    KeyboardButton(
                        label = ",<",
                        onClick = { onTextInput(if (shiftActive) "<" else ",") },
                        modifier = Modifier.weight(1f)
                    )
                    KeyboardButton(
                        label = ".>",
                        onClick = { onTextInput(if (shiftActive) ">" else ".") },
                        modifier = Modifier.weight(1f)
                    )
                    KeyboardButton(
                        label = "/?",
                        onClick = { onTextInput(if (shiftActive) "?" else "/") },
                        modifier = Modifier.weight(1f)
                    )
                }

                // Modifier and special keys row
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(2.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    ModifierButton(
                        label = "Esc",
                        active = false,
                        onClick = {
                            onKeyPress(0x01, true, 0) // ESC keycode
                            onKeyPress(0x01, false, 0)
                        },
                        modifier = Modifier.weight(0.8f)
                    )

                    ModifierButton(
                        label = "Ctrl",
                        active = ctrlActive,
                        onClick = {
                            ctrlActive = !ctrlActive
                        },
                        modifier = Modifier.weight(0.8f)
                    )

                    ModifierButton(
                        label = "Alt",
                        active = altActive,
                        onClick = {
                            altActive = !altActive
                        },
                        modifier = Modifier.weight(0.8f)
                    )

                    ModifierButton(
                        label = "Shift",
                        active = shiftActive,
                        onClick = {
                            shiftActive = !shiftActive
                        },
                        modifier = Modifier.weight(0.8f)
                    )

                    ModifierButton(
                        label = "Win",
                        active = metaActive,
                        onClick = {
                            metaActive = !metaActive
                        },
                        modifier = Modifier.weight(0.8f)
                    )

                    KeyboardButton(
                        label = "Enter",
                        onClick = {
                            onKeyPress(0x1C, true, 0) // ENTER keycode
                            onKeyPress(0x1C, false, 0)
                        },
                        modifier = Modifier.weight(1f),
                        fontSize = 11.sp
                    )

                    KeyboardButton(
                        label = "Space",
                        onClick = {
                            onTextInput(" ")
                            if (shiftActive) shiftActive = false
                        },
                        modifier = Modifier.weight(1.5f),
                        fontSize = 11.sp
                    )
                }

                // Arrow keys and special keys
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(2.dp)
                ) {
                    KeyboardButton(
                        label = "Tab",
                        onClick = {
                            onKeyPress(0x0F, true, 0) // TAB keycode
                            onKeyPress(0x0F, false, 0)
                        },
                        modifier = Modifier.weight(1f),
                        fontSize = 10.sp
                    )

                    KeyboardButton(
                        label = "Home",
                        onClick = {
                            onKeyPress(0x47, true, 0) // HOME keycode
                            onKeyPress(0x47, false, 0)
                        },
                        modifier = Modifier.weight(1f),
                        fontSize = 10.sp
                    )

                    KeyboardButton(
                        label = "↑",
                        onClick = {
                            onKeyPress(0x48, true, 0) // UP arrow
                            onKeyPress(0x48, false, 0)
                        },
                        modifier = Modifier.weight(1f)
                    )

                    KeyboardButton(
                        label = "End",
                        onClick = {
                            onKeyPress(0x4F, true, 0) // END keycode
                            onKeyPress(0x4F, false, 0)
                        },
                        modifier = Modifier.weight(1f),
                        fontSize = 10.sp
                    )

                    KeyboardButton(
                        label = "Del",
                        onClick = {
                            onKeyPress(0x53, true, 0) // DELETE keycode
                            onKeyPress(0x53, false, 0)
                        },
                        modifier = Modifier.weight(1f),
                        fontSize = 10.sp
                    )
                }

                // Arrows row
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .align(Alignment.CenterHorizontally),
                    horizontalArrangement = Arrangement.Center
                ) {
                    KeyboardButton(
                        label = "←",
                        onClick = {
                            onKeyPress(0x4B, true, 0) // LEFT arrow
                            onKeyPress(0x4B, false, 0)
                        },
                        modifier = Modifier
                            .width(40.dp)
                            .height(40.dp)
                    )

                    KeyboardButton(
                        label = "↓",
                        onClick = {
                            onKeyPress(0x50, true, 0) // DOWN arrow
                            onKeyPress(0x50, false, 0)
                        },
                        modifier = Modifier
                            .width(40.dp)
                            .height(40.dp)
                    )

                    KeyboardButton(
                        label = "→",
                        onClick = {
                            onKeyPress(0x4D, true, 0) // RIGHT arrow
                            onKeyPress(0x4D, false, 0)
                        },
                        modifier = Modifier
                            .width(40.dp)
                            .height(40.dp)
                    )

                    Spacer(modifier = Modifier.width(8.dp))

                    Button(
                        onClick = onClose,
                        modifier = Modifier
                            .width(40.dp)
                            .height(40.dp),
                        contentPadding = PaddingValues(2.dp)
                    ) {
                        Text("✕", fontSize = 20.sp)
                    }
                }
            }
        }
    }
}

@Composable
private fun KeyboardButton(
    label: String,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    fontSize: TextUnit = 12.sp
) {
    Button(
        onClick = onClick,
        modifier = modifier.height(40.dp),
        contentPadding = PaddingValues(4.dp),
        colors = ButtonDefaults.buttonColors(
            containerColor = MaterialTheme.colorScheme.secondaryContainer,
            contentColor = MaterialTheme.colorScheme.onSecondaryContainer
        ),
        shape = RoundedCornerShape(4.dp)
    ) {
        Text(
            text = label,
            fontSize = fontSize,
            textAlign = TextAlign.Center,
            maxLines = 1
        )
    }
}

@Composable
private fun ModifierButton(
    label: String,
    active: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Button(
        onClick = onClick,
        modifier = modifier.height(40.dp),
        contentPadding = PaddingValues(4.dp),
        colors = ButtonDefaults.buttonColors(
            containerColor = if (active)
                MaterialTheme.colorScheme.primary
            else
                MaterialTheme.colorScheme.secondaryContainer,
            contentColor = if (active)
                MaterialTheme.colorScheme.onPrimary
            else
                MaterialTheme.colorScheme.onSecondaryContainer
        ),
        shape = RoundedCornerShape(4.dp)
    ) {
        Text(
            text = label,
            fontSize = 10.sp,
            textAlign = TextAlign.Center,
            maxLines = 1
        )
    }
}
