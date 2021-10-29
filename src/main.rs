// How to import a package in a file 
use std::io;

fn main() {
    println!("Guess the Number!");

    println!("Please input a Guess.");

    // mut is short for mutable, 
    //allowing the value to be any string value
    let mut guess = String::new();

    io::stdin()
        //reads input from the user, with mutable value
        .read_line(&mut guess)

        //this line always has to be added
        //since this allows errors to be caught
        //failing to add this line will cause a warning to appear
        .expect("Failed to read line.");

    println!("Did you guess: {}", guess);
}
