use std::str;
use std::vec::Vec;
use rustc_serialize::Decodable;
use rustc_serialize::json;

pub struct StripeDecoder<T>(T);

impl<T: Decodable> StripeDecoder<T> {
    // TODO this should be a Result
    pub fn decode(data: Vec<u8>) -> T {
        let data = str::from_utf8(data.as_slice());

        let decoded: T = match json::decode(data.unwrap()) {
            Ok(v) => v,
            Err(e) => panic!("Decoding error: {}", e)
        };

        return decoded;
    }
}
