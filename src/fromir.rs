use crate::common::{Variant, Colour};
use half::f16;
use std::convert::TryInto;
use std::str::{from_utf8, from_utf8_unchecked};
use std::net::{SocketAddrV4, SocketAddrV6};
use std::io::Write;
use std::borrow::Cow;

use ansi_term::{ANSIGenericString, Style};
use std::env::var;

pub enum Endianness {
    Default,
    Dual,
}

//A trait that defines functions to convert from IR to T
pub trait FromIR {

    /// Used to identify all the possible variants that the IR can represent
    fn variants(ir: & [u8]) -> Option<Vec<Variant>>;

    ///Used to convert an IR to a String. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn encode(ir: & [u8], variant: Variant) -> ANSIGenericString<str>;

    fn endianness() -> Endianness;
}

impl FromIR for crate::common::Base2_16 {
    fn variants(_ir: & [u8]) -> Option<Vec<Variant>> {
        //Any set of bytes can be converted to base 2-16
        Some(vec![Variant("Base 2"), //Variant("Base 3"), Variant("Base 4"), Variant("Base 5"), Variant("Base 6"), Variant("Base 7"),
             Variant("Base 8"), //Variant("Base 9"),
             Variant("Base 10"), //Variant("Base 11"), Variant("Base 12"), Variant("Base 13"), Variant("Base 14"), Variant("Base 15"),
             Variant("Base 16")])
    }

