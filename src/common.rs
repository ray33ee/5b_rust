use std::str::FromStr;

///Convertable types
pub struct Base2_16; //All numbers between base 2 and 16, each base implemented as a variant
pub struct FixedFloat; //16, 32, 64-bit floats
pub struct ArbitraryFloat; //Arbitrary precision floats
pub struct DateTime; //32 and 64-bit unix time
pub struct FixedInt; //8-128 bit signed/unsigned integer
pub struct Unicode8;
pub struct IpV4;
pub struct IpV6;
pub struct Base64;
pub struct Base85;
pub struct Base91;
pub struct ByteList;
pub struct UUID;
pub struct Hash;
pub struct EscapedString;

impl Base2_16 {
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

    pub fn is_hex(byte: u8) -> bool {
        byte >= '0' as u8 && byte <= '9' as u8 || byte >= 'a' as u8 && byte <= 'f' as u8 || byte >= 'A' as u8 && byte <= 'F' as u8
    }

    pub fn get_base(variant: & Variant) -> u64 {
        u64::from_str(&(variant.0)[5..]).unwrap()
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