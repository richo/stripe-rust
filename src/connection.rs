use http::client::RequestWriter;
use http::method::Get;
use http::headers::request::ExtensionHeader;
use url::Url;

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

    pub fn customers(&self) -> RequestWriter {
        return self.request(~"/v1/customers");
    }
}
