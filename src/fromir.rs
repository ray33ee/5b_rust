use crate::common::{IR, Variant};

//A trait that defines functions to convert from IR to T
pub trait FromIT {

    ///Used to ensure that a given IR can actually be converted to the required type as a particular variant, since this is not always possible (the number 256 cannot be converted to u8)
    fn identify<I: AsRef<IR>>(ir: I, variant: Variant) -> bool;

    ///Used to convert an IR to Self. NOTE: this function does NOT check to make sure that the IR can be converted, and will panic if the conversion fails
    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> Self;
}

impl FromIT for crate::common::Base16 {
    fn identify<I: AsRef<IR>>(_ir: I, _variant: Variant) -> bool {
        //Any set of bytes can be converted to base 2-16
        true
    }

    fn encode<I: AsRef<IR>>(ir: I, variant: Variant) -> Self {
        let base = Self::get_base(&variant);

        let mut base_n_list = convert_base::Convert::new(256, base).convert::<u8, u8>(ir.as_ref());

        let mut string = String::new();

        for byte in base_n_list.iter_mut() {
            string.insert(0, Self::num_to_ascii(*byte, true) as char)
        }

        Self(string)
    }
}