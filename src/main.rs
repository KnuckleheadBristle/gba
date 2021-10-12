mod decode;
mod arm7tdmi;
mod exec;

pub use decode::*;
pub use arm7tdmi::*;
pub use exec::*;

fn main() {
    println!("Hello, world!");

    let mut core = arm7tdmi::Core::new();

    let inst: u32 = 0b00000001011101001011110011000100;

    exec::step_arm(&mut core, inst);

    println!("Core context: \n {}", core);
}