use std::fs::File;
use std::path::Path;
use std::error::Error;

pub fn open_file(filename: &str) -> Option<File> {
    let path = Path::new(filename);
    let display = path.display();

    let mut file: Option<File> = match File::open(&path) {
        Ok(file) => Option::Some(file),
        Err(why) => {
            println!("{}", why.description());
            Option::None
        },
    };
    file
}
