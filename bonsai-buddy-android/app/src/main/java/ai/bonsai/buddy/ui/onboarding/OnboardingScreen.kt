package ai.bonsai.buddy.ui.onboarding

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.QrCodeScanner
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import ai.bonsai.buddy.data.discovery.DiscoveredWorkspace

@Composable
fun OnboardingRoute(
    onOnboardingComplete: () -> Unit,
    modifier: Modifier = Modifier,
    viewModel: OnboardingViewModel = hiltViewModel()
) {
    val state = viewModel.uiState.value
    OnboardingScreen(
        state = state,
        onSelectWorkspace = viewModel::selectWorkspace,
        onManualEndpoint = viewModel::setManualEndpoint,
        onTokenChange = viewModel::setToken,
        onQrToken = viewModel::setTokenFromQr,
        onNext = viewModel::nextStep,
        onBack = viewModel::prevStep,
        onVerify = { viewModel.verifyAndPersist(onOnboardingComplete) },
        modifier = modifier
    )
}

@Composable
fun OnboardingScreen(
    state: OnboardingUiState,
    onSelectWorkspace: (DiscoveredWorkspace) -> Unit,
    onManualEndpoint: (String) -> Unit,
    onTokenChange: (String) -> Unit,
    onQrToken: (String) -> Unit,
    onNext: () -> Unit,
    onBack: () -> Unit,
    onVerify: () -> Unit,
    modifier: Modifier = Modifier
) {
    var manualEndpoint by remember { mutableStateOf("") }
    var mockQrToken by remember { mutableStateOf("") }

    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Text("Bonsai Buddy Setup", style = MaterialTheme.typography.headlineSmall)
        Text(
            "Step ${state.step.ordinal + 1} of 3",
            style = MaterialTheme.typography.labelLarge,
            color = MaterialTheme.colorScheme.primary
        )

        when (state.step) {
            OnboardingStep.Discover -> {
                Text("Discover Bonsai Workspace instances on your LAN.")
                LazyColumn(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(220.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    items(state.discovered, key = { "${it.host}:${it.port}" }) { item ->
                        WorkspaceCard(item = item, onClick = { onSelectWorkspace(item) })
                    }
                }
                OutlinedTextField(
                    value = manualEndpoint,
                    onValueChange = { manualEndpoint = it },
                    label = { Text("Manual host:port") },
                    placeholder = { Text("192.168.1.20:11420") },
                    modifier = Modifier.fillMaxWidth()
                )
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    TextButton(onClick = { onManualEndpoint(manualEndpoint) }) {
                        Text("Use Manual Endpoint")
                    }
                    Button(onClick = onNext, enabled = state.selectedHost.isNotBlank()) {
                        Text("Continue")
                    }
                }
            }

            OnboardingStep.Authenticate -> {
                Text("Enter desktop connection token.")
                OutlinedTextField(
                    value = state.token,
                    onValueChange = onTokenChange,
                    label = { Text("Desktop token") },
                    modifier = Modifier.fillMaxWidth()
                )
                OutlinedTextField(
                    value = mockQrToken,
                    onValueChange = { mockQrToken = it },
                    label = { Text("QR payload (temporary until CameraX setup)") },
                    modifier = Modifier.fillMaxWidth()
                )
                Button(onClick = { onQrToken(mockQrToken) }) {
                    Icon(Icons.Default.QrCodeScanner, contentDescription = null)
                    Text("  Scan QR Code")
                }
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    TextButton(onClick = onBack) { Text("Back") }
                    Button(onClick = onNext, enabled = state.token.isNotBlank()) {
                        Text("Continue")
                    }
                }
            }

            OnboardingStep.Verify -> {
                Text("Verify connection against /health.")
                Card(
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.surfaceVariant
                    )
                ) {
                    Column(modifier = Modifier.padding(12.dp)) {
                        Text("Host: ${state.selectedHost}:${state.selectedPort}")
                        Text("Token: ${if (state.token.isBlank()) "Not set" else "Configured"}")
                    }
                }

                if (state.busy) {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        CircularProgressIndicator(modifier = Modifier.padding(end = 8.dp))
                        Text("Checking connectivity...")
                    }
                }

                if (state.status?.contains("Connected", ignoreCase = true) == true) {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Icon(Icons.Default.CheckCircle, contentDescription = null)
                        Text("  ${state.status}")
                    }
                }

                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    TextButton(onClick = onBack) { Text("Back") }
                    Button(onClick = onVerify, enabled = !state.busy) {
                        Text("Start Chatting")
                    }
                }
            }
        }

        state.status?.let {
            Text(
                text = it,
                style = MaterialTheme.typography.bodyMedium,
                modifier = Modifier
                    .fillMaxWidth()
                    .background(
                        MaterialTheme.colorScheme.surfaceVariant,
                        shape = RoundedCornerShape(8.dp)
                    )
                    .padding(10.dp)
                    .alpha(0.95f)
            )
        }
    }
}

@Composable
private fun WorkspaceCard(
    item: DiscoveredWorkspace,
    onClick: () -> Unit
) {
    Card(
        onClick = onClick,
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.secondaryContainer)
    ) {
        Column(modifier = Modifier.padding(12.dp)) {
            Text(item.name, fontWeight = FontWeight.SemiBold)
            Text("${item.host}:${item.port}")
            Text(item.source.name)
        }
    }
}
