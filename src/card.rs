#[deriving(Decodable,Show)]
pub struct CardList {
    object: ~str,
    has_more: bool,
    url: ~str,
    pub data: ~[Card]
}

#[deriving(Decodable,Show)]
pub struct Card {
    id: ~str,
    object: ~str,
    last4: ~str,
    // type: ~str
    exp_month: uint,
    exp_year: uint,
    fingerprint: ~str,
    customer: ~str,
    country: ~str,
    name: ~str,
    address_line1: Option<~str>,
    address_line2: Option<~str>,
    address_city: Option<~str>,
    address_state: Option<~str>,
    address_zip: Option<~str>,
    address_country: Option<~str>,
    cvc_check: Option<~str>,
    address_line1_check: Option<~str>,
    address_zip_check: Option<~str>
}
