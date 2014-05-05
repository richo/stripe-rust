#![crate_id = "cli"]

extern crate stripe;
extern crate http;
use stripe::connection::Connection;
use stripe::customer::CustomerId;
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

fn fix_args_types(args: ~[~str]) -> ~[&str] {
    args.move_iter().map |s| {
        &s
    }
}


fn main() {
    let CUSTOMERS: ~str = "customers".to_owned();
    let CARDS    : ~str = "cards".to_owned();

    let conn = get_conn();
    match os::args().as_slice() {
        [argv0, "customers"] => print_customers(conn),
        [argv0, "cards", customer] => print_cards(conn, customer),
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
    Connection::new(secretKey)
}

fn print_customers(conn: Connection) {
    print_records(conn.customers().data.iter());
}

fn print_cards(conn: Connection, customer: CustomerId) {
    print_records(conn.cards(customer).data.iter());
}
