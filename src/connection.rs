use std::string::String;
use std::io::Read;
use customer::{CustomerList,CustomerId};
use card::CardList;
use decoder::StripeDecoder;
use url::Url;
use serialize::{json,Decodable};
use hyper::client::Request;
use hyper::net::Fresh;
use hyper::Get;
use hyper::header::Authorization;
use hyper::error::HttpError;

type SecretKey = String;

pub struct Connection {
    baseUrl: Url,
    secretKey: SecretKey
}

macro_rules! urlify {
    ($($component:expr),*) => {
        vec![$($component.to_string()),*]
    }
}

impl Connection {
    pub fn new(secretKey: String) -> Connection {
        return Connection {
            baseUrl: Url::parse("https://api.stripe.com").unwrap(),
            secretKey: secretKey
        };
    }

    fn request(&self, mut path: Vec<String>) -> Request<Fresh> {
        let mut url = self.baseUrl.clone();
        url.path_mut().unwrap().append(&mut path);
        let mut request = Request::new(Get, url).unwrap();
        let mut auth = "Bearer ".to_string();
        auth.push_str(self.secretKey.as_slice());
        request.headers_mut().set(Authorization(auth));

        return request;
    }

    fn fetch<T: Decodable>(req: Request<Fresh>) -> Result<T, HttpError> {
        let connection = try!(req.start());
        let mut response = try!(connection.send());

        let mut body = vec![];
        let bytes = try!(response.read_to_end(&mut body));

        let object: T = StripeDecoder::decode(body);

        return Ok(object);
    }

    pub fn customers(&self) -> Result<CustomerList, HttpError> {
        let req = self.request(urlify!("v1", "customers"));
        return Connection::fetch(req);
    }

    pub fn cards(&self, customer: CustomerId) -> Result<CardList, HttpError> {
        let req = self.request(urlify!("v1", "customers", customer.as_slice(), "cards"));
        return Connection::fetch(req);
    }

}
