use std::str::{Chars, CharIndices, from_boxed_utf8_unchecked, from_utf8_unchecked};

#[derive(Debug, Clone)]
enum Escapes {
    Literal(u8), //Literal, printable, non-escaped character
    Hex(u8),
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
                let upper = crate::common::Base2_16::ascii_to_num(string[2]);
                let lower = crate::common::Base2_16::ascii_to_num(string[3]);

                let byte = lower + upper * 16;

                Self::Hex(byte)
            } else {
                panic!("Invalid hex number");
            }
        } else {
            let byte = string[0];
            Self::Literal(byte)
        }
    }
}

impl Escapes {
    fn to_byte(& self) -> u8 {
        match &self {
            Escapes::Literal(byte) => {*byte}
            Escapes::Hex(byte) => {*byte}
            Escapes::Backslash => {'\\' as u8}
            Escapes::SingleQuote => {'\'' as u8}
            Escapes::DoubleQuote => {'\"' as u8}
            Escapes::Bell => {7}
            Escapes::Backspace => {8}
            Escapes::FormFeed => {12}
            Escapes::LineFeed => {'\n' as u8}
            Escapes::CarriageReturn => {'\r' as u8}
            Escapes::HorizontalTab => {'\t' as u8}
            Escapes::VerticalTab => {11}
        }
    }
}

struct EscapedStringIterator<'t> {
    remaining: & 't [u8],
}

impl<'t> EscapedStringIterator<'t> {
    pub fn new(string: & 't str) -> Self {
        Self {
            remaining: string.as_bytes(),
        }
    }

    pub fn is_valid_escaped_string(string: & 't str) -> bool {
        //Make sure string doesn't contain any unicode characters
        for byte in string.as_bytes() {
            if *byte > 128 {
                return false
            }
        }
        //Make sure any \x is followed by exactly 2 hex characters


        true
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

                if (&self.remaining[2..]).len() < 2 {
                    self.remaining = &self.remaining[0..0];
                    return Some(Err(()))
                }

                if !crate::common::Base2_16::is_hex(self.remaining[2]) || !crate::common::Base2_16::is_hex(self.remaining[3]){
                    self.remaining = &self.remaining[0..0];
                    return Some(Err(()))
                }

                let slice = &self.remaining[0..4];
                self.remaining = &self.remaining[4..];
                slice

            } else {
                let slice = &self.remaining[0..1];
                self.remaining = &self.remaining[1..];
                slice
            }
        } else {
            let slice = &self.remaining[0..1];
            self.remaining = &self.remaining[1..];
            slice
        };

        unsafe {
            Some(Ok(Escapes::from(slice)))
        }
    }
}

pub struct EscapedString;

impl EscapedString {
    pub fn ascii_to_bytes(string: &str) -> Result<Vec<u8>, ()> {
        let mut bytes = Vec::new();

        for escaped in EscapedStringIterator::new(string) {
            bytes.push((escaped?).to_byte());
        }

        Ok(bytes)
    }

    pub fn bytes_to_ascii(bytes: & [u8]) -> String {
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
