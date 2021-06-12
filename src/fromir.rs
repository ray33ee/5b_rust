use crate::common::{Variant};
use half::f16;
use std::convert::TryInto;
use std::str::{from_utf8, from_utf8_unchecked};
use std::net::{SocketAddrV4, SocketAddrV6};
use std::io::Write;
use std::borrow::Cow;

//A trait that defines functions to convert from IR to T
pub trait FromIR {

    /// Used to identify all the possible variants that the IR can represent
    fn variants(ir: & [u8]) -> Option<Vec<Variant>>;

    ///Used to convert an IR to a String. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn encode(ir: & [u8], variant: Variant) -> String;
}

impl FromIR for crate::common::Base2_16 {
    fn variants(_ir: & [u8]) -> Option<Vec<Variant>> {
        //Any set of bytes can be converted to base 2-16
        Some(vec![Variant("Base 2"), //Variant("Base 3"), Variant("Base 4"), Variant("Base 5"), Variant("Base 6"), Variant("Base 7"),
             Variant("Base 8"), //Variant("Base 9"),
             Variant("Base 10"), //Variant("Base 11"), Variant("Base 12"), Variant("Base 13"), Variant("Base 14"), Variant("Base 15"),
             Variant("Base 16")])
    }

    fn encode(ir: & [u8], variant: Variant) -> String {
        let base = Self::get_base(&variant);

        let mut base_n_list = convert_base::Convert::new(256, base).convert::<u8, u8>(ir.as_ref());

        let mut string = String::new();

        for byte in base_n_list.iter_mut() {
            string.insert(0, Self::num_to_ascii(*byte, true) as char)
        }

        string
    }
}

impl FromIR for crate::common::FixedFloat {
    fn variants(ir: & [u8]) -> Option<Vec<Variant>> {
        let len = ir.as_ref().len();

        match len {
            2 => Some(vec![Variant("16-bit")]),
            4 => Some(vec![Variant("32-bit")]),
            8 => Some(vec![Variant("64-bit")]),
            _ => None,
        }

    }

    fn encode(ir: & [u8], variant: Variant) -> String {
        let float = match variant.0 {
            "16-bit" => f16::from_le_bytes(ir.as_ref().try_into().unwrap()).to_f64(),
            "32-bit" => f32::from_le_bytes(ir.as_ref().try_into().unwrap()) as f64,
            "64-bit" => f64::from_le_bytes(ir.as_ref().try_into().unwrap()),
            _ => panic!("Invalid variant in FromIT FixedFloat")
        };

        float.to_string()
    }
}

impl FromIR for crate::common::FixedInt {
    fn variants(ir: & [u8]) -> Option<Vec<Variant>> {
        let len = ir.as_ref().len();

        match len {
            1 => Some(vec![Variant("u8"), Variant("i8")]),
            2 => Some(vec![Variant("u16"), Variant("i16")]),
            4 => Some(vec![Variant("u32"), Variant("i32")]),
            8 => Some(vec![Variant("u64"), Variant("i64")]),
            16 => Some(vec![Variant("u128"), Variant("i128")]),
            _ => None,
        }
    }

    fn encode(ir: & [u8], variant: Variant) -> String {
        match variant.0 {
            "u8" => u8::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            "i8" => i8::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            "u16" => u16::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            "i16" => i16::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            "u32" => u32::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            "i32" => i32::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            "u64" => u64::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            "i64" => i64::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            "u128" => u128::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            "i128" => i128::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            _ => panic!("Invalid variant in FromIT FixedInt")
        }
    }
}

