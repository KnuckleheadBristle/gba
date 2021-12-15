/* Declare so we get linting */
mod decode;
mod arm7tdmi;
mod exec;
mod bus;

pub use decode::*;
pub use arm7tdmi::*;
pub use exec::*;
pub use bus::*;

/*
This is here like this so that the rust analyser will actually give me what's
wrong with my code, otherwise I don't get linting. Plus we want to make sure that
cargo knows what to do when building/running etc.
*/

fn main() { //I will probably make this do some thing later, but it will stay like this for now
    println!("Hello, world!");
}