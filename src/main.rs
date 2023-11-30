extern crate r9cc;

use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("{}", r9cc::compile(buffer));
}
