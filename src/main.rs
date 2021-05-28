mod common;
mod fromir;
mod toir;

use fromir::FromIT;
use toir::ToIT;
use crate::common::Variant;

fn main() {
    println!("Hello, world!");

    let data = common::Base16::decode("ffffffffffffffffff", Variant("16")).to_vec();

    println!("data: {:?}", data);

    let base16 = common::Base16::encode(data, Variant("2"));

    println!("number: {:?}", base16.0);


}
