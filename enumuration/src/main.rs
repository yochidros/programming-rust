
extern crate enum_primitive;


#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "mounths",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_right_matches('s')
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFutre(TimeUnit, u32),
}

#[derive(Debug)]
struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

trait StringSet {
    fn new() -> Self 
         where Self: Sized;

    fn from_slice(strings: &[&str]) -> Self
         where Self: Sized;

    fn contains(&self, string: &str) -> bool;

    fn add(&mut self, string: &str);
}


extern crate rand;
extern crate num;
use rand::Rand;
use num::Num;

use std::ops::{ Add, Mul};

fn dot<N: Num + Copy>(v1: &[N], v2: &[N]) -> N{
    let mut total = N::zero();
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1,1,1,1], &[1,1]), 2);
}
fn main() {
    println!("Hello, world!");
    println!("{}", TimeUnit::Years.plural());
    println!("{}", TimeUnit::Seconds.singular());

    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4*20 + 7);
    let three_hours_from_now = RoughTime::InTheFutre(TimeUnit::Hours, 3);

    println!("{:?}, {:?}", four_score_and_seven_years_ago, three_hours_from_now);

    let unit_sphere = Shape::Sphere { center: Point3d { x:1.0,y:1.0,z:1.0}, radius: 1.0 };
    println!("{:?}", unit_sphere);



}
