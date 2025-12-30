use std::fmt;

struct Unit;

struct Pair(i32, i32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("joe");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point { x: 5.2, y: 0.4 };

    println!("Coordinates: ({}, {})", point.x, point.y);

    let rect = Rectangle {
        top_left: Point { x: 4., y: 3. },
        bottom_right: Point { x: 8., y: 9. },
    };
}
