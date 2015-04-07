#![crate_id = "stripe"]

extern crate hyper;
extern crate rustc_serialize;
extern crate url;
extern crate serialize;
extern crate collections;

macro_rules! iterable{
    ($name:ident) => (
        impl IntoIterator for $name {
            type Item = $name;
            fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
                self.data().into_iter()
            }
        }
        )
}

pub mod connection;
pub mod customer;
pub mod subscription;
pub mod card;
mod decoder;
