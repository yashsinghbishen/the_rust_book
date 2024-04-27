fn main() {
    moving_example();
    ownweship_example();
}
fn moving_example() {
    let mut s1 = String::from("Yogesh");
    s1.push_str(" Singh");
    println!("Printing the value of S1: '{s1}'");

    let mut s2 = s1;
    /*
    In this example, even the S1 is not out of scope, still ti can't be used
    as it's value has been moved to s2. In later lines, we can see the value has
    been moved back to s1 and s2 can't be used to access the value.
    */
    // println!("{s1}"); This will fail as the value has been moved to s2
    println!("Printing the value of S2: '{s2}'");

    s1 = s2;
    println!("Printing the value of S1: '{s1}'");
    // println!("{s2}"); This will fail as the value has been moved back to s1

    // Now we're performed clone(Deep copy) both values are valid.
    s2 = s1.clone();
    println!("Printing the value of S1: '{s1}'");
    println!("Printing the value of S2: '{s2}'");
}

fn ownweship_example() {
    let s = String::from("ownership string");
    println!("Prining s before transferring the owenership: {s}");
    
    takes_ownership(s);
    // println!("Prining s after transferring the owenership: {s}"); // This will fail
    
    let mut s1 = gives_ownership();
    println!("Prining s1 after receving the owenership: {s1}");
    
    s1 = takes_and_gives_back(s1);
    println!("Prining s1 after transferring and receving the owenership: {s1}");

    let i = 10;
    println!("Prining scaler type before passing it to a function {i}");
    
    makes_copy(i);
    println!("Prining scaler type after passing it to a function {i}");
    
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} /* some_string goes out of scope as the ownership was taken to some_string but never returned.
  As some_sting goes out of scope the `drop` method is executed and some_string does not exists anymore.
  */

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("Printing in function {}", some_integer);
} // Even after going out of scope it won't affect the owner as scaler datatypes create copy not move.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // Some string is returned, the ownership is tranfer to function caller and some_string is dropped.
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // Ownership is returned back to function caller.
    a_string // a_string is returned and moves out to the calling function
}
