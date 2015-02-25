#[derive(RustcDecodable,Debug)]
pub struct SubscriptionList {
    object: String,
    has_more: bool,
    url: String,
    data: Vec<Subscription>
}

#[derive(RustcDecodable,Debug)]
pub struct Subscription;