    fn encode(ir: & [u8], variant: Variant) -> ANSIGenericString<str> {
        let base = Self::get_base(&variant);

        let mut base_n_list = convert_base::Convert::new(256, base).convert::<u8, u8>(ir.as_ref());

        let mut string = String::new();

        for byte in base_n_list.iter_mut() {
            string.insert(0, Self::num_to_ascii(*byte, true) as char)
        }

        Style::default().paint(string)
    }

    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl FromIR for crate::common::FixedFloat {
    fn variants(ir: & [u8]) -> Option<Vec<Variant>> {
        let len = ir.as_ref().len();

        match len {
            2 => Some(vec![Variant("16-bit"), Variant("16-bit mantissa/exponent")]),
            4 => Some(vec![Variant("32-bit"), Variant("32-bit mantissa/exponent")]),
            8 => Some(vec![Variant("64-bit"), Variant("64-bit mantissa/exponent")]),
            _ => None,
        }

    }

    fn encode(ir: & [u8], variant: Variant) -> ANSIGenericString<str> {

        if variant.0.len() == 24 {
            if &variant.0[7..] == "mantissa/exponent" {
                return Style::default().paint(Self::to_mes_string(ir));
            }
        }

        Style::default().paint(match &variant.0[0..6] {
            "16-bit" => f16::from_le_bytes(ir.as_ref().try_into().unwrap()).to_f64().to_string(),
            "32-bit" => (f32::from_le_bytes(ir.as_ref().try_into().unwrap()) as f64).to_string(),
            "64-bit" => f64::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string(),
            _ => panic!("Invalid variant in FromIT FixedFloat")
        })
    }

    fn endianness() -> Endianness {
        Endianness::Dual
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

    fn encode(ir: & [u8], variant: Variant) -> ANSIGenericString<str> {
        Style::default().paint(match variant.0 {
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
        })
    }

    fn endianness() -> Endianness {
        Endianness::Dual
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

    fn encode(ir: & [u8], variant: Variant) -> ANSIGenericString<str> {

        let timestamp = match &(variant.0)[0..2] {
            "64" => i64::from_le_bytes(ir.as_ref().try_into().unwrap()),
            "32" => i32::from_le_bytes(ir.as_ref().try_into().unwrap()) as i64,
            _ => panic!("Invalid variant (width) in FromIT DateTime"),
        };

        let datetime = chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(timestamp, 0), chrono::Utc);

        Style::default().paint(match &(variant.0)[7..14] {
            "rfc2822" => datetime.to_rfc2822(),
            "rfc3339" => datetime.to_rfc3339(),
            "custom " => datetime.format(&(variant.0)[14..]).to_string(),
            _ => panic!("Invalid variant (format) in FromIT DateTime")
        })

    }

    fn endianness() -> Endianness {
        Endianness::Dual
    }
}

impl FromIR for crate::common::Unicode8 {
    fn variants(ir: &[u8]) -> Option<Vec<Variant>> {
        if from_utf8(ir).is_ok() {
            for character in unsafe {from_utf8_unchecked(ir).chars()} {
                if unicode_names2::name(character).is_none() {
                    return Some(vec![Variant("Literal String")])
                }
            }

            Some(vec![Variant("Literal String"), Variant("Unicode Names")])

        } else {
            None
        }
    }

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {

        if variant.0 == "Literal String" {
            unsafe {
                Style::default().paint(String::from(from_utf8_unchecked(ir)))
            }
        } else if variant.0 == "Unicode Names" {
            let mut string = String::from("[");

            let unicode = unsafe {
                from_utf8_unchecked(ir)
            };

            for character in unicode.chars() {

                string.push_str(&unicode_names2::name(character).unwrap().to_string());
                string.push_str(" '");
                string.push(character);
                string.push_str("', ")
            }

            Style::default().paint(string)
        } else {
            panic!("Invalid variant in FromIR Unicode8");
        }
    }

    fn endianness() -> Endianness {
        Endianness::Default
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

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {

        let ip = std::net::Ipv4Addr::from(u32::from_le_bytes((&ir[0..4]).try_into().unwrap()));

        Style::default().paint(match variant.0 {
            "with port" => {
                let port = u16::from_le_bytes((&ir[4..6]).try_into().unwrap());

                SocketAddrV4::new(ip, port).to_string()
            },
            "without port" => {

                ip.to_string()
            }
            _ => panic!("Invalid variant in FromIT IpV4"),
        })
    }

    fn endianness() -> Endianness {
        Endianness::Default
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

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {

        let ip = std::net::Ipv6Addr::from(u128::from_le_bytes((&ir[0..16]).try_into().unwrap()));

        Style::default().paint(match variant.0 {
            "with port" => {
                let port = u16::from_le_bytes((&ir[16..18]).try_into().unwrap());

                SocketAddrV6::new(ip, port, 0, 0).to_string()
            },
            "without port" => {

                ip.to_string()
            }
            _ => panic!("Invalid variant in FromIT IpV4"),
        })
    }

    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl FromIR for crate::common::Base91 {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn encode(ir: &[u8], _variant: Variant) -> ANSIGenericString<str> {
        unsafe {
            Style::default().paint(String::from_utf8_unchecked(base91::slice_encode(ir)))
        }
    }

    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl FromIR for crate::common::Base85 {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("z85"), Variant("ascii85")])
    }

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {
        Style::default().paint(match variant.0 {
            "z85" => z85::encode(ir),
             "ascii85" => ascii85::encode(ir),
            _ => panic!("Invalid variant in FromIR Base85")
        })
    }

    fn endianness() -> Endianness {
        Endianness::Default
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

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {
        Style::default().paint(match variant.0 {
            "Bcrypt" => base64::encode_config(ir, base64::BCRYPT),
            "BinHex" => base64::encode_config(ir, base64::BINHEX),
            "crypt" => base64::encode_config(ir, base64::CRYPT),
            "IMAP UTF-7" => base64::encode_config(ir, base64::IMAP_MUTF7),
            "Standard" => base64::encode_config(ir, base64::STANDARD),
            "Standard no padding" => base64::encode_config(ir, base64::STANDARD_NO_PAD),
            "URL-safe" => base64::encode_config(ir, base64::STANDARD_NO_PAD),
            "URL-safe no padding" => base64::encode_config(ir, base64::URL_SAFE_NO_PAD),
            _ => panic!("Invalid variant in FromIr Base64"),
        })
    }

    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl FromIR for crate::common::ByteList {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn encode(ir: &[u8], _variant: Variant) -> ANSIGenericString<str> {
        Style::default().paint(format!("({} byte(s)) {:?}", ir.len(), ir))
    }

    fn endianness() -> Endianness {
        Endianness::Default
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

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {
        if variant.0 == "" {
            Style::default().paint(uuid::Uuid::from_slice(ir).unwrap().to_string())
        } else {
            panic!("Invalid variant in FromIT uuid")
        }
    }

    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl FromIR for crate::common::EscapedString {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {
        if variant.0 == "" {
            Style::default().paint(crate::escape::EscapeSequence::encode(ir))
        } else {
            panic!("Invalid variant in FromIR EscapedString");
        }
    }

    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl FromIR for crate::common::UrlEncode {
    fn variants(_ir: &[u8]) -> Option<Vec<Variant>> {
        Some(vec![Variant("")])
    }

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {
        if variant.0 == "" {
            Style::default().paint(urlencoding::encode_binary(ir).to_string())
        } else {
            panic!("Invalid variant in FromIR UrlEncode");
        }
    }

    fn endianness() -> Endianness {
        Endianness::Default
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

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {

        let string = if variant.0 == "RFC 3986" {
            let string = String::from_utf8_lossy(ir);
            Cow::from(string.replace("+", "%20"))
        } else {
            String::from_utf8_lossy(ir)
        };

        if variant.0 == "RFC 3986" || variant.0 == "Legacy" {
            Style::default().paint(urlencoding::decode(&string).unwrap())
        } else {
            panic!("Invalid variant in FromIR UrlDecode");
        }
    }

    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl FromIR for crate::common::Colour {
    fn variants(ir: &[u8]) -> Option<Vec<Variant>> {
        let variants = match ir.len() {
            1 => vec![Variant("8-bit color"), Variant("8-bit greyscale"), Variant("8-bit terminal")],
            2 => vec![Variant("16-bit color")],
            3 => vec![Variant("24-bit color"), Variant("24-bit rgb"), Variant("24-bit hsl")],
            4 => vec![Variant("32-bit color")],
            _ => return None
        };

        Some(variants)
    }

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {
        use ansi_term::Color;

        let colour = match variant.0 {
            "8-bit color" => {
                Colour::_8_to_24(*ir.get(0).unwrap())
            },
            "8-bit greyscale" => { let byte = *ir.get(0).unwrap(); Color::RGB(byte, byte, byte) },
            "8-bit terminal" => { Color::Fixed(*ir.get(0).unwrap()) },
            "16-bit color" => {Colour::_16_to_24(u16::from_le_bytes(ir.try_into().unwrap()))},
            "24-bit color" | "24-bit rgb" | "24-bit hsl" => {
                let red = *ir.get(2).unwrap();
                let green = *ir.get(1).unwrap();
                let blue = *ir.get(0).unwrap();

                Color::RGB(red, green, blue)
            },
            "32-bit color" => {
                let red = *ir.get(2).unwrap();
                let green = *ir.get(1).unwrap();
                let blue = *ir.get(0).unwrap();
                Color::RGB(red, green, blue)
            },
            _ => panic!("Invalid variant in FromIR Colour")
        };

        match variant.0 {
            "24-bit rgb" => {
                if let Color::RGB(r, g, b) = colour {
                    Style::default().paint(format!("rgb({}, {}, {})", r, g, b))
                } else {
                    panic!("Invalid variant in FromIR colour");
                }

            },
            "24-bit hsl" => {
                if let Color::RGB(r, g, b) = colour {

                    let slice = [b, g, r];

                    let hsl_value = hsl::HSL::from_rgb(&slice);

                    Style::default().paint(format!("hsl({}, {}, {})", hsl_value.h, hsl_value.s, hsl_value.l))
                } else {
                    panic!("Invalid variant in FromIR colour");
                }
            },
            _ => Style::default().fg(colour).paint("⬛"),
        }


    }

    fn endianness() -> Endianness {
        Endianness::Dual
    }
}

/*

impl FromIR for crate::common:: {
    fn variants(ir: &[u8]) -> Option<Vec<Variant>> {
        unimplemented!()
    }

    fn encode(ir: &[u8], variant: Variant) -> ANSIGenericString<str> {
        unimplemented!()
    }

    fn endianness() -> Endianness {
        Endianness::Default
    }
}

*/


