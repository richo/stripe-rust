use rustc_serialize::Encodable;

pub trait UrlEncodable {
    fn into_iter(self) -> Vec<(&'static str, String)>;
}

pub trait Creatable {
    type Object : Encodable + UrlEncodable;

    fn path() -> &'static str;
}

