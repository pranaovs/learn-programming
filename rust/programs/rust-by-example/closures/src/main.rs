fn main() {
    // Closures as input parameters

    let color = String::from("green");
    let print = || println!("`color`: {}", color);

    print();

    let _borrow = &color;
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    println!("`count` outside closure: {}", count);

    // This won't compile, because `inc` mutably borrows `count` on each call.
    // inc();
}
