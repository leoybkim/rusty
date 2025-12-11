fn main() {
    // Testing mutability of a variable
    println!("Testing mutability of a variable");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}\n\n");

    // Declaring constants with data type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of constant THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}\n\n");

    // Shawdowing
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}\n\n");

    println!("Demonstrating ability to mutate the shadow variable's type");
    let spaces = "  ";
    println!("The value of spaces: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces: {spaces}");

    let mut mut_spaces = "  ";
    println!("The value of mut_spaces: {mut_spaces}");
    // let mut_spaces = mut_spaces.len(); // Expected to throw error
}
