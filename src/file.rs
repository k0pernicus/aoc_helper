use std::path::Path;
use std::{fs::File, str::FromStr};
use std::io::{self, BufReader, prelude::*};

/// Read and return the content of a file.
/// This function returns a Result type: Vec<T> which contains handled line content, or an io::Error.
pub fn content<P, T>(path: P) -> io::Result<Vec<T>>
    where T: FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug, P: AsRef<Path>
{
    let file: File = File::open(path).expect("no such file");
    let buf: BufReader<File> = BufReader::new(file);
    let lines = 
        buf.lines().enumerate()
        .map(|(index, line)| 
            line.expect(&format!("Could not read line at index {}", index))
                .parse::<T>().expect(&format!("Could not parse line at index {}", index))
        ).collect();
    Ok(lines)
}
