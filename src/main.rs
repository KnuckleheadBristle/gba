/* Declare so we get linting */
mod decode;
mod arm7tdmi;
mod exec;
mod bus;

pub use decode::*;
pub use arm7tdmi::*;
pub use exec::*;
pub use bus::*;

use std::{thread, time};

/*
This is here like this so that the rust analyser will actually give me what's
wrong with my code, otherwise I don't get linting. Plus we want to make sure that
cargo knows what to do when building/running etc.
*/


fn test_inst(core: &mut arm7tdmi::Core, bus: &mut bus::Bus, inst: u32) {
    core.cycle = 0;

    println!("Inst: 0x{:x}, Type: {}, Dissassembly: {}, PC: 0x{:x}", inst, decode::decode_arm(inst), decode::disassemble_arm(inst), core.reg.gp[15]);

    let mut state = None;

    while state == None {
        println!("{}", core.reg);
        state = exec::step_arm(core, bus, inst);
        core.cycle += 1;
    }

    core.reg.gp[15] += 4;
}

fn main() { //I will probably make this do some thing later, but it will stay like this for now
    let one_second = time::Duration::from_millis(500);

    println!("Hello, world!");

    thread::sleep(one_second);

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

    thread::sleep(one_second);

    let mut core = arm7tdmi::Core::new();
        let mut bus = bus::Bus::new();

        println!("{:?}", bus.load_mem());
        
        let mut instructions = 200;
        
        let mut inst_same_counter = 0;

        let mut old_inst = 0;

        print!("\x1B[2J\x1B[1;1H");
        
        while instructions < 300 {
            let instruction = bus.mem_read_32(core.reg.gp[15] as usize);
            test_inst(&mut core, &mut bus, instruction);
            instructions -= 1;
            if old_inst == instruction {
                inst_same_counter += 1;
            }
            if inst_same_counter > 10 {
                panic!("Infinite loop")
            }
            old_inst = instruction;

            //print!("\x1B[2J\x1B[1;1H");
            thread::sleep(one_second);
        }

        assert_eq!(core.reg.gp[12], 0);
}
