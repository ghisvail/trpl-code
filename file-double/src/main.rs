use std::fs::File;
use std::io::Read;
use std::path::Path;

use std::io;
use std::num;

// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `CliError` values.
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path).map_err(CliError::Io));
    let mut content = String::new();
    try!(file.read_to_string(&mut content).map_err(CliError::Io));
    let n = try!(content.trim().parse::<i32>().map_err(CliError::Parse));
    Ok(2 * n)
}

fn main() {
    match file_double("double.me") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
