
use std::io::{ self, Read, Write, ErrorKind };
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::PathBuf;

use std::fs::OpenOptions;

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

fn main() {
    let filename = std::path::Path::new("hello.txt");
    let file = File::open(filename).unwrap();

    let result = grep_main();
    if let Err(err) = result {
        let _ = writeln!(io::stderr(), "{}", err);
    }

    let log = OpenOptions::new().append(true).open("hello.txt");

    let file = OpenOptions::new().write(true).create_new(true).open("new_hello.txt");
    

}

pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64> where R: Read, W: Write {
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;

    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        writer.write_all(&buf[..len]);
        written += len as u64;
    }
}

fn grep<R>(target: &str, reader: R) -> io::Result<()> where R: BufRead {
   for line_result in reader.lines() {
       let line = line_result?;
       if line.contains(target) {
           println!("find {}!!", line);
       }
   } 
   Ok(())
}

fn grep_main() -> Result<(), Box<Error>>  {
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE...")?
    };

    let files: Vec<PathBuf> = args.map(PathBuf::from).collect(); 
    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, io::BufReader::new(f))?;
        }
    }

    Ok(())
}