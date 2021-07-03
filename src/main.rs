extern crate lazy_static;

mod common;
mod fromir;
mod toir;
mod escape;

use fromir::FromIR;
use toir::ToIR;
use crate::common::{Variant, FixedInt, FixedFloat, Base2_16, DateTime, Unicode8, IpV4, IpV6, Base91, Base64, Base85, ByteList, UUID, EscapedString, UrlEncode, UrlDecode, UnicodeNames};

use colour::{blue, yellow, green, magenta};
use crate::fromir::Endianness;
use std::io::SeekFrom::End;

use ansi_term::{Style, Colour, ANSIGenericString};
use std::str::{from_utf8_unchecked, from_utf8};


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

    let to_ir: Vec<(fn(& str) -> Option<Vec<Variant>>, & str, fn(& str, Variant) -> Vec<u8>)> = vec![
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
        (EscapedString::identify, "Escaped sequence", EscapedString::decode),
        (UnicodeNames::identify, "Unicode character names", UnicodeNames::decode),
        (common::Colour::identify, "HTML colour", common::Colour::decode),
    ];

    let from_ir: Vec<(fn(& [u8]) -> Option<Vec<Variant>>, & str, fn(& [u8], Variant) -> ANSIGenericString<str>, fn() -> Endianness)> = vec![
        (IpV4::variants, "Ipv4 address", IpV4::encode, IpV4::endianness),
        (IpV6::variants, "Ipv6 address", IpV6::encode, IpV6::endianness),
        (DateTime::variants, "Unix time", DateTime::encode, DateTime::endianness),
        (FixedFloat::variants, "Floats", FixedFloat::encode, FixedFloat::endianness),
        (UUID::variants, "UUID", UUID::encode, UUID::endianness),
        (FixedInt::variants, "Primitive integers", FixedInt::encode, FixedInt::endianness),
        (Base2_16::variants, "Base 2-16 numbers", Base2_16::encode, Base2_16::endianness),
        (Base64::variants, "Base64 data", Base64::encode, Base64::endianness),
        (Base85::variants, "Base85 data", Base85::encode, Base85::endianness),
        (Base91::variants, "Base91 data", Base91::encode, Base91::endianness),
        (ByteList::variants, "Byte list", ByteList::encode, ByteList::endianness),
        (Unicode8::variants, "Unicode 8 string", Unicode8::encode, Unicode8::endianness),
        (EscapedString::variants, "Escaped sequence", EscapedString::encode, EscapedString::endianness),
        (UrlEncode::variants, "Encoded URL", UrlEncode::encode, UrlEncode::endianness),
        (UrlDecode::variants, "Decoded URL", UrlDecode::encode, UrlDecode::endianness),
        (common::Colour::variants, "Colour", common::Colour::encode, common::Colour::endianness),
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

            let reverse: Vec<_> = ir.iter().rev().map(|x| *x).collect();

            println!("**************");
            println!("Other formats:");
            println!("**************");

            for (variants_function, name, encoder, endianness) in from_ir {

                let endianness = endianness();

                let optional_variants = (variants_function)(&ir);

                if let Some(variants) = optional_variants {
                    green!("{}", name);
                    println!();

                    for variant in variants {

                        yellow!("    {}", variant.0);

                        match endianness {
                            Endianness::Default => {
                                println!("    {}", (encoder)(&ir, variant.clone()));
                            },
                            Endianness::Dual => {
                                print!("    {} ", (encoder)(&ir, variant.clone()));
                                magenta!("(");
                                print!("{}", (encoder)(&reverse, variant.clone()));
                                magenta!(")");
                                println!();
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            panic!("Invalid number '{:?}'", option.as_bytes());
        }
    }


}
