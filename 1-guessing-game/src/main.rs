use std::io; //Library

fn main() {  //Main Function
    println!("Guess the number"); // Print a string to the screen

    println!("Please input your number");

    let mut guess = String::new();  
    /*
    In Rust, variables are IMMUTABLE by default. 'mut' must be added before a variable to make it mutable.

    String::new is a function which returns a new instance of a String. 
    */

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    /*
    The 'stdin' function from the 'io' module is being called.

    .read_line(&mut guess) calls the 'read_line' method on the standard input handle to get input from the user. 'read_line' takes whatever the user types into standard input and appends that into a string without overwriting its contents.
    [& => call by reference]

    read_line returns a value io::Result. If this instance of io::Result is an Err value, 'expect' will cause the program to crash and display the message that is passed as an argument to 'expect'.
    */

    println!("You guessed: {}", guess);
}