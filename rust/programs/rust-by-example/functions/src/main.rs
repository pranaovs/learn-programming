use std::fs::FileTimes;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> i32 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        (x2 - x1).abs() * (y2 - y1).abs()
    }

    fn perimeter(&self) -> i32 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2 * ((x2 - x1).abs() + (y2 - y1).abs())
    }
}

fn main() {
    fizzbuzz_to(100);

    let rect = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3, 4),
    };

    println!("Area: {}", rect.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1, 1),
    };

    square.p2 = Point::new(5, 5);

    let outer_var = 10;
    let function = |i: i32| i + outer_var;

    println!("Function result: {}", function(5));
}

fn divisible_by(lhs: u32, rhs: u32) -> bool {
    lhs.is_multiple_of(rhs)
}

fn fizzbuzz(n: u32) {
    if divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if divisible_by(n, 3) {
        println!("fizz");
    } else if divisible_by(n, 5) {
        println!("buzz");
    }
}

fn fizzbuzz_to(n: u32) {
    for i in 1..=n {
        fizzbuzz(i);
    }
}
