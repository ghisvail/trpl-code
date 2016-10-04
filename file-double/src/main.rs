use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let mut content = String::new();
    if let Err(err) = file.read_to_string(&mut content) {
        return Err(err.to_string());
    };
    let n: i32 = match content.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2 * n)
}

fn main() {
    match file_double("double.me") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
