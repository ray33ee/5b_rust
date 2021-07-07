
use crate::common::{Variant, Base2_16, FixedInt};
use std::str::FromStr;
use half::f16;
use lazy_static::lazy_static;
use regex::Regex;

//A trait that defines functions to convert from & str to IR
pub trait ToIR {

    ///Used to test to see whether the input string can be converted to Self and return a set of variants that match the value
    fn identify(value: & str) -> Option<Vec<Variant>>;

    ///Used to convert an IR to Self. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn decode(value: & str, variant: Variant) -> Vec<u8>;
}

///Numbers from base 2 to base 16 using 0-9 and a-f
impl ToIR for crate::common::Base2_16 {

    fn identify(value: &str) -> Option<Vec<Variant>> {

        let mut chars = value.chars();

        if let Some(character) = chars.next() {
            if character == '0' {
                if let Some(character) = chars.next() {
                    match character {
                        'b' => return Some(vec![Variant("Base 2")]),
                        'o' => return Some(vec![Variant("Base 8")]),
                        'x' => return Some(vec![Variant("Base 16")]),
                        _ => {}
                    }
                }
            }
        }

        let is_valid_base16 = value.as_bytes().iter().fold(true, |acc, &x| {
            acc && ((x >= '0' as u8 && x <= '9' as u8) || (x >= 'a' as u8 && x <= 'f' as u8) || (x >= 'A' as u8 && x <= 'F' as u8))
        });

        //If the number does not convert to Hexadecimal, it cannot be valid base 16 or lower
        if !is_valid_base16 {
            return None;
        } else { //Get a list of all possible bases, based on the largest digit. (for example if the largest digit is 1, then it could be base 2 or more. If the largest digit is 7,, it could be base 8 or more)

            let largest = value
                .as_bytes()
                .iter()
                .map(|&x| Self::ascii_to_num(x))
                .max()
                .unwrap_or(0);

            let possible_bases= [Variant("Base 2"), //Variant("Base 3"), Variant("Base 4"), Variant("Base 5"), Variant("Base 6"), Variant("Base 7"),
                Variant("Base 8"), //Variant("Base 9"),
                Variant("Base 10"), //Variant("Base 11"), Variant("Base 12"), Variant("Base 13"), Variant("Base 14"), Variant("Base 15"),
                Variant("Base 16")
            ];

            let index = if largest < 2 {
                    0
                } else if largest >= 2 && largest < 8 {
                    1
                } else if largest >= 8 && largest < 10 {
                    2
                } else {
                    3
                }
            ;

            let variants = Vec::from(&possible_bases[index..]);

            Some(variants)

        }

    }

    fn decode(mut value: &str, variant: Variant) -> Vec<u8> {

        if value.len() > 1 {
            if &value[0..2] == "0b" || &value[0..2] == "0o" || &value[0..2] == "0x" {
                value = &value[2..];
            }
        }

        let base = Self::get_base(&variant);

        //Convert ascii bytes 0-9a-fA-F into digits
        let input: Vec<_> = value.as_bytes().iter()
            .rev()
            .map(|&byte| Self::ascii_to_num(byte)).collect();

        convert_base::Convert::new(base, 256).convert::<u8, u8>(& input)
    }
}

///All common fixed precision floating point numbers (16, 32 and 64-bit)
impl ToIR for crate::common::FixedFloat {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        //If conversion to f64 succeeds, then conversion to f32 and f16 will also succeed
        if f64::from_str(value).is_ok() {
            Some(vec![Variant("32-bit"), Variant("16-bit"), Variant("64-bit"),])
        } else {
            None
        }
    }

    fn decode(value: &str, variant: Variant) ->  Vec<u8> {

        let number = f64::from_str(value).unwrap();

        let bytes = match variant.0 {
            "16-bit" => Vec::from(f16::from_f64(number).to_le_bytes()),
            "32-bit" => Vec::from((number as f32).to_le_bytes()),
            "64-bit" => Vec::from(number.to_le_bytes()),
            _ => panic!("Invalid variant in FixedFloat")
        };

        bytes
    }
}

