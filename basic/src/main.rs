
use std::cmp::Ordering;

mod functions;
mod files;

fn main() {
    let language: Vec<String> = std::env::args().skip(1).collect();
    functions::print_args_type(&language);
    functions::print_padovan();
    functions::print_composer();

    functions::owenership_value();
    functions::reference_values();
    functions::insert_ref(true);

    functions::execute_table();
    functions::reference_reference();
    functions::execute_factorial();

    let x = 10;
    functions::g(&x);
    println!("{}", x);

    let samples: [i32; 4] = [2, 4, 32, 0];
    let smallest = functions::smallest(&samples);
    println!("{}", smallest);

    let mut v = Vec::new();
    v.push("hello".to_string());
    v.push("hello".to_string());
    v.push("hello".to_string());
    v.push("hello".to_string());
    v.push("h".to_string());

    let sa =  functions::StringTable { elements: v};

    let find = sa.find_by_prefix("h");
    match find {
        Some(value) => println!("{}", value),
        None => assert!(false),
    }

    println!("{}",{ 1 });

    println!("{}",{ 1; 0 });

    let name;
    if "h" == "h" {
        name = "h"
    } else {
        name = "unknwon"
    }

    println!("{}", name);
    show_files();

    let filename = "rustv4.txt";

    let mut file = match files::open::open_file(&filename) {
        Some(file) => file,
        None => match files::create::create_file(&filename) {
            Ok(file) => files::open::open_file(&filename).unwrap(),
            Err(why) => panic!("{}",why)
        }
    };
    files::read::read_file(&mut file);
}

fn show_files() {
    let mut v = vec![];
    v.push("hell");
    v.push("dog");
    v.push("cat");
    v.push("dog");

    fn cmp_name(a: &str, b: &str) -> Ordering {
       a.cmp(&b) 
    }

    let order = cmp_name(&v[1], &v[3]);

    match order {
        Ordering::Less => println!("Less"),
        Ordering::Equal => println!("Equal"),
        Ordering::Greater => println!("Greater"),
    }
}