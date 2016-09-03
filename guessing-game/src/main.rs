extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    // Draw a number from 1 to 100.
    let secret = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is {}", secret);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
