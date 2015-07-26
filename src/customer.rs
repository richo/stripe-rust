use std::collections::HashMap;
use subscription::SubscriptionList;
use card::CardList;
use std::iter::{IntoIterator};

use connection::Connection;
use util::{Creatable,UrlEncodable};

pub type CustomerId = String;

// Decodable type from the API
#[derive(RustcDecodable,Debug,RustcEncodable)]
pub struct CustomerList {
    object: String,
    has_more: bool,
    url: String,
    pub data: Vec<Customer>
}

#[derive(RustcDecodable,Debug,RustcEncodable)]
pub struct Customer {
    object: String,
    created: usize,
    id: CustomerId,
    livemode: bool,
    description: Option<String>,
    email: Option<String>,
    delinquent: bool,
    metadata: HashMap<String,String>, // FIXME This is a blatant lie.
    subscriptions: SubscriptionList,
    discount: Option<String>,
    account_balance: f64,
    currency: Option<String>,
    cards: Option<CardList>,
    default_card: Option<String>
}

creatable!(Customer, CustomerRequest, "customers",
           (source => String,
            email => String));

// TODO(richo) rip this out into a trait?
// TODO(richo) Alternately, to avoid parameter hell, materialize a Customer and then ::save?

impl Customer {
    pub fn create(conn: Connection, email: String, source: String) -> Customer {
        let tmp = CustomerRequest {
            source: Some(source),
            email: Some(email),
        };

        match conn.create(tmp) {
            Ok(o) => o,
            Err(e) => panic!("{:?}", e),
        }
    }
}

iterable!(CustomerList, Customer);
