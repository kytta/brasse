use crate::homebrew::get_caskroom;
use crate::util::filesystem::get_directories;
use std::io;

pub fn get_casks() -> Result<Vec<String>, io::Error> {
    let caskroom = get_caskroom();

    get_directories(caskroom)
}
