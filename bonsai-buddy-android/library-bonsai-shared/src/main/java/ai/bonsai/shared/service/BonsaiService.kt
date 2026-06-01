package ai.bonsai.shared.service

import android.app.Service
import android.content.Intent
import android.os.IBinder
import ai.bonsai.shared.IBonsaiCallback
import ai.bonsai.shared.IBonsaiService

class BonsaiService : Service() {
    private val binder = BonsaiServiceImpl(this)

    init {
        System.loadLibrary("bonsai_android")
    }

    override fun onBind(intent: Intent): IBinder = binder.asBinder()

    override fun onDestroy() {
        super.onDestroy()
        binder.shutdown()
    }
}

class BonsaiServiceImpl(service: BonsaiService) : IBonsaiService.Stub() {
    private var modelHandle: Long = 0
    private var transferDaemonHandle: Long = 0

    override fun initModel(modelPath: String, tokenizerPath: String): Long {
        modelHandle = nativeInitModel(modelPath, tokenizerPath)
        return modelHandle
    }

    override fun chat(handle: Long, prompt: String, temperature: Float): String {
        return nativeChat(handle, prompt, temperature)
    }

    override fun generateStream(handle: Long, prompt: String, callback: IBonsaiCallback) {
        nativeGenerateStream(handle, prompt, object : StreamCallback {
            override fun onToken(token: String) {
                try { callback.onToken(token) } catch (e: Exception) { }
            }
            override fun onComplete() {
                try { callback.onComplete() } catch (e: Exception) { }
            }
            override fun onError(error: String) {
                try { callback.onError(error) } catch (e: Exception) { }
            }
        })
    }

    override fun loadToken(token: ByteArray): Boolean {
        return nativeLoadToken(token)
    }

    override fun verifyToken(peerId: String): Boolean {
        return nativeVerifyToken(peerId)
    }

    override fun startTransferDaemon(configPath: String): Boolean {
        transferDaemonHandle = nativeStartTransferDaemon(configPath)
        return transferDaemonHandle != 0L
    }

    override fun connectToPeer(peerId: String, token: ByteArray): Long {
        return nativeConnectToPeer(peerId, token)
    }

    override fun injectInput(sessionHandle: Long, eventType: Int, data: ByteArray) {
        nativeInjectInput(sessionHandle, eventType, data)
    }

    override fun releaseHandle(handle: Long) {
        nativeReleaseHandle(handle)
    }

    override fun shutdown() {
        if (modelHandle != 0L) nativeReleaseHandle(modelHandle)
        if (transferDaemonHandle != 0L) nativeReleaseHandle(transferDaemonHandle)
    }

    private external fun nativeInitModel(modelPath: String, tokenizerPath: String): Long
    private external fun nativeChat(handle: Long, prompt: String, temperature: Float): String
    private external fun nativeGenerateStream(handle: Long, prompt: String, callback: StreamCallback)
    private external fun nativeLoadToken(token: ByteArray): Boolean
    private external fun nativeVerifyToken(peerId: String): Boolean
    private external fun nativeStartTransferDaemon(configPath: String): Long
    private external fun nativeConnectToPeer(peerId: String, token: ByteArray): Long
    private external fun nativeInjectInput(sessionHandle: Long, eventType: Int, data: ByteArray)
    private external fun nativeReleaseHandle(handle: Long)

    interface StreamCallback {
        fun onToken(token: String)
        fun onComplete()
        fun onError(error: String)
    }
}
