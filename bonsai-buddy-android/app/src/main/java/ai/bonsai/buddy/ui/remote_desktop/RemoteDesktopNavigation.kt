package ai.bonsai.buddy.ui.remote_desktop

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable

/**
 * Navigation destination for remote desktop screen.
 *
 * Route format: remote_desktop/{peerId}/{tokenBase64}
 *
 * Example:
 * navController.navigate("remote_desktop/peer-001/dGVzdHRva2Vu")
 */
fun NavGraphBuilder.remoteDesktopRoute() {
    composable(
        route = "remote_desktop/{peerId}/{tokenBase64}",
        arguments = listOf(
            androidx.navigation.navArgument("peerId") { type = androidx.navigation.NavType.StringType },
            androidx.navigation.navArgument("tokenBase64") { type = androidx.navigation.NavType.StringType }
        )
    ) { backStackEntry ->
        val peerId = backStackEntry.arguments?.getString("peerId") ?: ""
        val tokenBase64 = backStackEntry.arguments?.getString("tokenBase64") ?: ""

        RemoteDesktopScreen(
            peerId = peerId,
            tokenBase64 = tokenBase64,
            onNavigateBack = {
                backStackEntry.navController.navigateUp()
            }
        )
    }
}

/**
 * Navigate to remote desktop screen.
 *
 * @param peerId Peer identifier
 * @param tokenBase64 Base64-encoded capability token
 */
fun NavController.navigateToRemoteDesktop(peerId: String, tokenBase64: String) {
    navigate("remote_desktop/$peerId/$tokenBase64")
}
