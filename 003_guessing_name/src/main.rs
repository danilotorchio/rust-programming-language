use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a random number, current thread, seed by the os
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        // Create a new mutable string variable
        let mut guess: String = String::new();

        // Read a input from the user, panic if fails
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Parse the string to a number, continue to next loop interaction if fails
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
