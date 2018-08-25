
mod functions;

fn main() {
    let language: Vec<String> = std::env::args().skip(1).collect();
    functions::print_args_type(&language);
    functions::print_padovan();
    functions::print_composer();

    functions::owenership_value();
    functions::reference_values();
    functions::insert_ref(true);

    functions::execute_table();
    reference_reference();
    execute_factorial();

    let x = 10;
    g(&x);
    println!("{}", x);

    let samples: [i32; 4] = [2, 4, 32, 0];
    let smallest = smallest(&samples);
    println!("{}", smallest);

    let mut v = Vec::new();
    v.push("hello".to_string());
    v.push("hello".to_string());
    v.push("hello".to_string());
    v.push("hello".to_string());
    v.push("h".to_string());

    let sa = StringTable { elements: v};

    let find = sa.find_by_prefix("hfjdksafljd").unwrap();
}

fn reference_reference() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32
    }

    let point = Point { x: 1000, y: 200 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    println!("point x:{}, y:{}", rrr.x, rrr.y);
    assert!(rr.y == rrr.y);
    assert!(r.x == rrr.x);
    let x = 10;
    let y = 10;

    let rx = &x;
    let rrx = &rx;
}

fn factorial(n: usize)  -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}

fn execute_factorial() {
    let r = &factorial(6);

    assert_eq!(r + &1009, 1729);
}

fn g<'a> (p: &'a i32) {
    println!("{}",p );
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r ;}
    }
    s
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0 .. self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}
