#[deriving(Decodable,Show)]
pub struct SubscriptionList {
    object: ~str,
    has_more: bool,
    url: ~str,
    data: ~[Subscription]
}

#[deriving(Decodable,Show)]
pub struct Subscription;
