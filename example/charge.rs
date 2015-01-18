#![crate_id = "charge"]

extern crate stripe;
extern crate http;

use std::io;
use stripe::card::CardNumber;

fn input(prompt: &'static str) -> ~str {
    let mut output = io::stdout();
    let mut reader = io::stdin();

    output.write(prompt.as_bytes());
    output.flush();

    match reader.read_line() {
        Ok(line) => line,
        Err(err) => panic!("Error: {}", err)
    }
}

fn main() {
    let cc_no = input("CC no: ");
    let cc_no = cc_no.trim();

    match CardNumber::new(cc_no) {
        Some(cc) => println!("Got: {}", cc),
        None => println!("ohnoes")
    }
}
