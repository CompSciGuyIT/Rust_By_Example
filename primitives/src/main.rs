// import the 'fmt' and 'mem' modules to make them available
use std::fmt::{self, Formatter, Display};
use std::mem;

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn transpose(matrix: (Matrix)) -> (Matrix) {
    let transposed = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
    transposed
}

// The following struct is for the activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

// This function borrows a slice
fn analyse_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

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

    println!();
    println!();

    // A tuple with a bunch of different types
    let long_tuple =    (1u8, 2u16, 3u32, 4u64,
                        -1i8, -2i16, -3i32, -4i64,
                        0.1f32, 0.2f64,
                        'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value is {}", long_tuple.0);
    println!("long tuple second value is {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required
    // to tell them apart from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    println!();
    
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    println!();
    println!();
    
    // Fixed size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialised to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at zero
    println!("the first element of the array: {}", xs[0]);
    println!("the second element of the array: {}", xs[1]);

    // 'len' returns the size of the array
    println!("the size of the array is {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyse_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of an array as a slice");
    analyse_slice(&ys[1..4]);

    // Out of bound indexing yeilds a panic
    // println!("{}", xs[5]);

}
