fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        }
    }

    // inclusive
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match *name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("{:?}", names);

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Cannot access names any more because into_iter takes ownership

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("{:?}", names);

    let number = 13;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("Prime"),
        13..19 => println!("A teen"),
        _ => println!("IDK Man"),
    }

    let triple = (0, -2, 3);
    match triple {
        (0, y, z) => println!("First is 0, y is {} and z is {}", y, z),
        (1, ..) => println!("First is 1"),
        (.., -2) => println!("Last is -2"),
        (3, .., 4) => println!("First is 3 and last is 4"),
        _ => println!("It doesn't matter what they are"),
    }

    let array = [1, 2, 3];
    match array {
        [1, second, third] => println!("First is 1, second is {} and third is {}", second, third),
        [0, ..] => println!("First is 0"),
        [first, .., last] => println!("First is {} and last is {}", first, last),
    }

    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, Saturation: {}, Lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, Magenta: {}, Yellow: {}, Key(Black): {}",
            c, m, y, k
        ),
    }
    // Dont need to use default arm because all variants are covered

    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }

    let temperature = Temperature::Celsius(35);
    // ^ TODO try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part is a guard
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }

    // Binding
    match age() {
        n @ 0..=12 => println!("Child of age {}", n),
        n @ 13..=19 => println!("Teenager of age {}", n),
        n @ 20..=64 => println!("Adult of age {}", n),
        n => println!("Senior of age {}", n),
    }
    // Use @ to bind the matched value to a variable

    // if let can be used to bind without a full match. Used for destructuring enums often
    if let n @ 1..=12 = age() {
        println!("Child of age {}", n);
    } else if let n @ 13..=19 = age() {
        println!("Teenager of age {}", n);
    } else if let n @ 20..=64 = age() {
        println!("Adult of age {}", n);
    } else {
        let n = age();
        println!("Senior of age {}", n);
    }
}

fn age() -> u32 {
    15
}
