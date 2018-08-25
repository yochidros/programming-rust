
extern crate fern_sim;

use fern_sim::{ Fern, run_simulation};
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
    functions::show_files();

    let filename = "rustv4.txt";

    let mut file = match files::open::open_file(&filename) {
        Some(file) => file,
        None => match files::create::create_file(&filename) {
            Ok(_file) => files::open::open_file(&filename).unwrap(),
            Err(why) => panic!("{}",why)
        }
    };

    files::read::read_file(&mut file);

    match files::delete::delete_file(&filename) {
        Ok(msg) => println!("{}", msg),
        Err(desc) => panic!("{}", desc),
    }

    println!("{:?}", create_vec(3));

    let closure = |x: u64| -> bool {x % 2 == 0 };
    println!("{}", closure(2));

    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001,
    };

    println!("{}", fern.size);
    run_simulation(&mut fern, 100);
    println!("{}", fern.size)

}


fn create_vec(capcity: usize) -> Vec<i32> {
    return Vec::<i32>::with_capacity(capcity);
}
