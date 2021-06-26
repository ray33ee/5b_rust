use std::str::FromStr;
use std::convert::TryInto;

///Convertable types
pub struct Base2_16; //All numbers between base 2 and 16, each base implemented as a variant
pub struct FixedFloat; //16, 32, 64-bit floats
//pub struct ArbitraryFloat; //Arbitrary precision floats
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
pub struct UrlEncode;
pub struct UrlDecode;

impl FixedFloat {
    fn mantissa_exponent_to_string(repr: (i8, u64, i16)) -> String {
        let (sign, mantissa, exponent) = repr;
        format!("Mantissa: {}({:b}), exponent: {}({:b}), sign: {}({:b})", mantissa, mantissa, exponent, exponent, sign, sign)
    }

    fn get_repr(bytes: &[u8]) -> (i8, u64, i16) {
        let int_bytes = match bytes.len() {
            2 => u16::from_le_bytes(bytes.try_into().unwrap()) as u64,
            4 => u32::from_le_bytes(bytes.try_into().unwrap()) as u64,
            8 => u64::from_le_bytes(bytes.try_into().unwrap()),
            _ => panic!("Invalid float ({} bytes long)", bytes.len()),
        };

        let (bits, mantissa_size, mantissa_mask, exponent_mask) = match bytes.len() {
            2 => (16, 10, 0x3FF, 0x7C00),
            4 => (32, 23, 0x07FFFFF, 0x7F800000),
            8 => (64, 52, 0xFFFFFFFFFFFFF, 0x7FF0000000000000),
            _ => panic!("Invalid float ({} bytes long)", bytes.len()),
        };

        let mantissa = mantissa_mask & int_bytes;

        let sign = if int_bytes >> (bits - 1) == 1 { -1 } else { 1 };

        let exponent = (exponent_mask & int_bytes) >> mantissa_size;

        (sign, mantissa, exponent as i16)
    }

    pub fn to_mes_string(bytes: &[u8]) -> String {
        Self::mantissa_exponent_to_string(Self::get_repr(bytes))
    }
}

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

    pub fn is_oct(byte: u8) -> bool {
        byte >= '0' as u8 && byte <= '7' as u8
    }

    pub fn get_base(variant: & Variant) -> u64 {
        u64::from_str(&(variant.0)[5..]).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Variant(pub & 'static str);
