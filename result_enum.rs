/*
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }
*/
use std::fs::read_to_string;

fn main() {
    let result = read_to_string("cargo.toml");

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("Error while reading the file, {}", err),
    }
}
