
use clap::{Arg, ArgMatches, Command};
pub fn build_cli() -> clap::Command{
    clap::Command::new("Manejador de Passwords")
    .version("0.1.0")
        .author("Aldo E. Fuentes Millan <aldo.fuentes@gmail.com>")
        .about("Gestor de contraseñas local y seguro")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Añadir una nueva entrada")
                .arg(Arg::new("service").required(true).help("Nombre del servicio"))
                .arg(Arg::new("username").required(true).help("Usuario"))
                .arg(Arg::new("password").required(true).help("Contraseña")),
        )
        .subcommand(Command::new("list").about("Listar todas las entradas"))

        .subcommand(
            Command::new("list")
                .about("Lista las entradas guardadas"),
        )
        .subcommand(
            Command::new("generate")
                .about("Genera una contraseña segura")
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .default_value("16")
                        .help("Longitud de la contraseña generada"),
                ),
        )
        .subcommand(
            Command::new("copy")
                .about("Copia al portapapeles la contraseña de un servicio")
                .arg(
                    Arg::new("service")
                        .required(true)
                        .help("Nombre del servicio"),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Elimina una entrada de servicio")
                .arg(
                    Arg::new("service")
                        .required(true)
                        .help("Nombre del servicio a eliminar"),
                ),
        )
}