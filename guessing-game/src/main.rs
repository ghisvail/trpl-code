extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    // Draw a number from 1 to 100.
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret);

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed {}", guess);
}
