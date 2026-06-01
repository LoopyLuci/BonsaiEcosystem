package ai.bonsai.shared;

import ai.bonsai.shared.IBonsaiCallback;

interface IBonsaiService {
    long initModel(String modelPath, String tokenizerPath);
    String chat(long handle, String prompt, float temperature);
    void generateStream(long handle, String prompt, IBonsaiCallback callback);
    boolean loadToken(in byte[] token);
    boolean verifyToken(String peerId);
    long startTransferDaemon(String configPath);
    long connectToPeer(String peerId, in byte[] token);
    void injectInput(long sessionHandle, int eventType, in byte[] data);
    void releaseHandle(long handle);
    void shutdown();
}
