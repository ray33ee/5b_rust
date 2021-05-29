use crate::common::{IR, Variant};
use std::env::var;
use half::f16;
use std::convert::TryInto;

//A trait that defines functions to convert from IR to T
pub trait FromIT {

    ///Used to ensure that a given IR can actually be converted to the required type as a particular variant, since this is not always possible (the number 256 cannot be converted to u8)
    fn identify<I: AsRef<IR>>(ir: I, variant: Variant) -> bool;

    ///Used to convert an IR to Self. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> String;
}


//Todo: Allow customisation of endianness.
impl FromIT for u16 {
    fn identify<I: AsRef<IR>>(ir: I, _variant: Variant) -> bool {
        ir.as_ref().len() == 2
    }

    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> String {
        u16::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string()
    }
}

//Todo: Allow customisation of endianness.
impl FromIT for u32 {
    fn identify<I: AsRef<IR>>(ir: I, _variant: Variant) -> bool {
        ir.as_ref().len() == 4
    }

    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> String {
        u32::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string()
    }
}


//Todo: Allow customisation of endianness.
impl FromIT for u64 {
    fn identify<I: AsRef<IR>>(ir: I, _variant: Variant) -> bool {
        ir.as_ref().len() == 8
    }

    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> String {
        u64::from_le_bytes(ir.as_ref().try_into().unwrap()).to_string()
    }
}

impl FromIT for crate::common::Base2_16 {
    fn identify<I: AsRef<IR>>(_ir: I, _variant: Variant) -> bool {
        //Any set of bytes can be converted to base 2-16
        true
    }

    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> String {
        let base = Self::get_base(&variant);

        let mut base_n_list = convert_base::Convert::new(256, base).convert::<u8, u8>(ir.as_ref());

        let mut string = String::new();

        for byte in base_n_list.iter_mut() {
            string.insert(0, Self::num_to_ascii(*byte, true) as char)
        }

        string
    }
}

//Todo: Allow customisation of endianness.
impl FromIT for crate::common::FixedFloat {
    fn identify<I: AsRef<IR>>(ir: I, _variant: Variant) -> bool {
        let len = ir.as_ref().len();

        len == 2 || len == 4 || len == 8
    }

    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> String {
        let float = match variant.0 {
            "16" => f16::from_be_bytes(ir.as_ref().try_into().unwrap()).to_f64(),
            "32" => f32::from_be_bytes(ir.as_ref().try_into().unwrap()) as f64,
            "64" => f64::from_be_bytes(ir.as_ref().try_into().unwrap()),
            _ => panic!("Invalid variant in FromIT FixedFloat")
        };

        float.to_string()
    }
}

//Todo: Allow customisation of timezone, endianness, etc.
impl FromIT for crate::common::DateTime {
    fn identify<I: AsRef<IR>>(ir: I, _variant: Variant) -> bool {
        let len = ir.as_ref().len();

        len == 4 || len == 8
    }

    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> String {
        let timestamp = match variant.0 {
            "64" => i64::from_le_bytes(ir.as_ref().try_into().unwrap()),
            "32" => i32::from_le_bytes(ir.as_ref().try_into().unwrap()) as i64,
            _ => panic!("Invalid variant in FromIT DateTime"),
        };

        let datetime = chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(timestamp, 0), chrono::Utc);

        datetime.to_rfc2822()
    }
}
