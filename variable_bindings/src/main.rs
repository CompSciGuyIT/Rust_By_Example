fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy 'an_integer' into 'copied_integer'
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings;
    // These warnings can be silenced by prefixing
    // the variable name with an underscore
    let _unused_variable = 3u32;

    //////////////////////
    // Mutability
    /////////////////////

    println!();

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error
    // _immutable_binding += 1;

    //////////////////////////
    // Scope and Shadowing
    //////////////////////////

    println!();
    
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}, outer long: {}", short_lived_binding, long_lived_binding);

        // This binding *shadows* the outer one
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // Error! 'short_lived_binding' does not exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // This binding also *shadows* the previous binding
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}