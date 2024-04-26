fn main() {
    tuple_example();
    array_example();
}

fn tuple_example() {
    println!("\n\nTuple example");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("Accessing tuple by destrcturing:");
    let (x, y, _) = tup;
    println!("First destrctured value in x {x}");
    println!("Second destrctured value in y {y}");

    println!("\nAccessing tuple values by index");
    println!("First value in tuple by index {}", tup.0);
    println!("Second value in tuple by index {}", tup.1);
    println!("Third value in tuple by index {}", tup.2);
}

fn array_example(){
    println!("\n\nArray example");

    let a = [1, 2, 3, 4, 5];
    println!("Printing the array:\n{:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("\nPrining the pretty array:\n{:#?}", months);

    println!("First value of a by index {}", a[0]);
    

    
}