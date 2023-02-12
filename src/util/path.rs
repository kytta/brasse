use std::env;
use std::path::PathBuf;

#[cfg(target_arch = "aarch64")]
const DEFAULT_PREFIX: &str = "/opt/homebrew/";

#[cfg(target_arch = "x86_64")]
const DEFAULT_PREFIX: &str = "/usr/local/";

pub fn get_prefix() -> PathBuf {
    PathBuf::from(env::var("HOMEBREW_PREFIX").unwrap_or_else(|_| String::from(DEFAULT_PREFIX)))
}

pub fn get_cellar() -> PathBuf {
    match env::var("HOMEBREW_CELLAR") {
        Ok(path) => PathBuf::from(path),
        Err(_) => get_prefix().join("Cellar"),
    }
}

pub fn get_caskroom() -> PathBuf {
    get_prefix().join("Caskroom")
}
