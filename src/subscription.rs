use enumerable::Enumerable;

#[deriving(Decodable,Show)]
pub struct SubscriptionList {
    object: ~str,
    has_more: bool,
    url: ~str,
    data: ~[Subscription]
}

#[deriving(Decodable,Show)]
pub struct Subscription;

impl Enumerable<Subscription> for SubscriptionList {
    fn items(&self) -> ~[Subscription] {
        return ~[];
    }
}
