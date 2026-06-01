package ai.bonsai.buddy

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Surface
import androidx.compose.ui.Modifier
import ai.bonsai.buddy.ui.BonsaiBuddyApp
import ai.bonsai.buddy.ui.theme.BonsaiBuddyTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            BonsaiBuddyTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    BonsaiBuddyApp()
                }
            }
        }
    }
}
