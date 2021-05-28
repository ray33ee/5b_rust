use std::str::FromStr;

pub type IR = [u8];

///Convertable types
pub struct Base16(pub String);

impl Base16 {
    pub fn ascii_to_num(byte: u8) -> u8 {
        if byte >= '0' as u8 && byte <= '9' as u8 {
            byte - ('0' as u8)
        } else if byte >= 'a' as u8 && byte <= 'f' as u8 {
            byte - ('a' as u8) + 10
        } else {
            byte - ('A' as u8) + 10
        }
    }

    pub fn num_to_ascii(byte: u8, is_capital: bool) -> u8 {
        if byte <= 9 {
            byte + ('0' as u8)
        } else {
            if is_capital {
                byte - 10 + ('A' as u8)
            } else {
                byte - 10 + ('a' as u8)
            }
        }
    }

    pub fn get_base(variant: & Variant) -> u64 {
        u64::from_str(variant.0).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Variant(pub & 'static str);

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