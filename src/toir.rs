
use std::boxed::Box;
use crate::common::{Variant, IR, Information};

//A trait that defines functions to convert from & str to IR
pub trait ToIT {

    ///Used to test to see whether the input string can be converted to Self and return a set of variants that match the value
    fn identify(value: & str) -> Result<Vec<Variant>, ()>;

    ///Used to convert an IR to Self. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn decode(value: & str, variant: Variant) -> Box<IR>;

    ///Function returns information about the type
    fn info() -> Information;
}

///Numbers from base 2 to base 16 using 0-9 and a-f
impl ToIT for crate::common::Base16 {

    //Todo: If the string is prefixed with 0x, 0o or ob, treat them as hex, octal and binary, respectively. And if it doesn't correctly convert to the specified type, return Err(())
    //Todo: Since dec, hex, bin and oct are the most likely bases (in that order) we reorder the variant list to show this
    fn identify(value: &str) -> Result<Vec<Variant>, ()> {

        let is_valid_base16 = value.as_bytes().iter().fold(true, |acc, &x| {
            acc && ((x >= '0' as u8 && x <= '9' as u8) || (x >= 'a' as u8 && x <= 'f' as u8) || (x >= 'A' as u8 && x <= 'F' as u8))
        });

        //If the number does not convert to Hexadecimal, it cannot be valid base 16 or lower
        if !is_valid_base16 {
            return Err(())
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

            Ok(variants)

        }

    }

    fn decode(value: &str, variant: Variant) -> Box<IR> {
        let base = Self::get_base(&variant);

        //Convert ascii bytes 0-9a-fA-F into digits
        let input: Vec<_> = value.as_bytes().iter().rev().map(|&byte| Self::ascii_to_num(byte)).collect();

        convert_base::Convert::new(base, 256).convert::<u8, u8>(& input).into_boxed_slice()
    }

    fn info() -> Information {
        Information::new("base 2-16")
    }
}
