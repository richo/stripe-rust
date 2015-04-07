use std::str;
use std::vec::Vec;
use serialize::{json,Decodable};
use std::marker::PhantomData;

pub struct StripeDecoder<T>(T);

impl<T: Decodable> StripeDecoder<T> {
    // TODO this should be a Result
    pub fn decode(data: Vec<u8>) -> T {
        let data = str::from_utf8(data.as_slice());

        let object = json::from_str(data.unwrap());
        let mut decoder = json::Decoder::new(object.unwrap());

        let decoded: T = match Decodable::decode(&mut decoder) {
            Ok(v) => v,
            Err(e) => panic!("Decoding error: {}", e)
        };

        return decoded;
    }
}
