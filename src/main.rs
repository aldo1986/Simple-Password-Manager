mod cli;
mod model;

use cli::build_cli;
use model::PasswordEntry;

fn main(){
    let matches = build_cli().get_matches();

    match matches.subcommand(){
        Some(("add", sub_m)) => {
            let service = sub_m.get_one::<String>("service").unwrap();
            let username = sub_m.get_one::<String>("username").unwrap();
            let password = sub_m.get_one::<String>("password").unwrap();

            let entry = PasswordEntry::new(service,username,password);
            println!("Se agrego Entrada de Password {:?}", entry);
        }
        Some(("list", _)) => {
            println!("Aun no se implementa :(");
        }
        _ => println!("Introduszca un comando utiliza --help para ver la lista de comandos"),
    }
}