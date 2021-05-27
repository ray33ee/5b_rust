
use std::boxed::Box;
use crate::common::{SelectVariant, Variant, IR, Information};
use hex::FromHexError;

//A trait that defines functions to convert from & str to IR
pub trait ToIT {

    ///Used to test to see whether the input string can be converted to Self and return a set of variants that match the value
    fn identify(value: & str) -> Result<Vec<Variant>, ()>;

    ///Used to convert an IR to Self. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn decode(value: & str, variant: Variant) -> Box<IR>;

    ///Function returns information about the type
    fn info() -> Information;
}

///Hexadecimal numbers
impl ToIT for crate::common::Hex {
    fn identify(value: &str) -> Result<Vec<Variant>, ()> {
        match hex::decode(value.as_bytes()) {
            Ok(_) => {Ok(vec![Variant("")])}
            Err(_) => {Err(())}
        }

    }

    fn decode(value: &str, _variant: Variant) -> Box<IR> {
        hex::decode(value.as_bytes()).unwrap().into_boxed_slice()
    }

    fn info() -> Information {
        Information::new("hex")
    }
}