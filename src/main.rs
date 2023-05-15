// input/output library
use std::io;
// rand library
use rand::Rng;

// Guessing game
fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Create a mutable variable that is currently bound to a new, empty instance of a String
        let mut guess = String::new();

        // Read input from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);
        println!("The secret number is: {}", secret_number);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => println!("You win!"),
        }
    }
}
