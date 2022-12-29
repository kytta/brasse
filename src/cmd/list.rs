use crate::util::output::get_grid;
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::{env, fs, io, path::PathBuf};

pub fn cli() -> Command {
    Command::new("list")
        .bin_name("brasse-list")
        .about("List installed formulae and casks")
        .arg(
            Arg::new("oneline")
                .short('1')
                .help("Force output to be one entry per line.")
                .action(ArgAction::SetTrue),
        )
}

fn get_formulae() -> Result<Vec<String>, io::Error> {
    let cellar = PathBuf::from(
        env::var("HOMEBREW_CELLAR").unwrap_or_else(|_| "/opt/homebrew/Cellar".to_string()),
    );

    let mut formulae: Vec<String> = Vec::new();

    for entry in fs::read_dir(cellar)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name();

        let formula_os_str = match file_name {
            Some(f) => f,
            None => continue,
        };

        let formula_name = match formula_os_str.to_str() {
            Some(name) => name,
            None => continue,
        };

        if formula_name.starts_with('.') {
            continue;
        }

        formulae.push(formula_name.to_string())
    }
    formulae.sort();

    Ok(formulae)
}

fn get_casks() -> Result<Vec<String>, io::Error> {
    let caskroom = PathBuf::from("/opt/homebrew/Caskroom".to_string());

    let mut casks: Vec<String> = Vec::new();

    for entry in fs::read_dir(caskroom)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name();

        let cask_os_str = match file_name {
            Some(f) => f,
            None => continue,
        };

        let cask_name = match cask_os_str.to_str() {
            Some(name) => name,
            None => continue,
        };

        if cask_name.starts_with('.') {
            continue;
        }

        casks.push(cask_name.to_string())
    }
    casks.sort();

    Ok(casks)
}

pub fn main(submatches: &ArgMatches) {
    let one_line = submatches.get_flag("oneline");

    eprintln!("==> Formulae");
    eprintln!("{}", get_grid(get_formulae().unwrap(), one_line));
    eprintln!("==> Casks");
    eprintln!("{}", get_grid(get_casks().unwrap(), one_line));
}