use crate::homebrew::get_cellar;
use crate::util::filesystem::get_directories;
use std::io;

pub fn get_formulae() -> Result<Vec<String>, io::Error> {
    let cellar = get_cellar();

    get_directories(cellar)
}