impl ToIR for crate::common::DateTime {
    fn identify(value: &str) -> Option<Vec<Variant>> {

        let mut variants = Vec::new();

        if chrono::DateTime::parse_from_rfc2822(value).is_ok() {
            variants.push(Variant("32-bit rfc2822"));
            variants.push(Variant("64-bit rfc2822"));
        }

        if chrono::DateTime::parse_from_rfc3339(value).is_ok() {
            variants.push(Variant("32-bit rfc3339"));
            variants.push(Variant("64-bit rfc3339"));
        }

        if variants.is_empty() {
            None
        } else {
            Some(variants)
        }
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {
        let datetime = match &(variant.0)[7..14] {
            "rfc2822" => chrono::DateTime::parse_from_rfc2822(value),
            "rfc3339" => chrono::DateTime::parse_from_rfc3339(value),
            _ => panic!("Invalid rfc variant in DateTime ToIR")
        }.unwrap();

        let timestamp = datetime.timestamp();

        match &(variant.0)[0..2] {
            "32" => Vec::from((timestamp as i32).to_le_bytes()),
            "64" => Vec::from(timestamp.to_le_bytes()),
            _ => panic!("Invalid size variant in DateTime ToIR"),
        }
    }
}

impl ToIR for crate::common::FixedInt {
    fn identify(value: &str) -> Option<Vec<Variant>> {

        let mut variants = vec![];

        /*let base2_16_size = if let Some(base2_16_variants) = Base2_16::identify(value) {

            println!("base variants: {:?}", base2_16_variants);

            if variants.len() == 1 {
                Some(Base2_16::decode(value, base2_16_variants.get(0).unwrap().clone()).len())
            } else {
                if variants.contains(&Variant("Base 10")) {
                    Some()
                }
                Some(Base2_16::decode(value, base2_16_variants.get(0).unwrap().clone()).len())
            }

        } else {
            None
        };*/

        let base2_16_size = FixedInt::get_base2_16_variant(value)
            .map(|x| Base2_16::decode(value, x).len());

        println!("Size: {:?} {}", base2_16_size, value);

        if /*i128::from_str(value).is_ok() ||*/ base2_16_size <= Some(16) {
            variants.insert(0, Variant("i128"));
        }

        if /*u128::from_str(value).is_ok() ||*/ base2_16_size <= Some(16) {
            variants.insert(0, Variant("u128"));
        }

        if /*i64::from_str(value).is_ok() ||*/ base2_16_size <= Some(8) {
            variants.insert(0, Variant("i64"));
        }

        if /*u64::from_str(value).is_ok() ||*/ base2_16_size <= Some(8) {
            variants.insert(0, Variant("u64"));
        }

        if /*i32::from_str(value).is_ok() ||*/ base2_16_size <= Some(4) {
            variants.insert(0, Variant("i32"));
        }

        if /*u32::from_str(value).is_ok() ||*/ base2_16_size <= Some(4) {
            variants.insert(0, Variant("u32"));
        }

        if /*i16::from_str(value).is_ok() ||*/ base2_16_size <= Some(2) {
            variants.insert(0, Variant("i16"));
        }

        if /*u16::from_str(value).is_ok() ||*/ base2_16_size <= Some(2) {
            variants.insert(0, Variant("u16"));
        }

        if /*i8::from_str(value).is_ok() ||*/ base2_16_size <= Some(1) {
            variants.insert(0, Variant("i8"));
        }

        if /*u8::from_str(value).is_ok() ||*/ base2_16_size <= Some(1) {
            variants.insert(0, Variant("u8"));
        }


        if variants.is_empty() {
            None
        } else {
            Some(variants)
        }
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {

        let base2_16_variant = FixedInt::get_base2_16_variant(value);

        let (mut bytes, size_required) = match variant.0 {
            "i8" => (i8::from_str(value)
                         .map(|x| vec![x as u8])
                         .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 1),
            "i16" => (i16::from_str(value)
                          .map(|x| Vec::from(x.to_le_bytes()))
                          .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 2),
            "i32" => (i32::from_str(value)
                          .map(|x| Vec::from(x.to_le_bytes()))
                          .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 4),
            "i64" => (i64::from_str(value)
                          .map(|x| Vec::from(x.to_le_bytes()))
                          .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 8),
            "i128" => (i128::from_str(value)
                           .map(|x| Vec::from(x.to_le_bytes()))
                           .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 16),

            "u8" => (u8::from_str(value)
                         .map(|x| vec![x])
                         .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 1),
            "u16" => (u16::from_str(value)
                          .map(|x| Vec::from(x.to_le_bytes()))
                          .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 2),
            "u32" => (u32::from_str(value)
                          .map(|x| Vec::from(x.to_le_bytes()))
                          .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 4),
            "u64" =>(u64::from_str(value)
                         .map(|x| Vec::from(x.to_le_bytes()))
                         .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 8),
            "u128" => (u128::from_str(value)
                           .map(|x| Vec::from(x.to_le_bytes()))
                           .unwrap_or_else(|_| Base2_16::decode(value, base2_16_variant.unwrap())), 16),
            _ => panic!("Invalid variant in ToIR FixedInt")
        };

        //If the value was converted via Base2_16, extra padding may be needed
        if bytes.len() != size_required {
            for _ in 0..size_required - bytes.len() {
                bytes.push(0);
            }
        }

        bytes
    }
}

impl ToIR for crate::common::Unicode8 {
    fn identify(_value: &str) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {
        if variant.0 == "" {
            unsafe {
                String::from(value).as_mut_vec().clone()
            }
        } else {
            panic!("Invalid variant in ToIR Unicode8");
        }
    }
}

impl ToIR for crate::common::IpV4 {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        if value.parse::<std::net::SocketAddrV4>().is_ok() {
            return Some(vec![Variant("with port")]);
        }

