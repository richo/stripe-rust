#![feature(slice_patterns)]
extern crate stripe;
use stripe::connection::Connection;
use stripe::customer::Customer;

use std::env;

fn usage() {
    let args: Vec<_> = env::args().collect();
    println!("Usage: {} tok", args[0]);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    match &args[..] {
        [_, ref tok] => create_customer(tok),
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

fn create_customer(token: &String) {
    let cus = Customer::create(get_conn(), token.clone());

    println!("{:?}", cus);
}
