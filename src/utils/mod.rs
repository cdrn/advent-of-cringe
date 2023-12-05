use std::fs;
use std::io;
use std::path::Path;

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(e),
    }
}
