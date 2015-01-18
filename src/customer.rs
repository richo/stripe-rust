use std::collections::hashmap::HashMap;
use std::slice::Items;
use subscription::SubscriptionList;
use card::CardList;

pub type CustomerId = String;

// Decodable type from the API
#[deriving(Decodable,Show)]
pub struct CustomerList {
    object: String,
    has_more: bool,
    url: String,
    pub data: Customer
}

#[deriving(Decodable,Show)]
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

impl CustomerList {
    pub fn iter<'a>(&'a mut self) -> Items<'a, Customer> {
        self.data.iter()
    }
}
