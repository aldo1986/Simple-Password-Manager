use std::fs::File;
use std::io::{Read, Write};
use crate::crypto::{encrypt_with_random_salt, decrypt_with_embedded_salt};
use crate::model::PasswordEntry;

const FILE_PATH: &str = "secrets.json.enc";

pub fn load_entries(master_password: &str) -> Vec<PasswordEntry> {
    if !std::path::Path::new(FILE_PATH).exists() {
        return vec![];
    }

    let mut file = File::open(FILE_PATH).expect("No se pudo abrir el archivo cifrado");
    let mut encrypted = vec![];
    file.read_to_end(&mut encrypted).unwrap();

    if let Some(decrypted) = decrypt_with_embedded_salt(&encrypted, master_password) {
        serde_json::from_slice(&decrypted).unwrap_or_else(|_| {
            println!("Archivo dañado o formato incorrecto.");
            vec![]
        })
    } else {
        println!("Contraseña incorrecta o archivo dañado.");
        std::process::exit(1);
    }
}

pub fn save_entries(entries: &[PasswordEntry], master_password: &str) {
    let json = serde_json::to_vec(entries).unwrap();
    let encrypted = encrypt_with_random_salt(&json, master_password);
    let mut file = File::create(FILE_PATH).expect("No se pudo crear el archivo cifrado");
    file.write_all(&encrypted).unwrap();
}