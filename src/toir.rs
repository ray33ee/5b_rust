
use std::boxed::Box;
use crate::common::{Variant, IR, Information, DateTime};
use std::str::FromStr;
use half::f16;
use chrono::format::Numeric::Day;
use std::env::var;

//A trait that defines functions to convert from & str to IR
pub trait ToIR {

    ///Used to test to see whether the input string can be converted to Self and return a set of variants that match the value
    fn identify(value: & str) -> Vec<Variant>;

    ///Used to convert an IR to Self. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn decode(value: & str, variant: Variant) -> Box<IR>;

    ///Function returns information about the type
    fn info() -> Information;
}

///Numbers from base 2 to base 16 using 0-9 and a-f
impl ToIR for crate::common::Base2_16 {

    //Todo: If the string is prefixed with 0x, 0o or ob, treat them as hex, octal and binary, respectively. And if it doesn't correctly convert to the specified type, return Err(())
    //Todo: Since dec, hex, bin and oct are the most likely bases (in that order) we reorder the variant list to show this
    fn identify(value: &str) -> Vec<Variant> {

        let is_valid_base16 = value.as_bytes().iter().fold(true, |acc, &x| {
            acc && ((x >= '0' as u8 && x <= '9' as u8) || (x >= 'a' as u8 && x <= 'f' as u8) || (x >= 'A' as u8 && x <= 'F' as u8))
        });

        //If the number does not convert to Hexadecimal, it cannot be valid base 16 or lower
        if !is_valid_base16 {
            return Vec::new();
        } else { //Get a list of all possible bases, based on the largest digit. (for example if the largest digit is 1, then it could be base 2 or more. If the largest digit is 7,, it could be base 8 or more)

            let largest = value
                .as_bytes()
                .iter()
                .map(|&x| Self::ascii_to_num(x))
                .max()
                .unwrap_or(0);

            let n = if largest == 0 as u8 { 0 } else {largest - 1};

            let possible_bases= [Variant("2"), Variant("3"), Variant("4"), Variant("5"), Variant("6"), Variant("7"), Variant("8"), Variant("9"), Variant("10"), Variant("11"), Variant("12"), Variant("13"), Variant("14"), Variant("15"), Variant("16")];

            let variants = Vec::from(&possible_bases[n as usize..]);

            variants

        }

    }

    fn decode(value: &str, variant: Variant) -> Box<IR> {
        let base = Self::get_base(&variant);

        //Convert ascii bytes 0-9a-fA-F into digits
        let input: Vec<_> = value.as_bytes().iter()
            .rev()
            .map(|&byte| Self::ascii_to_num(byte)).collect();

        convert_base::Convert::new(base, 256).convert::<u8, u8>(& input).into_boxed_slice()
    }

    fn info() -> Information {
        Information::new("base 2-16")
    }
}

///All common fixed precision floating point numbers (16, 32 and 64-bit)
impl ToIR for crate::common::FixedFloat {
    fn identify(value: &str) -> Vec<Variant> {
        //If conversion to f64 succeeds, then conversion to f32 and f16 will also succeed
        if f64::from_str(value).is_ok() {
            vec![Variant("32"), Variant("16"), Variant("64"), ]
        } else {
            vec![]
        }
    }

    fn decode(value: &str, variant: Variant) -> Box<IR> {

        let number = f64::from_str(value).unwrap();

        let bytes = match variant.0 {
            "16" => Vec::from(f16::from_f64(number).to_le_bytes()),
            "32" => Vec::from((number as f32).to_le_bytes()),
            "64" => Vec::from(number.to_le_bytes()),
            _ => panic!("Invalid variant in FixedFloat")
        };

        bytes.into_boxed_slice()
    }

    fn info() -> Information {
        Information::new("fixed precision float")
    }
}

impl ToIR for crate::common::DateTime {
    fn identify(value: &str) -> Vec<Variant> {

        let mut variants = Vec::new();

        if chrono::DateTime::parse_from_rfc2822(value).is_ok() {
            variants.push(Variant("rfc2822-32"));
            variants.push(Variant("rfc2822-64"));
        }

        if chrono::DateTime::parse_from_rfc3339(value).is_ok() {
            variants.push(Variant("rfc3339-32"));
            variants.push(Variant("rfc3339-64"));
        }

        variants

    }

    fn decode(value: &str, variant: Variant) -> Box<IR> {
        let datetime = match &(variant.0)[0..7] {
            "rfc2822" => chrono::DateTime::parse_from_rfc2822(value),
            "rfc3339" => chrono::DateTime::parse_from_rfc3339(value),
            _ => panic!("Invalid rfc variant in DateTime ToIR")
        }.unwrap();

        let timestamp = datetime.timestamp();

        match &(variant.0)[8..10] {
            "32" => Vec::from((timestamp as i32).to_le_bytes()).into_boxed_slice(),
            "64" => Vec::from(timestamp.to_le_bytes()).into_boxed_slice(),
            _ => panic!("Invalid size variant in DateTime ToIR"),
        }
    }

    fn info() -> Information {
        Information::new("unix date time")
    }
}
