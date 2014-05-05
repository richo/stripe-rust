#![crate_id = "charge"]

extern crate stripe;
extern crate http;

use std::io;

fn input(prompt: &'static str) -> ~str {
    let mut output = io::stdout();
    let mut reader = io::stdin();

    output.write(prompt.as_bytes());
    output.flush();

    match reader.read_line() {
        Ok(line) => line,
        Err(err) => fail!("Error: {}", err)
    }
}

fn main() {
    let cc_no = input("CC no: ");
}
