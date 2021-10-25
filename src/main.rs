/* Declare so we get linting */
mod decode;
mod arm7tdmi;
mod exec;
mod bus;

pub use decode::*;
pub use arm7tdmi::*;
pub use exec::*;
pub use bus::*;

fn main() {
    println!("Hello, world!");
}