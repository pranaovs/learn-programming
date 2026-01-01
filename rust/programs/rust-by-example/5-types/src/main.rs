fn main() {
    let decimal = 65.4321_f32; // 32 bit floating

    let integer = decimal as u8; // no implicit conversion

    let ch = integer as char;

    println!("ch: {:?}", ch);

    // Inferences
    let mut vec = Vec::new();

    vec.push(5u8); // Type of vec inferred to be Vec<u8>

    // Aliasing (new name for an existing type)

    type NanoSecond = u64;
    type Inch = u64;

    let time: NanoSecond = 5;
    let length: Inch = 10;

    // They do not provide extra type safety
}
