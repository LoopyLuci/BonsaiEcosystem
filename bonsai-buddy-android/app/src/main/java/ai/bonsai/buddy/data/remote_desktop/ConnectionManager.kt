package ai.bonsai.buddy.data.remote_desktop

import android.content.Context
import android.content.SharedPreferences
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import kotlinx.serialization.encodeToString

/**
 * Manages peer discovery and pairing for remote desktop connections.
 *
 * Features:
 * - List available BRDF peers on local network
 * - Query peer information (name, capabilities, status)
 * - Generate capability tokens for pairing
 * - Save/load paired devices (SharedPreferences)
 * - Auto-connect to recently used devices
 * - QR code generation for pairing
 * - Network error handling
 */
class ConnectionManager(context: Context) {
    private val sharedPrefs: SharedPreferences = context.getSharedPreferences(
        "remote_desktop_prefs",
        Context.MODE_PRIVATE
    )

    private val _availablePeers = MutableStateFlow<List<RemoteDesktopPeer>>(emptyList())
    val availablePeers: StateFlow<List<RemoteDesktopPeer>> = _availablePeers.asStateFlow()

    private val _pairedDevices = MutableStateFlow<List<RemoteDesktopPeer>>(emptyList())
    val pairedDevices: StateFlow<List<RemoteDesktopPeer>> = _pairedDevices.asStateFlow()

    private val _isDiscovering = MutableStateFlow(false)
    val isDiscovering: StateFlow<Boolean> = _isDiscovering.asStateFlow()

    init {
        loadPairedDevices()
    }

    /**
     * Start discovering available peers on the local network.
     *
     * Uses mDNS/Bonjour to find BRDF servers.
     */
    suspend fun startDiscovery() = withContext(Dispatchers.IO) {
        if (_isDiscovering.value) {
            return@withContext
        }

        _isDiscovering.value = true
        try {
            // In a real implementation, this would use NsdManager or similar
            // for local network discovery. For now, we simulate it.
            val discoveredPeers = simulatePeerDiscovery()
            _availablePeers.value = discoveredPeers
        } catch (e: Exception) {
            // Handle discovery error
        } finally {
            _isDiscovering.value = false
        }
    }

    /**
     * Stop peer discovery.
     */
    suspend fun stopDiscovery() = withContext(Dispatchers.IO) {
        _isDiscovering.value = false
    }

    /**
     * Get information about a specific peer.
     *
     * @param peerId Peer identifier
     * @return Peer information or null if not found
     */
    suspend fun getPeerInfo(peerId: String): RemoteDesktopPeer? = withContext(Dispatchers.IO) {
        return@withContext _availablePeers.value.find { it.peerId == peerId }
            ?: _pairedDevices.value.find { it.peerId == peerId }
    }

    /**
     * Generate a capability token for a peer.
     *
     * @param peerId Peer identifier
     * @param permissions Requested permissions (default: video, input)
     * @param expiresIn Expiration time in milliseconds (default: 24 hours)
     * @return Capability token
     */
    suspend fun generateToken(
        peerId: String,
        permissions: Set<String> = setOf("video", "input"),
        expiresIn: Long = 24 * 60 * 60 * 1000
    ): RemoteDesktopToken = withContext(Dispatchers.Default) {
        val now = System.currentTimeMillis()
        return@withContext RemoteDesktopToken(
            peerId = peerId,
            permissions = permissions,
            issuedAt = now,
            expiresAt = now + expiresIn,
            capabilityData = generateCapabilityToken()
        )
    }

    /**
     * Add a device to the paired devices list.
     *
     * @param peer Device to pair
     */
    suspend fun pairDevice(peer: RemoteDesktopPeer) = withContext(Dispatchers.IO) {
        val currentPaired = _pairedDevices.value.toMutableList()
        if (!currentPaired.any { it.peerId == peer.peerId }) {
            currentPaired.add(peer)
            _pairedDevices.value = currentPaired
            savePairedDevices()
        }
    }

