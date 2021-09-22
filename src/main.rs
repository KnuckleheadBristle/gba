mod proccore;

pub use proccore::*;

fn main() {
    println!("Hello, world!");
    proccore::decode_arm(0x1234);
    proccore::decode_thumb(0b0000011001101010);
}