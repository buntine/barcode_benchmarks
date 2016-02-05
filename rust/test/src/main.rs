extern crate barcoders;

use barcoders::sym::code128::*;
use barcoders::generators::image::*;

fn main() {
    println!("Start");
    for _ in 0..5000 {
        let barcode = Code128::new("ƁBARBY".to_owned()).unwrap();
        let png = Image::PNG{height: 100, xdim: 1, rotation: Rotation::Zero};
        let encoded = barcode.encode();

        png.generate(&encoded[..]).unwrap();
    }
    println!("Stop");
}
