use crate::common::{IR, Variant};

//A trait that defines functions to convert from IR to T
pub trait FromIT {

    ///Used to ensure that a given IR can actually be converted to the required type as a particular variant, since this is not always possible (the number 256 cannot be converted to u8)
    fn identify<I: AsRef<IR>>(ir: I, variant: Variant) -> bool;

    ///Used to convert an IR to Self. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> Self;
}


impl FromIT for crate::common::Hex {
    fn identify<I: AsRef<IR>>(ir: I, _variant: Variant) -> bool {
        true
    }

    fn encode<I: AsRef<IR>>(ir: I, _variant: Variant) -> Self {
        Self(hex::encode(ir))
    }
}