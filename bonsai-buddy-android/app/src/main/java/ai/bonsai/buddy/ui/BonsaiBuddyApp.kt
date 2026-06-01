package ai.bonsai.buddy.ui

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.ui.unit.dp

@Composable
fun BonsaiBuddyApp(modifier: Modifier = Modifier) {
    Scaffold(modifier = modifier.fillMaxSize()) { innerPadding ->
        Column(modifier = Modifier.padding(innerPadding)) {
            Text("Bonsai Buddy")
            RemoteDesktopNavigationContent()
        }
    }
}

@Composable
fun RemoteDesktopNavigationContent() {
    Box(modifier = Modifier.fillMaxSize()) {
        Text("Remote Desktop Client")
    }
}
