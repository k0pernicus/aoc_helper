use std::{fs::File, str::FromStr};
use std::io::prelude::*;
use std::io::{self, BufReader};

/// Read and return the content of an input file.
/// The file is given as function parameter.
/// This function returns a Result type: Vec<T> which contains handled line content, or an io::Error.
/// The type must derive `std::str::FromStr` trait.
pub fn get_content<T>(filepath: &str) -> io::Result<Vec<T>>
    where T: FromStr
{
    let f = File::open(filepath)?;
    let f = BufReader::new(f);
    let mut v: Vec<T> = Vec::new();
    for (i, line) in f.lines().enumerate() {
        match line {
            Ok(raw_line) => {
                if let Ok(line_content) = raw_line.parse::<T>() {
                    v.push(line_content);
                }
            }
            Err(err) => println!("Got error when reading line {}: {:?}", i, err),
        }
    }
    Ok(v)
}
