use bin::brasse_list;
use clap::Command;

mod bin {
    pub mod brasse_list;
}

fn cli() -> Command {
    Command::new("brasse")
        .about("Homebrew but better.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(brasse_list::cli())
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("list", _)) => bin::brasse_list::main(),
        _ => unreachable!(),
    };
}
