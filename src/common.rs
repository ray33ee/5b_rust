
pub type IR = [u8];

///Convertable types
pub struct Hex(pub String);
pub struct Base64Standard(pub String);

pub struct Variant(pub & 'static str);

pub enum SelectVariant {
    Chose(Variant),
    Deduce,
}

///Constains metadata about a type T to aid in conversion
pub struct Information {
    identifier: & 'static str,
}

impl Information {
    pub fn new(identifier: & 'static str) -> Self {
        Self {
            identifier,
        }
    }
}