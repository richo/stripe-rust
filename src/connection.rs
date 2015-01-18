use std::string::String;
use http::client::RequestWriter;
use http::method::Get;
use http::headers::request::ExtensionHeader;
use customer::{CustomerList,CustomerId};
use card::CardList;
use decoder::Decoder;
use url::Url;
use serialize::{json,Decodable};

type SecretKey = String;

pub struct Connection {
    baseUrl: Url,
    secretKey: SecretKey
}

impl Connection {
    pub fn new(secretKey: String) -> Connection {
        return Connection {
            baseUrl: from_str("https://api.stripe.com").unwrap(),
            secretKey: secretKey
        };
    }

    fn request(&self, path: String) -> RequestWriter {
        let mut url = self.baseUrl.clone();
        url.path = path.into_owned();
        let mut request: RequestWriter = RequestWriter::new(Get, url).unwrap();
        let mut auth = String::from_str("Bearer ");
        auth.push_str(self.secretKey.as_slice());
        request.headers.insert(ExtensionHeader(String::from_str("Authorization"), auth));

        return request;
    }

    fn fetch<T: Decodable<json::Decoder,json::DecoderError>>(req: RequestWriter) -> T {
        let mut response = match req.read_response() {
            Ok(response) => response,
            Err(_) => panic!("Something very bad has happened:"),
        };
        let body = match response.read_to_end() {
            Ok(body) => body,
            Err(err) => panic!("Something very bad has happened: {}", err),
        };

        let object = Decoder::<T>::decode(body);

        return object;
    }

    pub fn customers(&self) -> CustomerList {
        let req = self.request(String::from_str("/v1/customers"));
        return Connection::fetch(req);
    }

    pub fn cards(&self, customer: CustomerId) -> CardList {
        let mut url = String::from_str("/v1/customers/");
        url.push_str(customer.as_slice());
        url.push_str("/cards");
        let req = self.request(url);
        return Connection::fetch(req);
    }

}
