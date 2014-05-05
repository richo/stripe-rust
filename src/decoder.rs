use std::str;
use std::vec::Vec;
use serialize::{json,Decodable};

pub struct Decoder<T>;

impl<T: Decodable<json::Decoder,json::DecoderError>> Decoder<T> {
    pub fn decode(data: Vec<u8>) -> T {
        let data = str::from_utf8(data.as_slice());

        let object = json::from_str(data.unwrap());
        let mut decoder = json::Decoder::new(object.unwrap());

        let decoded: T = match Decodable::decode(&mut decoder) {
            Ok(v) => v,
            Err(e) => fail!("Decoding error: {}", e)
        };

        return decoded;
    }
}
