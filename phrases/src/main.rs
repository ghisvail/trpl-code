extern crate phrases as sayings;

use sayings::english::{greetings as en_greetings, farewells as en_farewells};
use sayings::japanese as jp;

fn main() {
    println!("Hello in English is {}", en_greetings::hello());
    println!("Goodbye in English is {}", en_farewells::goodbye());
    println!("Hello in Japanese is {}", jp::hello());
    println!("Goodbye in Japanese is {}", jp::goodbye());
}
