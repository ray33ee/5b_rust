mod common;
mod fromir;
mod toir;

use fromir::FromIT;
use toir::ToIR;
use crate::common::{Variant, FixedInt, FixedFloat, Base2_16, DateTime, NegativeFixedInt, Unicode8, IpV4, IpV6};
use chrono::format::Numeric::Day;
use std::env::var;

fn main() {
    let to_ir: [(fn(& str) -> Vec<Variant>, & str, fn(& str, Variant) -> Vec<u8>); 7] = [
        (IpV4::identify, "Ipv4 address", IpV4::decode),
        (IpV6::identify, "Ipv6 address", IpV6::decode),
        (DateTime::identify, "Unix date and time", DateTime::decode),
        (FixedFloat::identify, "Float", FixedFloat::decode),
        (FixedInt::identify, "Primitive integers", FixedInt::decode),
        (Base2_16::identify, "Base 2-16 number", Base2_16::decode),
        (Unicode8::identify, "Unicode 8 string", Unicode8::decode),
    ];

    let from_ir: [(fn(& [u8]) -> Vec<Variant>, & str, fn(& [u8], Variant) -> String); 7] = [
        (IpV4::variants, "Ipv4 address", IpV4::encode),
        (IpV6::variants, "Ipv6 address", IpV6::encode),
        (DateTime::variants, "Unix date and time", DateTime::encode),
        (FixedFloat::variants, "Floats", FixedFloat::encode),
        (FixedInt::variants, "Primitive integers", FixedInt::encode),
        (Base2_16::variants, "Base 2-16 numbers", Base2_16::encode),
        (Unicode8::variants, "Unicode 8 string", Unicode8::encode),
    ];

    let input = "-1";

    let mut option_map = Vec::new();

    println!("***************");
    println!("Possible types:");
    println!("***************");

    for (identifier, name, decoder) in to_ir {
        let variants = (identifier)(input);

        if variants.len() > 0 {
            println!("{}", name);

            for variant in variants {
                println!("    {}     {}", option_map.len(), variant.0);

                option_map.push((variant, decoder));
            }
        }
    }

    let option = 6;

    let (variant, decoder) = option_map[option].clone();

    let ir = (decoder)(input, variant);

    println!("**************");
    println!("Other formats:");
    println!("**************");

    for (variants_function, name, encoder) in from_ir {



        let variants = (variants_function)(&ir);

        if variants.len() > 0 {
            println!("{}", name);

            for variant in variants {
                println!("    {}    {}", variant.0, (encoder)(&ir, variant.clone()))
            }
        }
    }


}