        if value.parse::<std::net::Ipv4Addr>().is_ok() {
            return Some(vec![Variant("without port")]);
        }

        None
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {

        let mut result = Vec::new();

        let (ipaddr, op_port) = match variant.0 {
            "with port" => {
                let socket = value.parse::<std::net::SocketAddrV4>().unwrap();

                (socket.ip().clone(), Some(socket.port()))
            },
            "without port" => {
                let ip = value.parse::<std::net::Ipv4Addr>().unwrap();

                (ip, None)
            },
            _ => panic!("Invalid variant in ToIR IpV4"),
        };

        if let Some(port) = op_port {
            result.extend_from_slice(&port.to_be_bytes());
        }

        result.extend_from_slice(&ipaddr.octets());

        result.reverse();

        result
    }
}

impl ToIR for crate::common::IpV6 {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        if value.parse::<std::net::SocketAddrV6>().is_ok() {
            return Some(vec![Variant("with port")]);
        }

        if value.parse::<std::net::Ipv6Addr>().is_ok() {
            return Some(vec![Variant("without port")]);
        }

        None
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {

        let mut result = Vec::new();

        let (ipaddr, op_port) = match variant.0 {
            "with port" => {
                let socket = value.parse::<std::net::SocketAddrV6>().unwrap();

                (socket.ip().clone(), Some(socket.port()))
            },
            "without port" => {
                let ip = value.parse::<std::net::Ipv6Addr>().unwrap();

                (ip, None)
            },
            _ => panic!("Invalid variant in ToIR IpV6"),
        };

        if let Some(port) = op_port {
            result.extend_from_slice(&port.to_be_bytes());
        }

        result.extend_from_slice(&ipaddr.octets());

        result.reverse();

        result
    }
}

impl ToIR for crate::common::Base91 {
    fn identify(_value: &str) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn decode(value: &str, _variant: Variant) -> Vec<u8> {
        base91::slice_decode(value.as_bytes())
    }
}

impl ToIR for crate::common::Base85 {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        let mut variants = Vec::new();

        if z85::decode(value.as_bytes()).is_ok() {
            variants.push(Variant("z85"));
        }

        /*if ascii85::decode(value).is_ok() {
            variants.push(Variant("ascii85"));
        }*/

        if variants.is_empty() {
            None
        } else {
            Some(variants)
        }

    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {

        match variant.0 {
            "z85" => z85::decode(value.as_bytes()).unwrap(),
            "ascii85" => panic!("Due to an issue with with the ascii85 crate, this is not suported"), //ascii85::decode(value).unwrap(),
            _ => panic!("Invalid variant in ToIR Base85")
        }
    }
}

impl ToIR for crate::common::Base64 {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        let mut variants = Vec::new();

        if base64::decode_config(value, base64::BCRYPT).is_ok() {
            variants.push(Variant("Bcrypt"));
        }

        if base64::decode_config(value, base64::BINHEX).is_ok() {
            variants.push(Variant("BinHex"));
        }

        if base64::decode_config(value, base64::CRYPT).is_ok() {
            variants.push(Variant("crypt"));
        }

        if base64::decode_config(value, base64::IMAP_MUTF7).is_ok() {
            variants.push(Variant("IMAP UTF-7"));
        }

        if base64::decode_config(value, base64::STANDARD).is_ok() {
            variants.push(Variant("Standard"));
        }

        if base64::decode_config(value, base64::STANDARD_NO_PAD).is_ok() {
            variants.push(Variant("Standard no padding"));
        }

        if base64::decode_config(value, base64::URL_SAFE).is_ok() {
            variants.push(Variant("URL-safe"));
        }

        if base64::decode_config(value, base64::URL_SAFE_NO_PAD).is_ok() {
            variants.push(Variant("URL-safe no padding"));
        }

