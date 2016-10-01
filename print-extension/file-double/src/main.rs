use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap();  // error 1
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();     // error 2
    let n: i32 = content.trim().parse().unwrap();     // error 3
    2 * n
}

fn main() {
    let doubled = file_double("double.me");
    println!("{}", doubled);
}
