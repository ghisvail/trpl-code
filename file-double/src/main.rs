use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = try!(File::open(file_path).map_err(|e| e.to_string()));
    let mut content = String::new();
    try!(file.read_to_string(&mut content).map_err(|e| e.to_string()));
    let n = try!(content.trim().parse::<i32>().map_err(|e| e.to_string()));
    Ok(2 * n)
}

fn main() {
    match file_double("double.me") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
