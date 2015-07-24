use std::str;
use std::vec::Vec;
use rustc_serialize::Decodable;
use rustc_serialize::json;

pub struct StripeDecoder<T>(T);

pub type StripeDecoderError = json::DecoderError;

impl<T: Decodable> StripeDecoder<T> {
    pub fn decode(data: Vec<u8>) -> Result<T, StripeDecoderError> {
        let data = str::from_utf8(data.as_slice());

        json::decode(data.unwrap())
    }
}
