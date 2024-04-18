use rand::Rng;
use std::{cmp::Ordering, io};
use colored::*;
fn main() {
    println!("Welcome to Yogesh's first gessing game created in Rust. Guess the random number between 1 to 5");

    let random_number = rand::thread_rng().gen_range(1..5);

    loop {
        println!("Please enter a number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to accept the input");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        println!("You entered {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("{}","Guessed too low number".red()),
            Ordering::Greater => println!("{}","Gussed too high number".red()),
            Ordering::Equal => {
                println!("{}","You Won!".green());
                break;
            },
        }
    }
}
