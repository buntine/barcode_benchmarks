extern crate barcoders;

use barcoders::sym::code128::*;
use barcoders::generators::ascii::*;

fn main() {
    println!("Start");
    for _ in 0..100000 {
        let barcode = Code128::new("ƁBARBY".to_owned()).unwrap();
        let encoded = barcode.encode();
        let ascii = ASCII::new();
        ascii.generate(&encoded[..]).unwrap();
    }
    println!("Stop");
}