impl FromIR for crate::common::DateTime {
    fn variants(ir: & [u8]) -> Option<Vec<Variant>> {
        let len = ir.as_ref().len();

        match len {
            4 => Some(vec![Variant("32-bit rfc2822"), Variant("32-bit rfc3339")]),
            8 => {

                //Not all combinations of 64-bits result in a valid date, so we

                let timestamp = i64::from_le_bytes(ir.as_ref().try_into().unwrap());

                if chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0).is_some() {
                    Some(vec![Variant("64-bit rfc2822"), Variant("64-bit rfc3339")])
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn encode(ir: & [u8], variant: Variant) -> String {

        let timestamp = match &(variant.0)[0..2] {
            "64" => i64::from_le_bytes(ir.as_ref().try_into().unwrap()),
            "32" => i32::from_le_bytes(ir.as_ref().try_into().unwrap()) as i64,
            _ => panic!("Invalid variant (width) in FromIT DateTime"),
        };

        let datetime = chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(timestamp, 0), chrono::Utc);

        match &(variant.0)[7..14] {
            "rfc2822" => datetime.to_rfc2822(),
            "rfc3339" => datetime.to_rfc3339(),
            "custom " => datetime.format(&(variant.0)[14..]).to_string(),
            _ => panic!("Invalid variant (format) in FromIT DateTime")
        }

    }
}

impl FromIR for crate::common::Unicode8 {
    fn variants(ir: &[u8]) -> Option<Vec<Variant>> {
        if from_utf8(ir).is_ok() {
            Some(vec![Variant("")])
        } else {
            None
        }
    }

    fn encode(ir: &[u8], variant: Variant) -> String {
        if variant.0 == "" {
            unsafe {
                String::from(from_utf8_unchecked(ir))
            }
        } else {
            panic!("Invalid variant in FromIR Unicode8");
        }
    }
}

impl FromIR for crate::common::IpV4 {
    fn variants(ir: &[u8]) -> Option<Vec<Variant>> {
        let len = ir.as_ref().len();

        match len {
            4 => Some(vec![Variant("without port")]),
            6 => Some(vec![Variant("with port")]),
            _ => None,
        }
    }

    fn encode(ir: &[u8], variant: Variant) -> String {

        let ip = std::net::Ipv4Addr::from(u32::from_le_bytes((&ir[0..4]).try_into().unwrap()));

        match variant.0 {
            "with port" => {
                let port = u16::from_le_bytes((&ir[4..6]).try_into().unwrap());

                SocketAddrV4::new(ip, port).to_string()
            },
            "without port" => {

                ip.to_string()
            }
            _ => panic!("Invalid variant in FromIT IpV4"),
        }
    }
}

impl FromIR for crate::common::IpV6 {
    fn variants(ir: &[u8]) -> Option<Vec<Variant>> {
        let len = ir.as_ref().len();

        match len {
            16 => Some(vec![Variant("without port")]),
            18 => Some(vec![Variant("with port")]),
            _ => None,
        }
    }

    fn encode(ir: &[u8], variant: Variant) -> String {

        let ip = std::net::Ipv6Addr::from(u128::from_le_bytes((&ir[0..16]).try_into().unwrap()));

        match variant.0 {
            "with port" => {
                let port = u16::from_le_bytes((&ir[16..18]).try_into().unwrap());

                SocketAddrV6::new(ip, port, 0, 0).to_string()
            },
            "without port" => {

                ip.to_string()
            }
            _ => panic!("Invalid variant in FromIT IpV4"),
        }
    }
}

impl FromIR for crate::common::Base91 {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn encode(ir: &[u8], _variant: Variant) -> String {
        unsafe {
            String::from_utf8_unchecked(base91::slice_encode(ir))
        }
    }
}

impl FromIR for crate::common::Base85 {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("z85"), Variant("ascii85")])
    }

    fn encode(ir: &[u8], variant: Variant) -> String {
        match variant.0 {
            "z85" => z85::encode(ir),
             "ascii85" => ascii85::encode(ir),
            _ => panic!("Invalid variant in FromIR Base85")
        }
    }
}

