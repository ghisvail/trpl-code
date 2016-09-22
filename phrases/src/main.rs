extern crate phrases as sayings;

use sayings::english::{greetings,farewells};
use sayings::japanese;

fn main() {
    println!("Hello in English is {}", greetings::hello());
    println!("Goodbye in English is {}", farewells::goodbye());
    println!("Hello in Japanese is {}", japanese::hello());
    println!("Goodbye in Japanese is {}", japanese::goodbye());
}
