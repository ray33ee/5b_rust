#[macro_use]
extern crate lazy_static;

mod common;
mod fromir;
mod toir;
mod escape;

use fromir::FromIT;
use toir::ToIR;
use crate::common::{Variant, FixedInt, FixedFloat, Base2_16, DateTime, Unicode8, IpV4, IpV6, Base91, Base64, Base85, ByteList, UUID, Hash, EscapedString};
use std::num::ParseIntError;


fn main() {

    let to_ir: [(fn(& str) -> Vec<Variant>, & str, fn(& str, Variant) -> Vec<u8>); 13] = [
        (IpV4::identify, "Ipv4 address", IpV4::decode),
        (IpV6::identify, "Ipv6 address", IpV6::decode),
        (DateTime::identify, "Unix time", DateTime::decode),
        (FixedFloat::identify, "Floats", FixedFloat::decode),
        (UUID::identify, "UUID", UUID::decode),
        (FixedInt::identify, "Primitive integers", FixedInt::decode),
        (Base2_16::identify, "Base 2-16 number", Base2_16::decode),
        (Base64::identify, "Base64 data", Base64::decode),
        (Base85::identify, "Base85 data", Base85::decode),
        (Base91::identify, "Base91 data", Base91::decode),
        (Unicode8::identify, "Unicode 8 string", Unicode8::decode),
        (ByteList::identify, "Byte list", ByteList::decode),
        (EscapedString::identify, "Escaped character string", EscapedString::decode),
    ];

    let from_ir: [(fn(& [u8]) -> Vec<Variant>, & str, fn(& [u8], Variant) -> String); 13] = [
        (IpV4::variants, "Ipv4 address", IpV4::encode),
        (IpV6::variants, "Ipv6 address", IpV6::encode),
        (DateTime::variants, "Unix time", DateTime::encode),
        (FixedFloat::variants, "Floats", FixedFloat::encode),
        (UUID::variants, "UUID", UUID::encode),
        (FixedInt::variants, "Primitive integers", FixedInt::encode),
        (Base2_16::variants, "Base 2-16 numbers", Base2_16::encode),
        (Base64::variants, "Base64 data", Base64::encode),
        (Base85::variants, "Base85 data", Base85::encode),
        (Base91::variants, "Base91 data", Base91::encode),
        (ByteList::variants, "Byte list", ByteList::encode),
        (Unicode8::variants, "Unicode 8 string", Unicode8::encode),
        (EscapedString::variants, "Escaped character string", EscapedString::encode),
    ];



    let mut input = String::new();

    println!("Please enter the input string:");

    std::io::stdin().read_line(& mut input).expect("Failed to read from stdin");

    input.remove(input.len()-1);

    let mut option_map = Vec::new();

    println!("***************");
    println!("Possible types:");
    println!("***************");

    for (identifier, name, decoder) in to_ir {
        let variants = (identifier)(&input);

        if variants.len() > 0 {
            println!("{}", name);

            for variant in variants {
                println!("    {}     {}", option_map.len(), variant.0);

                option_map.push((variant, decoder));
            }
        }
    }

    let mut option = String::new();

    println!("Please enter the index of the possible format you would like to use:");

    std::io::stdin().read_line(& mut option).expect("Failed to read from stdin");

    match (&option[0..option.len()-1]).parse::<usize>() {
        Ok(option) => {

            if option >= option_map.len() {
                panic!("Invalid number (must be between 0 and {}, inclusive)", option_map.len()-1);
            }

            let (variant, decoder) = option_map[option].clone();

            let ir = (decoder)(&input, variant);

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
        Err(_) => {

        }
    }


}
