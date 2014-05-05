use std::fmt;

static CARD_NO_LEN: uint = 16;

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

pub struct CardNumber {
    x: [u8, ..CARD_NO_LEN]
}

impl fmt::Show for CardNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{} {} {} {}",
               self.x.slice(0, 4), self.x.slice(4, 8),
               self.x.slice(8, 12), self.x.slice(12, 16))
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
        let mut numbers = [0u8, ..CARD_NO_LEN];
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
