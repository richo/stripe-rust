#[deriving(Decodable,Show)]
pub struct CardList {
    object: ~str,
    has_more: bool,
    url: ~str,
    pub data: ~[Card]
}

#[deriving(Decodable,Show)]
pub struct Card {
    object: ~str,
    id: ~str,
    last4: ~str,
    exp_month: uint,
    exp_year: uint,
    fingerprint: ~str,
    customer: Option<~str>, // Customerlist, but I'm not sure how that works
                            // with circular references
    country: ~str,
    name: Option<~str>,
    address_line1: Option<~str>,
    address_line2: Option<~str>,
    address_city: Option<~str>,
    address_state: Option<~str>,
    address_zip: Option<~str>,
    address_country: Option<~str>,
    cvc_check: ~str,
    address_line1_check: ~str,
    address_zip_check: ~str
}
