fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");
    {
        let x = 7;
        println!("{x} in the scope");
    }
    println!("{x} out of scope");

    const DAY_TO_COMPLETE_RUST: u8 = 7;
    println!("{DAY_TO_COMPLETE_RUST} will be required to complete the Rust Tutorial!");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Number of spaces in '   ' is {spaces}");
}