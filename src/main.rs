extern crate lazy_static;

mod common;
mod fromir;
mod toir;
mod escape;

use fromir::FromIT;
use toir::ToIR;
use crate::common::{Variant, FixedInt, FixedFloat, Base2_16, DateTime, Unicode8, IpV4, IpV6, Base91, Base64, Base85, ByteList, UUID, EscapedString, UrlEncode, UrlDecode};

use colour::{blue, yellow};

fn read_without_newline() -> String {
    let mut string = String::new();

    std::io::stdin().read_line(& mut string).expect("Failed to read from stdin");

    if string.bytes().last().unwrap() == 10 {
        string.remove(string.len() - 1);
    }

    if string.bytes().last().unwrap() == 13 {
        string.remove(string.len() - 1);
    }

    string

}

fn main() {

    let to_ir: [(fn(& str) -> Option<Vec<Variant>>, & str, fn(& str, Variant) -> Vec<u8>); 13] = [
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

    let from_ir: [(fn(& [u8]) -> Option<Vec<Variant>>, & str, fn(& [u8], Variant) -> String); 15] = [
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
        (UrlEncode::variants, "Encoded URL", UrlEncode::encode),
        (UrlDecode::variants, "Decoded URL", UrlDecode::encode),
    ];



    println!("Please enter the input string:");

    let input = read_without_newline();

    let mut option_map = Vec::new();

    println!("***************");
    println!("Possible types:");
    println!("***************");

    for (identifier, name, decoder) in to_ir {
        let optional_variants = (identifier)(&input);

        if let Some(variants) = optional_variants {
            println!("{}", name);

            for variant in variants {
                blue!("    {}", option_map.len());
                println!("     {}", variant.0);

                option_map.push((variant, decoder));
            }
        }
    }

    println!("Please enter the index of the possible format you would like to use:");

    let option = read_without_newline();

    match option.parse::<usize>() {
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
                let optional_variants = (variants_function)(&ir);

                if let Some(variants) = optional_variants {
                    println!("{}", name);

                    for variant in variants {
                        yellow!("    {}", variant.0);
                        println!("    {}", (encoder)(&ir, variant.clone()))
                    }
                }
            }
        }
        Err(_) => {
            panic!("Invalid number '{:?}'", option.as_bytes());
        }
    }


}
