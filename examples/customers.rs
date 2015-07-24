#![feature(slice_patterns)]
extern crate stripe;
use stripe::connection::Connection;

use std::env;

fn usage() {
    let args: Vec<_> = env::args().collect();
    println!("Usage: {} [customer_id]", args[0]);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    match &args[..] {
        [] => unreachable!(),
        [_] => fetch_and_print_customers(),
        [_, ref cus] => fetch_and_print_customer(cus),
        _ => {
            usage();
            return;
        },
    };
}

fn get_conn() -> Connection {
    let secret_key: String = env::var("STRIPE_SECRET_KEY").ok().expect("No STRIPE_SECRET_KEY set");
    return Connection::new(secret_key);
}

fn fetch_and_print_customers() {
    let conn = get_conn();
    for i in conn.customers().into_iter() {
        println!("{:?}", i);
    }
}

fn fetch_and_print_customer(id: &String) {
    panic!("Unimplemented!");
}
