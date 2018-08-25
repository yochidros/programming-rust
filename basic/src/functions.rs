
use std::collections::HashMap;
use std::rc::Rc;

type Table = HashMap<String, Vec<String>>;

pub struct Person {
    pub name: String,
    pub birth: i32
}

pub fn print_padovan() {
    let mut padovan = vec![1,1,1];
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}

pub fn print_composer() {
    let mut composers = Vec::new();

    composers.push(Person{ name: "Palastina".to_string(), birth: 1532 });
    composers.push(Person{ name: "Dowland".to_string(), birth: 3829 });
    composers.push(Person{ name: "Lully".to_string(), birth: 23981 });
    composers.push(Person{ name: "dogggging".to_string(), birth: 43298 });

    for composer in composers {
        println!("{}, borned {}", composer.name, composer.birth);

    }
}

pub fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!("    {}", work);
        }
    }
}

pub fn execute_table() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                vec!["many madrigals".to_string(), "helop".to_string()]);
    table.insert("Caravvagino".to_string(),
                vec!["music music".to_string(), "make song".to_string()]);
    table.insert("Doggiiiige".to_string(),
                vec!["madscientist".to_string(), "gahahahaha".to_string()]);
    show(&table);
    sort_works(&mut table);
    show(&table);
}

pub fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

pub fn insert_ref(b: bool) {
    let x = 10;
    let y = 20;
    let mut r = &x;

    if b { r = &y; }

    assert!(*r == 10 || *r == 20);
    println!("{}", *r);
}

pub fn print_args_type(args: &Vec<String>) {
    if args.len() == 0 { return }
    for l in args {
        println!("{}: {}", l, 
        if l.len() % 2 == 0 {
            "functional"
        } else {
            "imperative"
        });
    }
}

pub fn owenership_value() {
    let mut x = vec![10, 20, 30];

    // x's item moved 'item'    
    // for item in x {
    //     println!("{}", item);
    // }

    // x's item not moved 'item'    
    // so, x can move owenership
    for item in x.iter() {
        println!("{}", item);
    }
}

pub fn reference_values() {
    let rs = Rc::new("shirataki".to_string());
    let rt = rs.clone();
    let mut ru = rs.clone();

    ru = Rc::new("ninjin".to_string());

    println!("{:?}, {:?}", rs, ru);
}
