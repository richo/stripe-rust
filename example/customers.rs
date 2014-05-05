#![crate_id = "customers"]

extern crate stripe;
extern crate http;
use stripe::connection::Connection;
use http::method::Get;

use std::os;
use std::fmt::Show;

fn usage() {
    let args = os::args();
    println!("Usage: {} [customer_id]", args[0]);
}

fn main() {
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        1 => fetch_and_print_customers(),
        2 => fetch_and_print_customer(args[1]),
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

fn get_conn() -> Connection {
    let secretKey: ~str = os::getenv("STRIPE_SECRET_KEY").expect("No STRIPE_SECRET_KEY set");
    return Connection::new(secretKey);
}

fn fetch_and_print_customers() {
    let conn = get_conn();
    print_records(conn.customers().data.iter());
}

fn fetch_and_print_customer(id: ~str) {
    println!("Unimplemented!");
    os::set_exit_status(1);
}
