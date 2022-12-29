use clap::{command, Command};

mod cmd {
    pub mod list;
}
mod util;

fn cli() -> Command {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(cmd::list::cli())
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("list", submatches)) => cmd::list::main(submatches),
        _ => unreachable!(),
    };
}
