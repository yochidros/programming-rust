
#[derive(Debug)]
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
} 

impl Extrema {
    pub fn new(&self) -> Self {
        Extrema { greatest: 0, least: 0 }
    }
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1 .. slice.len() {
        if slice[i] < *least { least = &slice[i] ;}
        if slice[i] > *greatest { greatest = &slice[i] ;}
    }

    Extrema { greatest, least }
}