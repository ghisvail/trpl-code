use std::env;

fn main() {
    let mut argv = env::args();
    let filename: String = argv.nth(1).unwrap();
    match filename.find('.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &filename[i+1..]),
    }
}
