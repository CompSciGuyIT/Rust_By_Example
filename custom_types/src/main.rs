// An attribute to hide warnings for unused code
#![allow(dead_code)]

use List::*;

// Globals are declared outside all other scopes
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

// Create an enum to classify someone.
// Note how both names and type information
// together specify the variant:
// 'Engineer != Scientist' and 'Height(i32) != Weight(i32)'.
// each is different and independent.
enum Person {
    // An enum may either be 'unit-like',
    Engineer,
    Scientist,
    // like tuple structs
    Height(i32),
    Weight(i32),
    // or like structures
    Info { name: String, height: i32 }
}

// A function which takes a Person enum
// as an argument and returns nothing
fn inspect(p: Person) {
    // Usage of an 'enum' must covre all cases (irrefutable)
    // so a 'match' is used to branch over it.
    match p {
        Person::Engineer    => println!("Is an Engineer!"),
        Person::Scientist   => println!("Is a Scientist!"),
        // Destructure 'i' from inside the 'enum'
        Person::Height(i)   => println!("Has a height of {}.", i),
        Person::Weight(i)   => println!("Has a weight of {}.", i),
        // Destructure 'Info' into 'name' and 'height'
        Person::Info { name, height }   => {
            println!("{} is {} tall.", name, height);
        },
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // 'Nil' has type 'List'
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // 'Cons' also has type 'List'
        Cons(elem, Box::new(self))
    }

    // Return the lenght of the list
    fn len(&self) -> u32 {
        // 'self' has to be matched, because the behaviour
        // of this method depends on the variant of 'self'
        // 'self' has type '&List', and '*self' has type 'List',
        // matching on a concrete type 'T' is preferred
        // over a match on a reference '&T'
        match *self {
            // Can't take ownership of the tail, because 'self' is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 'format!' is similar to 'print!', but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}


// A unit struct
struct _Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { p1: Point { x: ref x1, y: ref y1 } , p2: Point { x: ref x2, y: ref y2 } } = rect;
    ((x2 - x1) * (y2 - y1)).abs()
}

fn square(point: Point, length: f32) -> Rectangle {
    let bottom_left_x = point.x;
    let bottom_left_y = point.y;
    let top_right_x = bottom_left_x + length;
    let top_right_y = bottom_left_y + length;
    let rect = Rectangle {  p1: Point { x: bottom_left_x, y: bottom_left_y },
                            p2: Point { x: top_right_x, y: top_right_y },
    };
    rect
}

fn main() {
    /////////////////////////////////
    //  structures
    /////////////////////////////////
 
    println!("Structures\n");

    // Instantiate a point
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
    println!();

    // Destructure the point using a 'let' binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = _Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    println!();

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!();

    let my_rect = Rectangle {
        p1: Point { x: -1.0, y: 11.0 },
        p2: Point { x: 9.0, y: 1.0 },
    };

    println!("The area of the rectangle is: {} units squared", rect_area(my_rect));
    println!();

    let my_square = square( Point { x: 1.0, y: 1.0 }, 10.0);

    let Rectangle { p1: point1, p2: point2 } = my_square;

    println!("bottom left x: {}, bottom left y: {}", point1.x, point1.y);
    println!("top right x: {}, top right y: {}", point2.x, point2.y);

    ////////////////////////////////////////////
    // ENUMS
    ///////////////////////////////////////////

    println!("\nEnums\n");
    let person  = Person::Height(18);
    let amira   = Person::Weight(10);
    // 'to_owned()' creates an owned 'String' from a string slice.
    let dave    = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca = Person::Scientist;
    let rohan   = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);

    // Explicitly 'use' eash name so they are
    // available without manual scoping
    use Status::{ Poor, Rich };
    // Automatically 'use' each name inside 'Work'
    use Work::*;

    // Equivalent to 'Status::Poor'
    let status = Poor;
    // Equivalent to 'Work::Civilian'
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit 'use' above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor don't have money..."),    
    }

    match work {
        // Note again the lack of scoping
        Civilian => println!("Civilians work"),
        Soldier  => println!("Soldiers fight"),
    }

    // enums can be cast as integers
    println!("\nzero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06}", Color::Blue as i32);

    println!();

    // Create an empty linked list
    let mut list = List::new();

    // Append some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list gas length: {}", list.len());
    println!("{}", list.stringify());

    //////////////////////////////
    // Constants
    //////////////////////////////

    println!();
    
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a constant
    // THRESHOLD = 5;
}
