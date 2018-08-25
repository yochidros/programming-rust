use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::fs::remove_file;

pub fn delete_file(filename: &str) -> Result<String,String> {
    let path = Path::new(filename);
    let display = path.display();

    let result = match remove_file(&path) {
        Ok(_) => Ok("remove success!!".to_string()),
        Err(_) => Err("cannot remove file".to_string()),
    };

    result
}