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
    println!("Now {:?} will print!", Deep(DebugPrintable(7)));
}
