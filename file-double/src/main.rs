use std::fs::File;
use std::io::Read;
use std::path::Path;

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|err| err.to_string())
                .map(|_| content)
        })
        .and_then(|content| {
            content.trim().parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| 2 * n)
}

fn main() {
    match file_double("double.me") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
