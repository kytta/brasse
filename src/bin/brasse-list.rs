use std::env;
use std::fs;
use std::io;

use std::path::PathBuf;

fn get_formulae() -> Result<Vec<String>, io::Error> {
    let cellar =
        PathBuf::from(env::var("HOMEBREW_CELLAR").unwrap_or_else(|_| "/opt/homebrew/Cellar".to_string()));

    let mut formulae: Vec<String> = Vec::new();

    for entry in fs::read_dir(cellar)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name();

        let formula_os_str  = match file_name {
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
    let caskroom =
        PathBuf::from("/opt/homebrew/Caskroom".to_string());

    let mut casks: Vec<String> = Vec::new();

    for entry in fs::read_dir(caskroom)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name();

        let cask_os_str  = match file_name {
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

fn main() {
    eprintln!("==> Formulae");

    for formula in get_formulae().unwrap() {
        eprintln!("{}", formula);
    }

    eprintln!("==> Casks");

    for cask in get_casks().unwrap() {
        eprintln!("{}", cask);
    }
}
