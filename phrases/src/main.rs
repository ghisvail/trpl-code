extern crate phrases;

fn main() {
    println!("Hello in English is {}", phrases::english::greetings::hello());
    println!("Goodbye in English is {}", phrases::english::farewells::goodbye());
    println!("Hello in Japanese is {}", phrases::japanese::greetings::hello());
    println!("Goodbye in Japanese is {}", phrases::japanese::farewells::goodbye());
}
