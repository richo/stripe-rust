use std::string::String;
use std::io;
use std::io::Read;
use customer::{CustomerList,CustomerId};
use card::CardList;
use decoder::{StripeDecoder,StripeDecoderError};
use url::Url;
use hyper::client::Request;
use hyper::net::Fresh;
use hyper::Get;
use hyper::header::Authorization;
use hyper::error::HttpError;
use rustc_serialize::Decodable;

type SecretKey = String;

pub struct Connection {
    base_url: Url,
    secret_key: SecretKey
}

macro_rules! urlify {
    ($($component:expr),*) => {
        vec![$($component.to_string()),*]
    }
}

pub enum StripeError {
    TransportError(HttpError),
    DecodingError(StripeDecoderError),
    IOError(io::Error),
}

impl Connection {
    pub fn new(secret_key: String) -> Connection {
        return Connection {
            base_url: Url::parse("https://api.stripe.com").unwrap(),
            secret_key: secret_key
        };
    }

    fn request(&self, mut path: Vec<String>) -> Request<Fresh> {
        let mut url = self.base_url.clone();
        url.path_mut().unwrap().append(&mut path);
        let mut request = Request::new(Get, url).unwrap();
        let mut auth = "Bearer ".to_string();
        auth.push_str(&self.secret_key[..]);
        request.headers_mut().set(Authorization(auth));

        return request;
    }

    fn fetch<T: Decodable>(req: Request<Fresh>) -> Result<T, StripeError> {
        let connection = match req.start() {
            Ok(o) => o,
            Err(e) => return Err(StripeError::TransportError(e)),
        };
        let mut response = match connection.send() {
            Ok(o) => o,
            Err(e) => return Err(StripeError::TransportError(e)),
        };

        let mut body = vec![];
        match response.read_to_end(&mut body) {
            Ok(_) => {},
            Err(e) => return Err(StripeError::IOError(e)),
        }

        let object: T = match StripeDecoder::decode(body) {
            Ok(o) => o,
            Err(e) => return Err(StripeError::DecodingError(e)),
        };

        return Ok(object);
    }

    pub fn customers(&self) -> Result<CustomerList, StripeError> {
        let req = self.request(urlify!("v1", "customers"));
        return Connection::fetch(req);
    }

    pub fn cards(&self, customer: CustomerId) -> Result<CardList, StripeError> {
        let req = self.request(urlify!("v1", "customers", customer, "cards"));
        return Connection::fetch(req);
    }

}
