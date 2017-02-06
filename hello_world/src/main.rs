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
    println!("My name is {0}, {1} {0}.", "Bond");

    // Create a structure wehich contains an i32
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types like this structure require more complicated handling
    // This will not work!
    // println!("This struct '{}' won't print...", Structure(3));

}
