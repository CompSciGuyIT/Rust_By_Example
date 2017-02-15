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
}