    /**
     * Remove a device from the paired devices list.
     *
     * @param peerId Peer identifier to unpair
     */
    suspend fun unpairDevice(peerId: String) = withContext(Dispatchers.IO) {
        val currentPaired = _pairedDevices.value.toMutableList()
        currentPaired.removeAll { it.peerId == peerId }
        _pairedDevices.value = currentPaired
        savePairedDevices()
    }

    /**
     * Get the most recently used device.
     *
     * @return Most recently used peer, or null if no history
     */
    fun getLastUsedDevice(): RemoteDesktopPeer? {
        val lastPeerId = sharedPrefs.getString("last_peer_id", null) ?: return null
        return _pairedDevices.value.find { it.peerId == lastPeerId }
    }

    /**
     * Record that a peer was just used.
     *
     * @param peerId Peer identifier
     */
    suspend fun recordPeerUsage(peerId: String) = withContext(Dispatchers.IO) {
        sharedPrefs.edit().putString("last_peer_id", peerId).apply()
    }

    /**
     * Generate a QR code string for pairing.
     *
     * @param peer Peer to generate QR code for
     * @return QR code content string
     */
    fun generatePairingQrCode(peer: RemoteDesktopPeer): String {
        // QR code format: brdf://peer/{peerId}/{peerName}
        return "brdf://peer/${peer.peerId}/${peer.hostname ?: peer.name}"
    }

    /**
     * Parse a QR code for pairing.
     *
     * @param qrContent QR code content
     * @return Peer information or null if invalid
     */
    suspend fun parsePairingQrCode(qrContent: String): RemoteDesktopPeer? = withContext(Dispatchers.Default) {
        if (!qrContent.startsWith("brdf://peer/")) {
            return@withContext null
        }

        val parts = qrContent.removePrefix("brdf://peer/").split("/")
        if (parts.size < 2) {
            return@withContext null
        }

        val peerId = parts[0]
        val peerName = parts[1]

        return@withContext RemoteDesktopPeer(
            peerId = peerId,
            name = peerName,
            hostname = parts.getOrNull(2),
            isOnline = false
        )
    }

    private fun loadPairedDevices() {
        val json = sharedPrefs.getString("paired_devices", "[]") ?: "[]"
        try {
            val devices = Json.decodeFromString<List<RemoteDesktopPeer>>(json)
            _pairedDevices.value = devices
        } catch (e: Exception) {
            _pairedDevices.value = emptyList()
        }
    }

    private fun savePairedDevices() {
        try {
            val json = Json.encodeToString(_pairedDevices.value)
            sharedPrefs.edit().putString("paired_devices", json).apply()
        } catch (e: Exception) {
            // Ignore serialization errors
        }
    }

    private fun generateCapabilityToken(): String {
        // In a real implementation, this would generate a cryptographic token
        // For now, return a placeholder
        val timestamp = System.currentTimeMillis()
        return "cap_${timestamp}_${(0..9999).random()}"
    }

    private fun simulatePeerDiscovery(): List<RemoteDesktopPeer> {
        // In a real implementation, this would use NsdManager or similar
        // For demo, return some simulated peers
        return listOf(
            RemoteDesktopPeer(
                peerId = "peer-001",
                name = "Desktop 1",
                hostname = "desktop1.local",
                osType = "Windows",
                resolution = Resolution(1920, 1080, 60),
                isOnline = true,
                lastSeen = System.currentTimeMillis()
            ),
            RemoteDesktopPeer(
                peerId = "peer-002",
                name = "Laptop",
                hostname = "laptop.local",
                osType = "macOS",
                resolution = Resolution(2560, 1600, 60),
                isOnline = true,
                lastSeen = System.currentTimeMillis()
            )
        )
    }
}

/**
 * Extension function to check if a token is still valid.
 */
fun RemoteDesktopToken.isValid(): Boolean {
    return System.currentTimeMillis() < expiresAt
}

/**
 * Extension function to get remaining validity time.
 */
fun RemoteDesktopToken.getTimeRemaining(): Long {
    val remaining = expiresAt - System.currentTimeMillis()
    return if (remaining > 0) remaining else 0
}