        if variants.is_empty() {
            None
        } else {
            Some(variants)
        }
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {
        match variant.0 {
            "Bcrypt" => base64::decode_config(value, base64::BCRYPT).unwrap(),
            "BinHex" => base64::decode_config(value, base64::BINHEX).unwrap(),
            "crypt" => base64::decode_config(value, base64::CRYPT).unwrap(),
            "IMAP UTF-7" => base64::decode_config(value, base64::IMAP_MUTF7).unwrap(),
            "Standard" => base64::decode_config(value, base64::STANDARD).unwrap(),
            "Standard no padding" => base64::decode_config(value, base64::STANDARD_NO_PAD).unwrap(),
            "URL-safe" => base64::decode_config(value, base64::STANDARD_NO_PAD).unwrap(),
            "URL-safe no padding" => base64::decode_config(value, base64::URL_SAFE_NO_PAD).unwrap(),
            _ => panic!("Invalid variant in ToIr Base64"),
        }
    }
}

impl ToIR for crate::common::ByteList {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        //Remove whitespace

        lazy_static! {
            static ref BYTE_LIST: Regex = Regex::new("\\[(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5]),)*([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5]),?\\]").unwrap();
            static ref WHITESPACE: Regex = Regex::new("\\s").unwrap();
        }

        let cleaned = WHITESPACE.replace_all(value, "");

        if BYTE_LIST.is_match(&cleaned) {
            Some(vec![Variant("")])
        } else {
            None
        }
    }

    fn decode(value: &str, _variant: Variant) -> Vec<u8> {

        let mut  bytes = Vec::new();

        lazy_static! {
            static ref COMMA: Regex = Regex::new(",").unwrap();
            static ref WHITESPACE: Regex = Regex::new("\\s").unwrap();
        }

        let cleaned = WHITESPACE.replace_all(value, "");

        let value = &cleaned[1..cleaned.len()-1];

        for str_number in COMMA.split(value) {
            if let Ok(number) = u8::from_str(str_number) {
                bytes.push(number);
            }
        }

        bytes

    }
}

impl ToIR for crate::common::UUID {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        if uuid::Uuid::parse_str(value).is_ok() {
            Some(vec![Variant("")])
        } else {
            None
        }
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {
        if variant.0 == "" {
            let id = uuid::Uuid::parse_str(value).unwrap();

            Vec::from(id.as_u128().to_le_bytes())
        } else {
            panic!("Invalid variant in ToIR UUID");
        }
    }
}

impl ToIR for crate::common::EscapedString {
    fn identify(value: &str) -> Option<Vec<Variant>> {

        let mut variants = Vec::new();

        if crate::escape::EscapeSequence::decode(value, crate::escape::Variant::C).is_ok() {
            variants.push(Variant("C"))
        }

        if crate::escape::EscapeSequence::decode(value, crate::escape::Variant::Python).is_ok() {
            variants.push(Variant("Python"))
        }

        if variants.is_empty() {
            None
        } else {
            Some(variants)
        }
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {

        match variant.0 {
            "C" => crate::escape::EscapeSequence::decode(value, crate::escape::Variant::C).unwrap(),
            "Python" => crate::escape::EscapeSequence::decode(value, crate::escape::Variant::Python).unwrap(),
            _ => panic!("Invalid variant in ToIR EscapedString"),
        }
    }
}

impl ToIR for crate::common::UnicodeNames {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        if unicode_names2::character(value).is_some() {
            Some(vec![Variant("")])
        } else {
            None
        }
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {
        if variant.0 == "" {
            let mut bytes = Vec::new();

            let character = unicode_names2::character(value).unwrap();

            bytes.extend_from_slice(character.to_string().as_bytes());

            bytes
        } else {
            panic!("Invalid variant in ToIR UUID");
        }
    }
}

impl ToIR for crate::common::Colour {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        if value.len() == 7 {
            if &value[0..1] == "#" {
                if crate::common::Base2_16::identify(&value[1..]).is_some() {
                    return Some(vec![Variant("")]);
                }
            }
        }

        None
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {
        if variant.0 == "" {
            crate::common::Base2_16::decode(&value[1..], Variant("Base 16"))
        } else {
            panic!("Invalid variant in ToIR Colour");
        }
    }
}

/*

impl ToIR for crate::common:: {
    fn identify(value: &str) -> Option<Vec<Variant>> {
        unimplemented!()
    }

    fn decode(value: &str, variant: Variant) -> Vec<u8> {
        unimplemented!()
    }
}

 */
