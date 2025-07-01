// mod cli;
// mod model;
// mod crypto;
// mod storage;

// use cli::build_cli;
// use model::PasswordEntry;
// use std::io::{stdin, stdout, Write};

// fn prompt_password() -> String {
//     print!("Contraseña maestra: ");
//     stdout().flush().unwrap();
//     let mut password = String::new();
//     stdin().read_line(&mut password).unwrap();
//     password.trim().to_string()
// }
// fn generate_password(length: usize) -> String {
//     use rand::{distributions::Alphanumeric, thread_rng, Rng};
//     thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(length)
//         .map(char::from)
//         .collect()
// }
// fn copy_to_clipboard(text: &str) {
//     use arboard::Clipboard;
//     let mut clipboard = Clipboard::new().expect("No se pudo acceder al portapapeles");
//     clipboard.set_text(text.to_owned()).expect("No se pudo copiar");
// }


// fn main(){
//     let matches = build_cli().get_matches();
//     let master_password = prompt_password();
//     let mut entries = storage::load_entries(&master_password);

//    match matches.subcommand() {
//         Some(("add", sub_m)) => {
//             let service = sub_m.get_one::<String>("service").unwrap();
//             let username = sub_m.get_one::<String>("username").unwrap();
//             let password = sub_m.get_one::<String>("password").unwrap();

//             let entry = PasswordEntry::new(service, username, password);
//             entries.push(entry);
//             storage::save_entries(&entries, &master_password);
//             println!("Entrada añadida y guardada.");
//         }
//         Some(("list", _)) => {
//             for entry in &entries {
//                 println!("Servicio: {}, Usuario: {}, Contraseña: {}", entry.service, entry.username, entry.password);
//             }
//         }
//         Some(("generate", sub_m)) => {
//             let len = sub_m.get_one::<String>("length").unwrap().parse::<usize>().unwrap_or(16);
//             let pass = generate_password(len);
//             println!("Contraseña generada: {}", pass);
//         }
//         Some(("copy", sub_m)) => {
//             let service = sub_m.get_one::<String>("service").unwrap();
//             if let Some(entry) = entries.iter().find(|e| &e.service == service) {
//                 copy_to_clipboard(&entry.password);
//                 println!("Contraseña de '{}' copiada al portapapeles.", service);
//             } else {
//                 println!("Servicio no encontrado.");
//             }
//         }
//         Some(("delete", sub_m)) => {
//             let service = sub_m.get_one::<String>("service").unwrap();
//             let before = entries.len();

//             entries.retain(|e| &e.service != service);
//             if entries.len() < before {
//                 storage::save_entries(&entries, &master_password);
//                 println!("Entrada eliminada: {}", service);
//             } else {
//                 println!("No se encontró ninguna entrada para '{}'", service);
//             }
// }

//         _ => println!("Usa --help para ver los comandos disponibles"),
//     }
// }