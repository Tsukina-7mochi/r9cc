extern crate r9cc;

use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let result = r9cc::compile(&buffer);
    match result {
        Ok(result) => println!("{}", result),
        Err(err) => print!("{}", err),
    }
}
