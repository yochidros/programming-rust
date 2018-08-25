use std::fs::File;
use std::io::Read;
use std::error::Error;

pub fn read_file(file: &mut File) {
    let mut s = String::new();

    match &file.read_to_string(&mut s) {
        Ok(_) => println!("contents is: {}", s),
        Err(why) => panic!("couldn't read file {}", why.description()),
    }
}