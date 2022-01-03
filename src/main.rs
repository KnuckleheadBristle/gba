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

    println!("{}", decode::decode_arm(0xEA00002E));
    println!("{}", decode::decode_arm(0x51AEFF24));
    println!("{}", decode::decode_arm(0x21A29A69));
    println!("{}", decode::decode_arm(0x0A82843D));
    println!("{}", decode::decode_arm(0xAD09E484));
    println!("{}", decode::decode_arm(0x988B2411));
    println!("{}", decode::decode_arm(0x217F81C0));
    println!("{}", decode::decode_arm(0x19BE52A3));
    println!("{}", decode::decode_arm(0x20CE0993));

    println!("{}", decode::disassemble_arm(0xEA00002E));
    println!("{}", decode::disassemble_arm(0x51AEFF24));
    println!("{}", decode::disassemble_arm(0x21A29A69));
    println!("{}", decode::disassemble_arm(0x0A82843D));
    println!("{}", decode::disassemble_arm(0xAD09E484));
    println!("{}", decode::disassemble_arm(0x988B2411));
    println!("{}", decode::disassemble_arm(0x217F81C0));
    println!("{}", decode::disassemble_arm(0x19BE52A3));
    println!("{}", decode::disassemble_arm(0x20CE0993));
}