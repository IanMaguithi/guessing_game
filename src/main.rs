// input/output library
use std::io;
// Guessing game
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Create a mutable variable that is currently bound to a new, empty instance of a String
    let mut guess = String::new();

    // Read input from user
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
