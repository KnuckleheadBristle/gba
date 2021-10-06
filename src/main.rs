mod decode;
mod arm7tdmi;
mod exec;

pub use decode::*;
pub use arm7tdmi::*;
pub use exec::*;

fn main() {
    println!("Hello, world!");
}