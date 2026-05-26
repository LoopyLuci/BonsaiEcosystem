use argon2::Argon2;
use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};
use ed25519_dalek::{SigningKey};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use sha2::Sha256;
use zeroize::Zeroize;
use std::collections::HashMap;
use x25519_dalek::{EphemeralSecret, PublicKey as XPublicKey};

pub struct Session {
    pub profile_id: String,
    pub workspace_key: Vec<u8>,
    pub signing_key: SigningKey,
}

impl Drop for Session {
    fn drop(&mut self) {
        self.workspace_key.zeroize();
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub display_name: String,
    pub identity_pub: Vec<u8>,
    pub encrypted_workspace_key: Vec<u8>,
    pub argon2_salt: [u8; 32],
    pub argon2_memory_kb: u32,
    pub argon2_iterations: u32,
    pub created_at: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub access_grants: Vec<AccessGrant>,
    pub created_at: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AccessGrant {
    pub grantee_pub: Vec<u8>,
    pub encrypted_key: Vec<u8>,
    pub permissions: u8,
}

pub fn derive_key(passphrase: &str, salt: &[u8; 32], _memory: u32, _iterations: u32) -> [u8; 32] {
    let mut key = [0u8; 32];
    // Use Argon2 default; this is a simple wrapper for demonstration
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .expect("argon2 derive");
    key
}

pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce: [u8; 12] = rand::random();
    let ct = cipher.encrypt(&nonce.into(), plaintext).unwrap();
    [nonce.to_vec(), ct].concat()
}

pub fn decrypt(key: &[u8; 32], ciphertext: &[u8]) -> Option<Vec<u8>> {
    if ciphertext.len() < 12 { return None; }
    let (nonce, ct) = ciphertext.split_at(12);
    Aes256Gcm::new_from_slice(key).ok()?.decrypt(nonce.into(), ct).ok()
}

pub fn create_profile(passphrase: &str, display_name: &str) -> (UserProfile, SigningKey) {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let salt: [u8; 32] = rand::random();
    let wk: [u8; 32] = rand::random();
    let key = derive_key(passphrase, &salt, 65536, 3);
    let enc_wk = encrypt(&key, &wk);
    let profile = UserProfile {
        id: uuid::Uuid::new_v4().to_string(),
        display_name: display_name.into(),
        identity_pub: signing_key.verifying_key().to_bytes().to_vec(),
        encrypted_workspace_key: enc_wk,
        argon2_salt: salt,
        argon2_memory_kb: 65536,
        argon2_iterations: 3,
        created_at: chrono::Utc::now().timestamp_millis(),
    };
    (profile, signing_key)
}

pub fn unlock_profile(profile: &UserProfile, passphrase: &str) -> Option<Session> {
    let key = derive_key(passphrase, &profile.argon2_salt, profile.argon2_memory_kb, profile.argon2_iterations);
    let wk = decrypt(&key, &profile.encrypted_workspace_key)?;
    // Placeholder for signing key retrieval
    let sk_bytes = [0u8; 32];
    let sk = SigningKey::from_bytes(&sk_bytes).ok()?;
    Some(Session {
        profile_id: profile.id.clone(),
        workspace_key: wk,
        signing_key: sk,
    })
}

pub fn create_workspace(session: &Session, name: &str) -> Workspace {
    Workspace {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.into(),
        owner_id: session.profile_id.clone(),
        access_grants: vec![],
        created_at: chrono::Utc::now().timestamp_millis(),
    }
}

pub fn grant_access(session: &Session, workspace: &mut Workspace, grantee_pub: &[u8], permissions: u8) {
    // Ephemeral X25519 exchange (placeholder)
    let ephemeral_secret = EphemeralSecret::random(&mut OsRng);
    let ephemeral_pub = XPublicKey::from(&ephemeral_secret);
    let shared_secret = ephemeral_secret.diffie_hellman(&XPublicKey::from([0u8; 32]));
    let enc_key = encrypt(shared_secret.as_bytes(), &session.workspace_key);
    workspace.access_grants.push(AccessGrant {
        grantee_pub: grantee_pub.to_vec(),
        encrypted_key: enc_key,
        permissions,
    });
}
