mod common;
mod fromir;
mod toir;

use fromir::FromIT;
use toir::ToIR;
use crate::common::Variant;

fn main() {
    println!("Hello, world!");

    let data = common::Base2_16::decode("60B2959D", Variant("16")).to_vec();

    println!("data: {:?}", u32::encode(&data, Variant("")));

    let result = common::DateTime::encode(&data, Variant("32"));

    println!("number: {:?}", result);


}
