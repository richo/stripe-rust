use collections::hashmap::HashMap;
use subscription::SubscriptionList;
use card::CardList;
use enumerable::Enumerable;

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
    id: ~str, // TODO typeclass for stripe ID's?
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

impl Enumerable<Customer> for CustomerList {
    fn items(&self) -> &[Customer] {
        return self.data;
    }
}