impl FromIR for crate::common::Base64 {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("Bcrypt"),
             Variant("BinHex"),
             Variant("crypt"),
             Variant("IMAP UTF-7"),
             Variant("Standard"),
             Variant("Standard no padding"),
             Variant("URL-safe"),
             Variant("URL-safe no padding"),])
    }

    fn encode(ir: &[u8], variant: Variant) -> String {
        match variant.0 {
            "Bcrypt" => base64::encode_config(ir, base64::BCRYPT),
            "BinHex" => base64::encode_config(ir, base64::BINHEX),
            "crypt" => base64::encode_config(ir, base64::CRYPT),
            "IMAP UTF-7" => base64::encode_config(ir, base64::IMAP_MUTF7),
            "Standard" => base64::encode_config(ir, base64::STANDARD),
            "Standard no padding" => base64::encode_config(ir, base64::STANDARD_NO_PAD),
            "URL-safe" => base64::encode_config(ir, base64::STANDARD_NO_PAD),
            "URL-safe no padding" => base64::encode_config(ir, base64::URL_SAFE_NO_PAD),
            _ => panic!("Invalid variant in FromIr Base64"),
        }
    }
}

impl FromIR for crate::common::ByteList {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn encode(ir: &[u8], _variant: Variant) -> String {
        format!("({} byte(s)) {:?}", ir.len(), ir)
    }
}

impl FromIR for crate::common::Hash {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("md5"), Variant("sha1"), Variant("sha256"), Variant("sha512")])
    }

    fn encode(ir: &[u8], variant: Variant) -> String {
        let algorithm = match variant.0 {
            "md5" => crypto_hash::Algorithm::MD5,
            "sha1" => crypto_hash::Algorithm::SHA1,
            "sha256" => crypto_hash::Algorithm::SHA256,
            "sha512" => crypto_hash::Algorithm::SHA512,
            _ => panic!("Invalid variant FromIT Hash")
        };

        let mut hasher = crypto_hash::Hasher::new(algorithm);

        hasher.write_all(ir).unwrap();

        let mut hash = hasher.finish();

        hash.reverse();

        crate::common::Base2_16::encode(&hash, Variant("Base 16"))
    }
}

impl FromIR for crate::common::UUID {
    fn variants(ir: &[u8]) -> Option<Vec<Variant>> {
        if uuid::Uuid::from_slice(ir).is_ok() {
            Some(vec![Variant("")])
        } else {
            None
        }
    }

    fn encode(ir: &[u8], variant: Variant) -> String {
        if variant.0 == "" {
            uuid::Uuid::from_slice(ir).unwrap().to_string()
        } else {
            panic!("Invalid variant in FromIT uuid")
        }
    }
}

impl FromIR for crate::common::EscapedString {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn encode(ir: &[u8], variant: Variant) -> String {
        if variant.0 == "" {
            crate::escape::EscapeSequence::encode(ir)
        } else {
            panic!("Invalid variant in FromIR EscapedString");
        }
    }
}

impl FromIR for crate::common::UrlEncode {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn encode(ir: &[u8], variant: Variant) -> String {
        if variant.0 == "" {
            urlencoding::encode_binary(ir).to_string()
        } else {
            panic!("Invalid variant in FromIR UrlEncode");
        }
    }
}

impl FromIR for crate::common::UrlDecode {
    fn variants(ir: &[u8]) -> Option<Vec<Variant>> {
        if from_utf8(ir).is_ok() {
            Some(vec![Variant("Legacy"), Variant("RFC 3986")])
        } else {
            None
        }
    }

    fn encode(ir: &[u8], variant: Variant) -> String {

        let string = if variant.0 == "RFC 3986" {
            let string = String::from_utf8_lossy(ir);
            Cow::from(string.replace("+", "%20"))
        } else {
            String::from_utf8_lossy(ir)
        };

        if variant.0 == "RFC 3986" || variant.0 == "Legacy" {
            urlencoding::decode(&string).unwrap()
        } else {
            panic!("Invalid variant in FromIR UrlDecode");
        }
    }
}

/*

impl FromIT for crate::common:: {
    fn variants(ir: &[u8]) -> Option<Vec<Variant>> {
        unimplemented!()
    }

    fn encode(ir: &[u8], variant: Variant) -> String {
        unimplemented!()
    }
}

*/


