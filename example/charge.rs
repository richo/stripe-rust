extern crate stripe;

use std::io;
use std::io::Write;
use stripe::card::CardNumber;

fn input(prompt: &'static str) -> String {
    let mut output = io::stdout();
    let mut reader = io::stdin();

    output.write(prompt.as_bytes());
    output.flush();

    let mut line =  String::new();
    match reader.read_line(&mut line) {
        Ok(_) => line,
        Err(err) => panic!("Error: {}", err)
    }
}

fn main() {
    let cc_no = input("CC no: ");
    let cc_no = cc_no.trim();

    match CardNumber::new(cc_no) {
        Some(cc) => println!("Got: {:?}", cc),
        None => println!("ohnoes")
    }
}
