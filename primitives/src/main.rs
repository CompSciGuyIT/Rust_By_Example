fn main() {
    // Variables can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0;     // Regular annotation
    let an_integer = 5i32;      // Suffix annotation

    // Or a default will be used
    let default_float = 3.0;    // f64
    let default_integer = 7;    // i32

    let mut mutable = 12;       // mutable i32

    // Error! The type of the variable cannot be changed
    // mutable = true;

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Warning: This expression will panic at run-time
    // println!("1 - 2 = {}", 1u32 - 2);

    // Short circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("not true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 1_000_000u32);

    // Use underscores to improve readability
    println!("One million is written as {}", 1_000_000u32);
}
