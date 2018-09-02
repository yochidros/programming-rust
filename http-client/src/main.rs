extern crate reqwest;

use std::error::Error;
use std::io::{self, Write};

fn http_get_main(url: &str) -> Result<(), Box<Error>> {
    let mut response = reqwest::get(url)?;
    if !response.status().is_success() {
        Err(format!("{}", response.status()))?;
    }

    let stdout = io::stdout();
    io::copy(&mut response, &mut stdout.lock())?;
    
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        writeln!(io::stderr(), "usega: http-get URL").unwrap();
        return;
    }

    if let Err(err) = http_get_main(&args[0]) {
        writeln!(io::stderr(), "error: {}", err).unwrap();
    }
}
