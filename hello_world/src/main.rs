// import the 'fmt' module to make it available
use std::fmt::{self, Formatter, Display};

// Define a structure which 'fmt::Display' will be implemented for
// A tuple struct containing an 'i32' bound to the name 'Structure'
struct Structure(i32);

// In order to use the '{}' marker, the trait 'fmt::Display'
// must be implemented manually for this type
impl fmt::Display for Structure {
    // This trait requires 'fmt' with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output stream: 'f'.
        // Returns 'fmt::Result' which indicates whether operation succeeded or failed.
        // Note that 'write!' uses syntax which is very similar to 'println!'
        write!(f, "{}", self.0)  
    }
}

// A structure holding two numbers
// 'Debug' will be derived so the results can be contrasted with 'Display'
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement 'Display' for 'MinMax'
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use 'self.number' to refer to each positional data point
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are namable for comparison
#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

// Implement 'Display' for 'Point2'
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customise so only 'x' and 'y' are denoted
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Struct for the Activity
#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

// Implement 'Display' for 'Complex'
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;
        try!(write!(f, "["));
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { try!(write!(f, ", "));}
            try!(write!(f, "{}: {}", count, v));
        }
        write!(f, "]")
    }
}

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // 'write!' is like 'format!', but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

//#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
                self.red, self.green, self.blue, self.red, self.green, self.blue)
    }
}

fn main() {

    // Line comments which go to the end of the line

    /*
        Block comments which go to the
        closing delimiter
    */

    /*
        /// Generate library docs for the following item
        //! Generate library docs for the enclosing item
    */
    
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    // {} will be replaced with any arguments
    println!("{} days", 31);

    // Positional Arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Bob", "Joe");

    // Named arguments can be used
    println!("{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over");

    // Special formatting can be specified after a ':'
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // You can right align text with a specified width
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeros
    println!("{number:0>width$}", number = 1, width = 6);

    // It will check that you've used the correct number of arguments
    println!("My name is {0}, {1} {0}.", "Bond", "James");

    // Create a structure wehich contains an i32
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types like this structure require more complicated handling
    // This will not work!
    // println!("This struct '{}' won't print...", Structure(3));

    let pi = 3.141592;
    println!("Pi is equal to {:.*}", 3, pi);

    // This structure cannot be printed either with
    // 'fmt::Display' or 'fmt::debug'
    struct UnPrintable(i32);
    
    // the 'derive' attribute automatically creates the implementation
    // required to make this 'struct' printable with 'fmt::Debug'
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is in {tvShow:?}.",
             "Slater",
             "Christian",
             tvShow = "Mr. Robot");
    
    // The structure is now printable
    println!("Now {:?} will print!", DebugPrintable(3));

    // The problem with derive is there is no control
    // over how the results will look.
    println!("Now {:?} will print!\n", Deep(DebugPrintable(7)));

    // The code for the Display section
    let minmax = MinMax(0, 14);

    println!("Compare Structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =     MinMax(-300, 300);
    let small_range =   MinMax(-3, 3);

    println!("\nThe big range is {big} and the small range is {small}\n",
            small = small_range,
            big = big_range);
    
    let point = Point2 { x: 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error.  Both 'Debug' and 'Display' were implemented,
    // but '{:b}' requires 'fmt::Binary' to be implemented.
    // This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);

    /*
     *      Activity
     */

    println!("\n");
    
    let complex = Complex { real: 3.3, imag: 7.2 };
    
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    println!("\n");

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    println!();

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color)
    }
}
