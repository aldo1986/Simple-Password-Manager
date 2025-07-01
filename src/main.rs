mod cli;
mod model;
mod crypto;
mod storage;

use cli::build_cli;
use model::PasswordEntry;
use std::io::{stdin, stdout, Write};

fn prompt_password() -> String {
    print!("Contraseña maestra: ");
    stdout().flush().unwrap();
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    password.trim().to_string()
}


fn main(){
    let matches = build_cli().get_matches();
    let master_password = prompt_password();
    let mut entries = storage::load_entries(&master_password);

   match matches.subcommand() {
        Some(("add", sub_m)) => {
            let service = sub_m.get_one::<String>("service").unwrap();
            let username = sub_m.get_one::<String>("username").unwrap();
            let password = sub_m.get_one::<String>("password").unwrap();

            let entry = PasswordEntry::new(service, username, password);
            entries.push(entry);
            storage::save_entries(&entries, &master_password);
            println!("Entrada añadida y guardada.");
        }
        Some(("list", _)) => {
            for entry in &entries {
                println!("Servicio: {}, Usuario: {}, Contraseña: {}", entry.service, entry.username, entry.password);
            }
        }
        _ => println!("Usa --help para ver los comandos disponibles"),
    }
}