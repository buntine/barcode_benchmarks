extern crate barcoders;

use barcoders::sym::code128::*;
use barcoders::generators::ascii::*;

fn main() {
    let barcode = Code128::new("ƁBARBY".to_owned()).unwrap();
    let encoded = barcode.encode();
    let ascii = ASCII::new();

    println!("{}", ascii.generate(&encoded[..]).unwrap());
}
