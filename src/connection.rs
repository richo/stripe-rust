use http::client::RequestWriter;
use http::method::Get;
use http::headers::request::ExtensionHeader;
use customer::CustomerList;
use decoder::Decoder;
use url::Url;
use serialize::{json,Decodable};

type SecretKey = ~str;

pub struct Connection {
    baseUrl: Url,
    secretKey: SecretKey
}

impl Connection {
    pub fn new(secretKey: ~str) -> Connection {
        return Connection {
            baseUrl: from_str("https://api.stripe.com").unwrap(),
            secretKey: secretKey
        };
    }

    fn request(&self, path: ~str) -> RequestWriter {
        let mut url = self.baseUrl.clone();
        url.path = path;
        let mut request: RequestWriter = RequestWriter::new(Get, url).unwrap();
        request.headers.insert(ExtensionHeader(~"Authorization", format!("Bearer {}", self.secretKey)));

        return request;
    }

    fn fetch<T: Decodable<json::Decoder,json::Error>>(req: RequestWriter) -> T {
        let mut response = match req.read_response() {
            Ok(response) => response,
            Err(err) => fail!("Something very bad has happened:"),
        };
        let body = match response.read_to_end() {
            Ok(body) => body,
            Err(err) => fail!("Something very bad has happened:"),
        };

        let object = Decoder::<T>::decode(body);

        return object;
    }

    pub fn customers(&self) -> CustomerList {
        let req = self.request(~"/v1/customers");
        return Connection::fetch(req);
    }
}
