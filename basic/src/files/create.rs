use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn create_file(filename: &str) -> Result<File, String> {
    let path = Path::new(filename);

    let result = match File::create(&path) {
        Ok(file) => {
            println!("create file {}", &filename);
            Ok(file)
        }
        Err(why) => {
            println!("ERRORORORO: {}", why.description());
            Err(why.description().to_string())
        }
    };
    result
}
