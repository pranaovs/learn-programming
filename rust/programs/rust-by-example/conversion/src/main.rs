use core::fmt;
use std::collections::btree_map::Values;
use std::convert::From;
use std::convert::Into;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::num::ParseIntError;
use std::result;
use std::str::FromStr;

#[derive(Debug)]
struct Number {
    value: i32,
}

#[derive(Debug)]
struct Number2 {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl Into<Number2> for i32 {
    fn into(self) -> Number2 {
        Number2 { value: self }
    }
}

#[derive(Debug)]
struct EvenNumber(i32);

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    // From trait use to create itself from another type

    let name = "hello";
    let name = String::from(name);
    println!("{}", name);

    let num = 32;
    let num = Number::from(num);
    println!("{}", num.value);

    // Into trait use to convert one type into another type
    // Used to specify result type if compiler is unable to infer
    // From is preferred as it gives into for free
    let num2 = 5;
    let num2: Number2 = num2.into();
    println!("number is {:?}", num2);

    // TryFrom and TryInto for fallible conversions
    let result = EvenNumber::try_from(5);
    match result {
        Ok(num) => println!("even: {}", num.0),
        Err(()) => println!("not even"),
    }

    // String conversion
    let circle = Circle { radius: 6 };
    println!("{}", circle); // println calls to_string() inline

    // Parsing a string
    let _parsed: i32 = "5".parse().unwrap(); // unwrap returns Ok() value
    let circle: Circle = "  3   ".parse().unwrap();
    println!("Circle radius from string {}", circle.radius);
}
