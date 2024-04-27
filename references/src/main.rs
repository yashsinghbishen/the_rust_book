fn main() {
    let mut s1 = String::from("Yogesh");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    println!("Before sending mutable reference to a function: '{s1}'");
    add_surname(&mut s1);
    println!("After sending mutable reference to a function: '{s1}'");

    multiple_borrowing();

}
// Borrowing (reference)
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Mutable Borrowing (reference)
fn add_surname(s: &mut String){
    s.push_str(" Singh");
}

fn multiple_borrowing(){
    let mut name = String::from("Yogesh Singh");

    let r1 = &mut name;
    // let r2 = &mut name; // This will fail as the r1 is still in scope after this statement
    println!("{r1}");
    
    let r2 = &mut name; // This works as r1 is out of scope. If we try to use r1 after this statment,
                                     // It will revert this statment for r2
    println!("{r2}");
    // println!("{r1}"); // This will fail the assignment for r2.
    

    // This works as multipe immutable references are allowed.
    let r3 = &name; // no problem
    let r4 = &name; // no problem
    let r5 = &name; // no problem

    // let r6 = &mut name; // This won't work as only single mutable reference is allowed in the scope, 
                                    // Even immutable refrence does not allow to have mutable references.

    println!("{}, {}, and {}", r3, r4, r5);

    let r6 = &mut name; //This works as no other reference is in the current scope
    println!("{r6}");
    // let reference_to_nothing = dangle();

    let r7 = no_dangle();
    print!("{r7}");
}


// This will return error as we're trying to pass the reference of s which has scope only in this method.
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}