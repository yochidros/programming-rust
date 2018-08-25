pub mod open;
pub mod create;
pub mod read;
pub mod delete;

#[test]
fn test_exist_file_open() {
    let filename = "text.txt";
    let file = open::open_file(&filename);
    assert!(file.is_some()); 
}

#[test]
fn test_not_exist_file_open() {
    let filename = "not_text.txt";
    let file = open::open_file(&filename); 
    assert!(file.is_none());
}

#[test]
fn test_create_file() {
    let filename = "create_test.txt";
    let file = create::create_file(&filename);
    assert!(file.is_ok());
    match delete::delete_file(&filename) {
        Ok(msg) => { 
            println!("{}", msg);
            assert!(true)
        },
        Err(error) => {
            println!("{}", error);
            assert!(false)
        },
    }
}

#[test]
fn test_file_read() {
    let filename = "text.txt";
    let mut file = open::open_file(&filename).unwrap();
    read::read_file(&mut file);
}
