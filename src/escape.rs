use std::str::{from_utf8, from_utf8_unchecked};
use crate::toir::ToIR;
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryInto;

/// escape.rs defines a set of tools for interpreting and creating different types of escape sequences



#[derive(Debug, Clone)]
enum Escapes {
    Literal(u8), //Literal, printable, non-escaped character
    Bytes(Vec<u8>),
    Byte(u8),
    Unicode(char),
    Backslash,
    SingleQuote,
    DoubleQuote,
    Bell,
    Backspace,
    FormFeed,
    LineFeed,
    CarriageReturn,
    HorizontalTab,
    VerticalTab,
}

#[derive(Debug, Clone)]
pub enum Variant {
    C,
    Python,
}

impl<'t> From<& 't [u8]> for Escapes {
    fn from(string: &[u8]) -> Self {

        if string == b"\\\\" {
            Self::Backslash
        } else if string == b"\\\"" {
            Self::DoubleQuote
        } else if string == b"\\\'"  {
            Self::SingleQuote
        } else if string == b"\\a" {
            Self::Bell
        } else if  string == b"\\b" {
            Self::Backspace
        } else if  string == b"\\f" {
            Self::FormFeed
        } else if  string == b"\\n" {
            Self::LineFeed
        } else if  string == b"\\r" {
            Self::CarriageReturn
        } else if  string == b"\\t" {
            Self::HorizontalTab
        } else if  string == b"\\v" {
            Self::VerticalTab
        } else if string.len() > 1  {
            if &string[0..2] == b"\\x" {

                let bytes = crate::Base2_16::decode(from_utf8(&string[2..]).unwrap(), crate::Variant("Base 16"));

                Self::Bytes(bytes)
            } else if &string[0..1] == b"\\" && string[1] >= '0' as u8 && string[1] <= '7' as u8 {
                let bytes = crate::Base2_16::decode(from_utf8(&string[1..]).unwrap(), crate::Variant("Base 8"));

                Self::Byte(bytes[0])

            } else {
                panic!("Invalid escape sequence")
            }
        } else {
            let byte = string[0];
            Self::Literal(byte)
        }
    }
}

impl Escapes {
    fn append_bytes(& self, vector: & mut Vec<u8>) {
        match &self {
            Escapes::Literal(byte) => {vector.push(*byte)}
            Escapes::Bytes(byte) => {vector.extend_from_slice(byte)}
            Escapes::Backslash => {vector.push('\\' as u8)}
            Escapes::SingleQuote => {vector.push('\'' as u8)}
            Escapes::DoubleQuote => {vector.push('\"' as u8)}
            Escapes::Bell => {vector.push(7)}
            Escapes::Backspace => {vector.push(8)}
            Escapes::FormFeed => {vector.push(12)}
            Escapes::LineFeed => {vector.push('\n' as u8)}
            Escapes::CarriageReturn => {vector.push('\r' as u8)}
            Escapes::HorizontalTab => {vector.push('\t' as u8)}
            Escapes::VerticalTab => {vector.push(11)}
            Escapes::Byte(byte) => {vector.push(*byte)}
            Escapes::Unicode(character) => {
                vector.extend_from_slice(character.to_string().as_bytes())
            }
        }
    }
}

struct EscapedStringIterator<'t> {
    remaining: & 't [u8],
    variant: Variant,
}

impl<'t> EscapedStringIterator<'t> {
    pub fn new(string: & 't str, variant: Variant) -> Self {
        Self {
            remaining: string.as_bytes(),
            variant
        }
    }
}

impl<'t> Iterator for EscapedStringIterator<'t> {
    type Item = Result<Escapes, ()>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.remaining.is_empty() {
            return None;
        }

