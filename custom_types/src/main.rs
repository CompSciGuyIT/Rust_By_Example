// A unit struct
struct Nil;

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
    let _nil = Nil;

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
}
