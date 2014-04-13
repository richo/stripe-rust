#![crate_id = "stripe#0.0-pre"]

#![comment = "Stripe API Bindings"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

extern crate http;
extern crate url;
extern crate serialize;
extern crate collections;

pub mod connection;
pub mod customer;
pub mod subscription;
pub mod card;
mod decoder;