        let slice = if self.remaining[0] == '\\' as u8 {
            let ch = self.remaining[1] as char;
            if ch == '\\' || ch == '\'' || ch == '\"' || ch == 'a' || ch == 'b' || ch == 'f' || ch == 'n' || ch == 'r' || ch == 't' || ch == 'v' {
                let slice = &self.remaining[0..2];
                self.remaining = &self.remaining[2..];
                slice
            } else if ch == 'x' {

                let mut hex_character_count = 0;

                for byte in &self.remaining[2..] {
                    if crate::Base2_16::is_hex(*byte) {
                        hex_character_count += 1;
                    } else {
                        break;
                    }
                }

                let advanceby = match self.variant {
                    Variant::C => {
                        if hex_character_count == 0 {
                            self.remaining = &self.remaining[0..0];
                            return Some(Err(()))
                        }

                        hex_character_count + 2
                    }
                    Variant::Python => {
                        if hex_character_count != 2 {
                            self.remaining = &self.remaining[0..0];
                            return Some(Err(()))
                        }

                        4
                    }
                };

                let slice = &self.remaining[0..advanceby];
                self.remaining = &self.remaining[advanceby..];
                slice

            } else if ch == 'N' {
                match self.variant {
                    Variant::C => {
                        //C strings do not support named unicode characters
                        self.remaining = &self.remaining[0..0];
                        return Some(Err(()))
                    }
                    Variant::Python => {
                        lazy_static! {
                            static ref UNICODE_NAME: Regex = Regex::new("\\{([^}]*)\\}").unwrap();
                        }


                        let captures = UNICODE_NAME.captures(unsafe {from_utf8_unchecked(self.remaining)});


                        match captures {
                            Some(groups) => {
                                //If the capture matches, then a capture group at 1 will always exist
                                let name = groups.get(1).unwrap();

                                match unicode_names2::character(name.as_str()) {
                                    None => {
                                        self.remaining = &self.remaining[0..0];
                                        return Some(Err(()))
                                    }
                                    Some(character) => {

                                        self.remaining = &self.remaining[name.end()+1..];
                                        return Some(Ok(Escapes::Unicode(character)));
                                    }
                                }
                            },
                            None => {
                                self.remaining = &self.remaining[0..0];
                                return Some(Err(()))
                            }
                        }

                    }
                }
            } else if ch == 'u' {

                let mut hex_character_count = 0;

                for byte in &self.remaining[2..] {
                    if crate::Base2_16::is_hex(*byte) {
                        hex_character_count += 1;
                    } else {
                        break;
                    }
                }

                if hex_character_count != 4 {
                    self.remaining = &self.remaining[0..0];
                    return Some(Err(()))
                }

                let get_byte = |i| {
                    crate::common::Base2_16::ascii_to_num(self.remaining[i]) as u32
                };

                let character: u32 = (((((get_byte(2) << 4) | get_byte(3)) << 4) | get_byte(4)) << 4) | get_byte(5);

                match character.try_into() {
                    Ok(ch) => {
                        self.remaining = &self.remaining[6..];
                        return Some(Ok(Escapes::Unicode(ch)));
                    },
                    Err(_) => {
                        self.remaining = &self.remaining[0..0];
                        return Some(Err(()))
                    }
                }




            } else if ch == 'U' {
                let mut hex_character_count = 0;

                for byte in &self.remaining[2..] {
                    if crate::Base2_16::is_hex(*byte) {
                        hex_character_count += 1;
                    } else {
                        break;
                    }
                }

                if hex_character_count != 8 {
                    self.remaining = &self.remaining[0..0];
                    return Some(Err(()))
                }

                let get_byte = |i| {
                    crate::common::Base2_16::ascii_to_num(self.remaining[i]) as u32
                };

                let character: u32 = (((((get_byte(2) << 4) | get_byte(3)) << 4) | get_byte(4)) << 4) | get_byte(5);

                let character: u32 = character << 16 | (((((get_byte(6) << 4) | get_byte(7)) << 4) | get_byte(8)) << 4) | get_byte(9);

                match character.try_into() {
                    Ok(ch) => {
                        self.remaining = &self.remaining[10..];
                        return Some(Ok(Escapes::Unicode(ch)));
                    },
                    Err(_) => {
                        self.remaining = &self.remaining[0..0];
                        return Some(Err(()))
                    }
                }



            } else if ch >= '0' && ch <= '7' {

                let mut octal_character_count = 0;

                for byte in &self.remaining[1..] {
                    if crate::Base2_16::is_oct(*byte) {
                        octal_character_count += 1;
                    } else {
                        break;
                    }
                }

                octal_character_count = if octal_character_count > 3 {
                    3
                } else {
                    octal_character_count
                };

                let advanceby = octal_character_count+1;

                let slice = &self.remaining[0..advanceby];
                self.remaining = &self.remaining[advanceby..];
                slice

            } else {
                //Invalid escape sequence
                self.remaining = &self.remaining[0..0];
                return Some(Err(()))
            }
        } else {
            let slice = &self.remaining[0..1];
            self.remaining = &self.remaining[1..];
            slice
        };

        Some(Ok(Escapes::from(slice)))
    }
}

pub struct EscapeSequence;

impl EscapeSequence {
    pub fn decode(string: &str, variant: Variant) -> Result<Vec<u8>, ()> {
        let mut bytes = Vec::new();

        for escaped in EscapedStringIterator::new(string, variant) {
            (escaped?).append_bytes(& mut bytes);
        }

        Ok(bytes)
    }

    pub fn encode(bytes: & [u8]) -> String {
        let mut string = String::new();

        for &byte in bytes {
            if byte == '\\' as u8 {
                string.push_str("\\\\");
            } else if byte >= 0x20 && byte <= 0x7E {
                string.push(byte as char)
            } else if byte == '\'' as u8 {
                string.push_str("\\'");
            } else if byte == '\"' as u8 {
                string.push_str("\\");
            } else if byte == 7 {
                string.push_str("\\a");
            } else if byte == 8 {
                string.push_str("\\b");
            } else if byte == 12 {
                string.push_str("\\f");
            } else if byte == '\n' as u8 {
                string.push_str("\\n");
            } else if byte == '\r' as u8 {
                string.push_str("\\r");
            }else if byte == '\t' as u8 {
                string.push_str("\\t");
            }else if byte == 11 {
                string.push_str("\\v");
            } else {

                let upper = crate::common::Base2_16::num_to_ascii(byte >> 4, true) as char;
                let lower = crate::common::Base2_16::num_to_ascii(byte & 0x0F, true) as char;

                string.push_str("\\x");
                string.push(upper);
                string.push(lower);
            }
        }

        string
    }
}
