use collections::hashmap::HashMap;
use std::slice::Items;
use subscription::SubscriptionList;
use card::CardList;

pub type CustomerId = ~str;

// Decodable type from the API
#[deriving(Decodable,Show)]
pub struct CustomerList {
    object: ~str,
    has_more: bool,
    url: ~str,
    pub data: ~[Customer]
}

#[deriving(Decodable,Show)]
pub struct Customer {
    object: ~str,
    created: uint,
    id: CustomerId,
    livemode: bool,
    description: Option<~str>,
    email: Option<~str>,
    delinquent: bool,
    metadata: HashMap<~str,~str>, // FIXME This is a blatant lie.
    subscriptions: SubscriptionList,
    discount: Option<~str>,
    account_balance: f64,
    currency: Option<~str>,
    cards: CardList,
    default_card: Option<~str>
}

impl CustomerList {
    pub fn iter<'a>(&'a mut self) -> Items<'a, Customer> {
        self.data.iter()
    }
}
