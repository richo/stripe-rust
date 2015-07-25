use std::fmt;

const CARD_NO_LEN: usize = 16;

#[derive(RustcDecodable,Debug,RustcEncodable)]
pub struct CardList {
    object: String,
    has_more: bool,
    url: String,
    pub data: Vec<Card>,
}

#[derive(RustcDecodable,Debug,RustcEncodable)]
pub struct Card {
    id: String,
    object: String,
    last4: String,
    // type: String
    exp_month: usize,
    exp_year: usize,
    fingerprint: String,
    customer: String,
    country: String,
    name: String,
    address_line1: Option<String>,
    address_line2: Option<String>,
    address_city: Option<String>,
    address_state: Option<String>,
    address_zip: Option<String>,
    address_country: Option<String>,
    cvc_check: Option<String>,
    address_line1_check: Option<String>,
    address_zip_check: Option<String>
}

pub struct CardNumber {
    x: [u8; CARD_NO_LEN]
}

impl fmt::Debug for CardNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // // TODO This doens't actually format properly
        self.x.fmt(f)
    }
}

fn char2int(c: char) -> Option<u8> {
    match c {
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        '0' => Some(0),
        _ => None
    }
}

impl CardNumber {
    pub fn new(cc: &str) -> Option<CardNumber> {
        let mut numbers = [0u8; CARD_NO_LEN];
        if cc.len() != CARD_NO_LEN {
            return None;
        }
        for (idx, c) in cc.chars().enumerate() {
            match char2int(c) {
                Some(i) => numbers[idx] = i,
                None => return None
            }
        }
        Some(CardNumber { x: numbers })
    }
}
