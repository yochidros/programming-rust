
use std::ops::Add;
use std::ops::Neg;
use std::ops::AddAssign;
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
struct Complex<T> {
    re: T,
    im: T
}

impl<L, R, O> Add<Complex<R>> for Complex<L> where L: Add<R, Output=O> {
    type Output = Complex<O>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T, O> Neg for Complex<T> where T: Neg<Output=O> {
    type Output = Complex<O>;
    fn neg(self) -> Complex<O> {
        Complex { re: -self.re, im: -self.im }
    }
}

impl<T> AddAssign for Complex<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
    
    fn ne(&self, other: &Complex<T>) -> bool {
        self.re != other.re && self.im != other.im
    }
}
fn main() {
    let mut c = Complex { re: 1.0, im: -1.0 };
    let c2 = Complex { re: 1.0, im: -1.0 };
    c.add_assign(c2);

    if (c != Complex { re: 2.0, im: -2.0}) {
        println!("{:?}", c);
    } else if (c != Complex { re: 1.0, im: 2.0 }) {
        println!("{:?}", c2);
    }

    let mut cities = Vec::<City>::new();
    cities.push(City { name: "hello".to_string(), population: 123 });
    cities.push(City { name: "ny".to_string(), population: 12 });
    cities.push(City { name: "lo".to_string(), population: 23 });
    cities.push(City { name: "xo".to_string(), population: 13 });

    println!("{:?}", cities);
    sort_cities(&mut cities);
    let completion = move |cities: Vec<City>| { 
        thread::spawn(move || {
            println!("hello new thread!!");
            println!("{:?}", cities);
            cities
        })
    };

    start(cities, completion);
}

use std::thread;

#[derive(Debug)]
struct City {
    name: String,
    population: i64,
}

fn start<T>(cities: Vec<City>, completion: T) -> Vec<City> where T: Fn(Vec<City>) -> thread::JoinHandle<Vec<City>> {
    println!("start completion!!");
    let mut cities = cities;
    sort_cities(&mut cities);
    let result = completion(cities).join();
    println!("end completion!!");
    result.unwrap()
}
fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}
