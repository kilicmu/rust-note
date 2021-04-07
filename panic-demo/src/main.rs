use std::{fs::File, io::ErrorKind};
use std::io;
use std::fs;
fn main() {
    match File::open("test.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("create file error: {:?}", e),
            },
            other_error => panic!("occer known error when open file: {:?}", other_error),
        },
    };
}

fn panic_fn() -> Result<String, io::Error> {
    // let mut s = String::new();
    // File::open("text.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string("text.txt")
}