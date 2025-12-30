fn main() {
    let integer = 1;

    println!("integer: {:?}", integer); // integer: 1
    // Variable shadowing
    let integer: i64 = 1;
    println!("value:: {}", integer); // value:: 1
    let _integer = 1u32; // silenec warning

    let mut mutable_integer = 1;
    mutable_integer += 1;
    println!("mutable integer: {:?}", mutable_integer); // mutable integer: 2

    // Scopes
    {
        let mutable_integer = 3;
        println!("shadowed mutable integer: {:?}", mutable_integer); // shadowed mutable integer: 3
    }
    println!("outer mutable integer: {:?}", mutable_integer); // outer mutable integer: 2

    let declared_later;
    {
        declared_later = 5;
        println!("declared later: {:?}", declared_later); // declared later: 5
    }

    // Valid usage of declared_later
    println!("declared later outside block: {:?}", declared_later); // declared later outside

    let mut mutable_var = 10;
    {
        // variable frozen in scope
        let mutable_var = mutable_var + 5; // mutable var shadowed, is not unmutable here
        println!("inner mutable_var: {:?}", mutable_var); // inner mutable_var: 15
    }
    mutable_var += 0; // valid since outer mutable_var is still mutable
    println!("outer mutable_var: {:?}", mutable_var); // outer mutable_var: 10
}
