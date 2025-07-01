
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
}