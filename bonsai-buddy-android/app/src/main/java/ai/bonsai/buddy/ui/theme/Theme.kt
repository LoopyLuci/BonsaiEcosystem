package ai.bonsai.buddy.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

// BonsAI brand colors
private val PrimaryColor = Color(0xFF7AA2F7)      // Blue
private val SecondaryColor = Color(0xFF9ECE6A)    // Green
private val TertiaryColor = Color(0xFFFF9500)     // Orange
private val BackgroundColor = Color(0xFF0F1419)   // Dark
private val SurfaceColor = Color(0xFF1A1B26)      // Darker

private val LightColorScheme = lightColorScheme(
    primary = PrimaryColor,
    secondary = SecondaryColor,
    tertiary = TertiaryColor,
    background = Color.White,
    surface = Color(0xFFF5F5F5),
    onBackground = Color.Black,
    onSurface = Color.Black
)

private val DarkColorScheme = darkColorScheme(
    primary = PrimaryColor,
    secondary = SecondaryColor,
    tertiary = TertiaryColor,
    background = BackgroundColor,
    surface = SurfaceColor,
    onBackground = Color.White,
    onSurface = Color.White
)

@Composable
fun BonsaiBuddyTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit
) {
    val colorScheme = if (darkTheme) DarkColorScheme else LightColorScheme
    
    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography,
        content = content
    )
}
