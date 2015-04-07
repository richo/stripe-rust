extern crate stripe;
use stripe::connection::Connection;

use std::env;

fn usage(args: &Vec<String>) {
    println!("Usage: {} customer_id", args[0]);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    match &args.len() {
        &0 => unreachable!(),
        &2 => fetch_and_print_cards(&args[1]),
        _ => {
            usage(&args);
            return;
        },
    };
}

fn get_conn() -> Connection {
    let secret_key = env::var("STRIPE_SECRET_KEY").ok().expect("No STRIPE_SECRET_KEY set");
    return Connection::new(secret_key);
}

fn fetch_and_print_cards(id: &String) {
    let conn = get_conn();
    for i in conn.cards(id.clone()).into_iter() {
        println!("{:?}", i);
    }
}
