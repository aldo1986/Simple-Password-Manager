use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{password_hash::Salt, Argon2, PasswordHasher};
use rand::{rngs::OsRng, RngCore};

/// Deriva una clave de 256 bits desde la contraseña y el salt usando Argon2.
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("Error derivando la clave");
    key
}

/// Cifra los datos y devuelve: salt + nonce + ciphertext
pub fn encrypt_with_random_salt(data: &[u8], password: &str) -> Vec<u8> {
    // Generar salt y nonce
    let mut salt = [0u8; 16];
    let mut nonce = [0u8; 12];
    let mut rng = OsRng;
    rng.fill_bytes(&mut salt);
    rng.fill_bytes(&mut nonce);

    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new(&key.into());
    let nonce_obj = Nonce::from_slice(&nonce);

    let ciphertext = cipher.encrypt(nonce_obj, data).expect("Fallo al cifrar");

    // salt + nonce + ciphertext
    [salt.to_vec(), nonce.to_vec(), ciphertext].concat()
}

/// Descifra los datos cifrados con formato: salt + nonce + ciphertext
pub fn decrypt_with_embedded_salt(encrypted_data: &[u8], password: &str) -> Option<Vec<u8>> {
    if encrypted_data.len() < 28 {
        return None;
    }

    let salt = &encrypted_data[0..16];
    let nonce = &encrypted_data[16..28];
    let ciphertext = &encrypted_data[28..];

    let key = derive_key(password, salt);
    let cipher = Aes256Gcm::new(&key.into());
    let nonce_obj = Nonce::from_slice(nonce);

    cipher.decrypt(nonce_obj, ciphertext).ok()
}