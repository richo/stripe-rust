#![crate_id = "client"]

extern crate stripe;
extern crate http;
use stripe::connection::Connection;
use http::method::Get;

use std::os;

fn usage() {
    let args = os::args();
    println!("Usage: {} <record type>", args[0]);
    println!("  Record types:");
    println!("  - customers");
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

fn fetch_and_print_records(typ: ~str) {
    let secretKey: ~str = os::getenv("STRIPE_SECRET_KEY").expect("No STRIPE_SECRET_KEY set");
    let conn = Connection::new(secretKey);
    let mut data;

    if typ == ~"customers" {
        data = conn.customers()
    } else {
        usage();
        return;
    }

    for item in data.iter() {
        println!("{}", item);
    }
}
