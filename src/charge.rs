use connection::{Connection,StripeError};
use util::{Creatable,UrlEncodable};
use customer::Customer;

pub type ChargeId = String;

#[derive(RustcDecodable,Debug,RustcEncodable)]
pub struct Charge {
    pub id: ChargeId,
    object: String,
    livemode: bool,
    created: usize,
    status: String,
}

creatable!(Charge, ChargeRequest, "charges",
           (amount => usize,
            currency => String,
            customer => String,
            source => String
            ));

impl Charge {
    pub fn create(conn: Connection, amount: usize, customer: Customer) -> Result<Charge,StripeError> {
        let tmp = ChargeRequest {
            amount: Some(amount),
            customer: Some(customer.id),
            // TODO(richo) Configurable currency
            currency: Some("USD".to_string()),
            source: None,
        };

        conn.create(tmp)
    }
}
