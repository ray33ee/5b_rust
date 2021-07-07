


#[derive(Debug, Copy, Clone)]
pub enum Endianness {
    Default,
    Dual,
}


pub trait Endian {
    fn endianness() -> Endianness;
}

impl Endian for crate::UnicodeNames {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::Base2_16 {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::FixedFloat {
    fn endianness() -> Endianness {
        Endianness::Dual
    }
}

impl Endian for crate::common::FixedInt {
    fn endianness() -> Endianness {
        Endianness::Dual
    }
}

impl Endian for crate::common::DateTime {
    fn endianness() -> Endianness {
        Endianness::Dual
    }
}

impl Endian for crate::common::Unicode8 {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::IpV4 {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::IpV6 {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::Base91 {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::Base85 {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::Base64 {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::ByteList {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::UUID {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::EscapedString {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::UrlEncode {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::UrlDecode {
    fn endianness() -> Endianness {
        Endianness::Default
    }
}

impl Endian for crate::common::Colour {
    fn endianness() -> Endianness {
        Endianness::Dual
    }
}