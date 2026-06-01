package ai.bonsai.buddy.ui

import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.os.IBinder
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import ai.bonsai.shared.IBonsaiCallback
import ai.bonsai.shared.IBonsaiService
import ai.bonsai.shared.db.ChatHistoryEntity

data class Message(
    val text: String,
    val isUser: Boolean,
    val timestamp: Long = System.currentTimeMillis()
)

class ChatViewModel(context: Context) : ViewModel() {
    private val _messages = MutableStateFlow<List<Message>>(listOf(
        Message("Hi! I'm Bonsai, your AI assistant. How can I help you today?", false),
        Message("Ask me to:\n• Answer questions\n• Help with coding\n• Control your devices\n• Manage your projects", false)
    ))
    val messages = _messages.asStateFlow()
    
    var inputText: String = ""
    
    private var bonsaiService: IBonsaiService? = null
    private var modelHandle: Long = 0L
    
    private val connection = object : ServiceConnection {
        override fun onServiceConnected(name: ComponentName, service: IBinder) {
            bonsaiService = IBonsaiService.Stub.asInterface(service)
            // Initialize model
            viewModelScope.launch {
                val modelPath = "/storage/emulated/0/Bonsai/models/default.gguf"
                val tokenizerPath = "/storage/emulated/0/Bonsai/models/tokenizer.model"
                try {
                    modelHandle = bonsaiService?.initModel(modelPath, tokenizerPath) ?: 0L
                } catch (e: Exception) {
                    _messages.value = _messages.value + Message("Error: Could not load model. ${e.message}", false)
                }
            }
        }
        override fun onServiceDisconnected(name: ComponentName) {
            bonsaiService = null
        }
    }
    
    init {
        val intent = Intent(context, ai.bonsai.shared.service.BonsaiService::class.java)
        context.bindService(intent, connection, Context.BIND_AUTO_CREATE)
    }
    
    fun sendMessage() {
        val userMessage = inputText.trim()
        if (userMessage.isBlank()) return
        
        _messages.value = _messages.value + Message(userMessage, true)
        inputText = ""
        
        viewModelScope.launch {
            try {
                val response = bonsaiService?.chat(modelHandle, userMessage, 0.7f) 
                    ?: "Error: BonsaiService not available"
                _messages.value = _messages.value + Message(response, false)
            } catch (e: Exception) {
                _messages.value = _messages.value + Message("Error: ${e.message}", false)
            }
        }
    }
    
    override fun onCleared() {
        super.onCleared()
        try {
            // Clean up
        } catch (e: Exception) {}
    }
}
