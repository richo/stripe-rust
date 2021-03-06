use std::string::String;
use std::io;
use std::io::{Read,Write};
use customer::{CustomerList,CustomerId};
use util::{Creatable,UrlEncodable};
use card::CardList;
use decoder::{StripeDecoder,StripeDecoderError};
use url::{Url,form_urlencoded};
use hyper::client::Request;
use hyper::net::{Fresh,Streaming};
use hyper::method::Method;
use hyper::{Get, Post};
use hyper::header::Authorization;
use hyper::error::Error as HttpError;
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

#[derive(Debug)]
pub enum StripeError {
    TransportError(HttpError),
    DecodingError(StripeDecoderError),
    IOError(io::Error),
}

impl From<StripeDecoderError> for StripeError {
    fn from(e: StripeDecoderError) -> StripeError {
        StripeError::DecodingError(e)
    }
}

impl From<io::Error> for StripeError {
    fn from(e: io::Error) -> StripeError {
        StripeError::IOError(e)
    }
}

impl From<HttpError> for StripeError {
    fn from(e: HttpError) -> StripeError {
        StripeError::TransportError(e)
    }
}

macro_rules! etry {
    ($expr:expr) => {
        match $expr {
            Ok(o) => o,
            Err(e) => return Err(StripeError::from(e)),
        }
    }
}

fn process<T: Decodable>(req: Request<Streaming>) -> Result<T, StripeError> {
    let mut response = etry!(req.send());

    let mut body = vec![];

    etry!(response.read_to_end(&mut body));
    let object: T = etry!(StripeDecoder::decode(body));

    return Ok(object);
}

impl Connection {
    pub fn new(secret_key: String) -> Connection {
        return Connection {
            base_url: Url::parse("https://api.stripe.com").unwrap(),
            secret_key: secret_key
        };
    }

    fn request(&self, method: Method, mut path: Vec<String>) -> Request<Fresh> {
        let mut url = self.base_url.clone();
        url.path_mut().unwrap().append(&mut path);
        let mut request = Request::new(method, url).unwrap();
        let mut auth = "Bearer ".to_string();
        auth.push_str(&self.secret_key[..]);
        request.headers_mut().set(Authorization(auth));

        return request;
    }

    fn fetch<T: Decodable>(req: Request<Fresh>) -> Result<T, StripeError> {
        let req = etry!(req.start());
        process(req)
    }

    pub fn create<T>(&self, object: T::Object) -> Result<T, StripeError>
        where T : Creatable + Decodable {
        let req = self.request(Post, urlify!("v1", T::path()));

        let mut req = etry!(req.start());

        let payload = form_urlencoded::serialize(object.into_iter());
        etry!(req.write_all(payload.as_bytes()));

        process(req)
    }

    pub fn customers(&self) -> Result<CustomerList, StripeError> {
        let req = self.request(Get, urlify!("v1", "customers"));
        return Connection::fetch(req);
    }

    pub fn cards(&self, customer: CustomerId) -> Result<CardList, StripeError> {
        let req = self.request(Get, urlify!("v1", "customers", customer, "cards"));
        return Connection::fetch(req);
    }

}
