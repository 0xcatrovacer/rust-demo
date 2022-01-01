// Generate a Secret Number that the user tries to guess

// Add rand crate to Cargo.toml

use std::io;
use rand::Rng;
/*
The Rng trait defines methods that random number generators implement
*/
use std::cmp::Ordering;
/*
Ordering is another enum, but the variants for Ordering are Less, Greater, and Equal. These are the three outcomes that are possible when two values are compared.
*/


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..11);
    /*
    The rand::thread_rng function will give the number generator that is seeded by the operating system.

    The gen_range method takes a range expression as an argument and generates a random number in the range.
    */

    let mut guess = String::new();
    
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    /*
    Rust allows us to shadow the previous value of guess with a new one since string guess and u32 secret_number will return an error.

    guess is bound to the expression guess.trim().parse()
    [trim method on a String instance will eliminate any whitespace at the beginning and end]
    */

    println!("Secret Number is {} and you guessed {}", secret_number, guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("smaller!"),
        Ordering::Greater => println!("greater!"),
        Ordering::Equal => println!("Perfect!")
    }
}