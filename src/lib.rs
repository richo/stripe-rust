#![feature(collections)]
#![feature(convert)]
#![feature(append)]

extern crate hyper;
extern crate rustc_serialize;
extern crate url;
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
            pub fn into_iter(self) -> ::collections::vec::IntoIter<$name> {
                self.data.into_iter()
            }
        }
    )
}

macro_rules! creatable {
    ($entity:ident, $request:ident, $path:expr,
     ($($field:ident => $ty:ty),*)) => {
        #[derive(RustcEncodable)]
        pub struct $request {
            $(
                $field: Option<$ty>,
                )*
        }

        impl UrlEncodable for $request {
            fn into_iter(self) -> Vec<(&'static str, String)> {
                let mut tmp = vec![];

                let $(($field,) = (self.$field,))*;
                $(
                    if let Some(v) = $field {
                        tmp.push((stringify!($field), v));
                    }
                 )*
                    tmp
            }
        }
        impl Creatable for $entity {
            type Object = $request;

            fn path() -> &'static str {
                $path
            }
        }
    }
}


pub mod connection;
pub mod customer;
pub mod subscription;
pub mod card;
mod decoder;
mod util;
