use rand::{self, Rng};
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..10); // Generate a random number

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let mut guess_message: &str = "incorrect";

    if secret_number == guess.trim().parse::<u32>().unwrap() {
        guess_message = "correct"
    }

    println!(
        "You guess: {} was {}. Right value was {}",
        guess, guess_message, secret_number
    );
}
