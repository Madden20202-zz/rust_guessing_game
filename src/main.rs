// How to import a package in a file 
use std::io;
use rand::Rng;
use std::cmp::Ordering;

//fn stands for function
//main() is the entry point for your program
fn main() {

    println!("Guess the Number!");

    println!("Please input a Guess.");

    loop {
        // mut is short for mutable, 
        //allowing the value to be any string value
        //all variables are immutable by default
        let mut guess = String::new();

        io::stdin()
            //reads input from the user, with mutable value
            .read_line(&mut guess)

            //this line always has to be added
            //since this allows errors to be caught
            //failing to add this line will cause a warning to appear
            .expect("Failed to read line.");

        println!("Did you guess: {}", guess);

        // Secret Number generation
        // creates a number between 1 and 100
        // note that the second number is exclusive and not used
        let secret_number = rand::thread_rng().gen_range(1..101);

        println!("The Secret Number is {}", secret_number);

        // Comparing the two variables

        //allows the guess string to be compared to the integer 
        let guess: u32 = guess.trim().parse().expect("Please Type a Number.");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => println!("You got It!"), 
    }
    
    }
}
