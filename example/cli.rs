#![crate_id = "cli"]

extern crate stripe;
extern crate http;
use stripe::connection::Connection;
use http::method::Get;

use std::os;
use std::fmt::Show;

fn usage() {
    let args = os::args();
    println!("Usage: {} <record type>", args[0]);
    println!("  Record types:");
    println!("  - customers");
    println!("  - cards");
}

fn main() {
    format!("{}", Get);
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        2 => fetch_and_print_records(args[1]),
        _ => {
            usage();
            return;
        },
    };
}

fn print_records<T: Show, I: Iterator<T>>(mut iter: I) {
    for i in iter {
        println!("{}", i);
    }
}

fn fetch_and_print_records(typ: ~str) {
    let secretKey: ~str = os::getenv("STRIPE_SECRET_KEY").expect("No STRIPE_SECRET_KEY set");
    let conn = Connection::new(secretKey);

    if typ == ~"customers" {
        print_records(conn.customers().data.iter());
    } else if typ == ~"cards" {
        print_records(conn.cards().data.iter());
    } else {
        usage();
        return;
    }
}
