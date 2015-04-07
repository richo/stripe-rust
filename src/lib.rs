#![crate_id = "stripe"]

extern crate hyper;
extern crate rustc_serialize;
extern crate url;
extern crate serialize;
extern crate collections;

macro_rules! iterable {
    ($collection:ident, $name:ident) => (

    //     impl<'a> IntoIterator for &'a $collection {
    //         type Item = $name;
    //         type IntoIter = ::collections::slice::Iter<'a, $name>;

    //         fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
    //             self.data.into_iter()
    //         }
    //     }
    //     )

    // This is a stopgap until I work out how2IntoIterator
        impl $collection {
            fn into_iter(self) -> ::collections::vec::IntoIter<$name> {
                self.data.into_iter()
            }
        }
    )
}

pub mod connection;
pub mod customer;
pub mod subscription;
pub mod card;
mod decoder;
