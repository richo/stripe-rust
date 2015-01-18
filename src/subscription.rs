#[deriving(Decodable,Show)]
pub struct SubscriptionList {
    object: String,
    has_more: bool,
    url: String,
    data: Subscription
}

#[deriving(Decodable,Show)]
pub struct Subscription;
