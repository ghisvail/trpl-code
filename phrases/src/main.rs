extern crate phrases;

use phrases::english::{greetings,farewells};

fn main() {
    println!("Hello in English is {}", greetings::hello());
    println!("Goodbye in English is {}", farewells::goodbye());
    println!("Hello in Japanese is {}", phrases::japanese::greetings::hello());
    println!("Goodbye in Japanese is {}", phrases::japanese::farewells::goodbye());
}
