use std::fs::File;
use std::path::Path;
use std::error::Error;

pub fn create_file(filename: &str) -> Result<File, String> {
   let path = Path::new(filename);
   let display = path.display();

   let result = match File::create(&path) {
       Ok(file) => {
           println!("create file {}", &filename);
           Ok(file)},
       Err(why) => {
           println!("ERRORORORO: {}", why.description());
           Err(why.description().to_string())
       }
   };
   result
}