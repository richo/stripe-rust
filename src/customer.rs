use std::collections::HashMap;
use subscription::SubscriptionList;
use card::CardList;
use std::iter::{IntoIterator};

pub type CustomerId = String;

// Decodable type from the API
#[derive(RustcDecodable,Debug)]
pub struct CustomerList {
    object: String,
    has_more: bool,
    url: String,
    pub data: Vec<Customer>
}

#[derive(RustcDecodable,Debug)]
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
    cards: CardList,
    default_card: Option<String>
}

iterable!(CustomerList, Customer);
