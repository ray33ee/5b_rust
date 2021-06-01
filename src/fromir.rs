use crate::common::{Variant, IpV4};
use std::env::var;
use half::f16;
use std::convert::TryInto;
use std::str::{from_utf8, from_utf8_unchecked};
use std::net::{SocketAddrV4, SocketAddrV6};

//A trait that defines functions to convert from IR to T
pub trait FromIT {

    /// Used to identify all the possible variants that the IR can represent
    fn variants(ir: & [u8]) -> Vec<Variant>;

    ///Used to convert an IR to a String. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn encode(ir: & [u8], variant: Variant) -> String;
}

//ToDo: Add options to show number as signed/unsigned 8, 16, 32, 64, 128-bit numbers (for Variant("10") only)
impl FromIT for crate::common::Base2_16 {
    fn variants(_ir: & [u8]) -> Vec<Variant> {
        //Any set of bytes can be converted to base 2-16
        vec![Variant("2"), Variant("3"), Variant("4"), Variant("5"), Variant("6"), Variant("7"),
             Variant("8"), Variant("9"),
             Variant("10"), Variant("11"), Variant("12"), Variant("13"), Variant("14"), Variant("15"),
             Variant("16")]
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

//Todo: Allow customisation of endianness via Options.
impl FromIT for crate::common::FixedFloat {
    fn variants(ir: & [u8]) -> Vec<Variant> {
        let len = ir.as_ref().len();

        match len {
            2 => vec![Variant("16")],
            4 => vec![Variant("32")],
            8 => vec![Variant("64")],
            _ => vec![],
        }

    }

    fn encode(ir: & [u8], variant: Variant) -> String {
        let float = match variant.0 {
            "16" => f16::from_le_bytes(ir.as_ref().try_into().unwrap()).to_f64(),
            "32" => f32::from_le_bytes(ir.as_ref().try_into().unwrap()) as f64,
            "64" => f64::from_le_bytes(ir.as_ref().try_into().unwrap()),
            _ => panic!("Invalid variant in FromIT FixedFloat")
        };

        float.to_string()
    }
}

impl FromIT for crate::common::FixedInt {
    fn variants(ir: & [u8]) -> Vec<Variant> {
        let len = ir.as_ref().len();

        match len {
            1 => vec![Variant("u8"), Variant("i8")],
            2 => vec![Variant("u16"), Variant("i16")],
            4 => vec![Variant("u32"), Variant("i32")],
            8 => vec![Variant("u64"), Variant("i64")],
            16 => vec![Variant("u128"), Variant("i128")],
            _ => vec![],
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

//Todo: Allow customisation of timezone, endianness, etc. via Options
impl FromIT for crate::common::DateTime {
    fn variants(ir: & [u8]) -> Vec<Variant> {
        let len = ir.as_ref().len();

        match len {
            4 => vec![Variant("32 rfc2822"), Variant("32 rfc3339")],
            8 => {

                //Not all combinations of 64-bits result in a valid date, so we

                let timestamp = i64::from_le_bytes(ir.as_ref().try_into().unwrap());

                if chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0).is_some() {
                    vec![Variant("64 rfc2822"), Variant("64 rfc3339")]
                } else {
                    vec![]
                }
            },
            _ => vec![],
        }
    }

    fn encode(ir: & [u8], variant: Variant) -> String {

        let timestamp = match &(variant.0)[0..2] {
            "64" => i64::from_le_bytes(ir.as_ref().try_into().unwrap()),
            "32" => i32::from_le_bytes(ir.as_ref().try_into().unwrap()) as i64,
            _ => panic!("Invalid variant (width) in FromIT DateTime"),
        };

        let datetime = chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(timestamp, 0), chrono::Utc);

        match &(variant.0)[3..10] {
            "rfc2822" => datetime.to_rfc2822(),
            "rfc3339" => datetime.to_rfc3339(),
            "custom " => datetime.format(&(variant.0)[10..]).to_string(),
            _ => panic!("Invalid variant (format) in FromIT DateTime")
        }

    }
}

impl FromIT for crate::common::Unicode8 {
    fn variants(ir: &[u8]) -> Vec<Variant> {
        if from_utf8(ir).is_ok() {
            vec![Variant("")]
        } else {
            vec![]
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

impl FromIT for crate::common::IpV4 {
    fn variants(ir: &[u8]) -> Vec<Variant> {
        let len = ir.as_ref().len();

        match len {
            4 => vec![Variant("without port")],
            6 => vec![Variant("with port")],
            _ => vec![],
        }
    }

    fn encode(ir: &[u8], variant: Variant) -> String {

        let ip = std::net::Ipv4Addr::from(u32::from_be_bytes((&ir[0..4]).try_into().unwrap()));

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

impl FromIT for crate::common::IpV6 {
    fn variants(ir: &[u8]) -> Vec<Variant> {
        let len = ir.as_ref().len();

        match len {
            16 => vec![Variant("without port")],
            18 => vec![Variant("with port")],
            _ => vec![],
        }
    }

    fn encode(ir: &[u8], variant: Variant) -> String {

        let ip = std::net::Ipv6Addr::from(u128::from_be_bytes((&ir[0..16]).try_into().unwrap()));

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

/*

impl FromIT for crate::common:: {
    fn variants(ir: &[u8]) -> Vec<Variant> {
        unimplemented!()
    }

    fn encode(ir: &[u8], variant: Variant) -> String {
        unimplemented!()
    }
}

*/


