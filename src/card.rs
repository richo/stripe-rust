use enumerable::Enumerable;
use std::slice::Items;

#[deriving(Decodable,Show)]
pub struct CardList {
    object: ~str,
    has_more: bool,
    url: ~str,
    pub data: ~[Card]
}

#[deriving(Decodable,Show)]
pub struct Card;

impl Enumerable<Card> for CardList {
    fn iter(&mut self) -> Items<Card> {
        return self.data.iter();
    }
}
