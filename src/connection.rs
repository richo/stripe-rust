extern crate url;
extern crate http;
use http::client::RequestWriter;
use http::method::Get;
use http::headers::HeaderEnum;
use http::headers::request::ExtensionHeader;
use std::os;
use std::str;
use std::io::println;
use url::{Url,UserInfo};

type SecretKey = ~str;

struct Connection {
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
}
