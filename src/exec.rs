use crate::arm7tdmi;
use crate::decode;

use super::{ ArmInstType };

/* 
    In a basic sense, because each instruction takes a variable amount of cycles, there is a cycle counter in the core struct which keeps track of the current step.
    We then match to the cycle depending to do the relevant step for the given instruction.
    e.g.
    match inst {
        Foo::Bar => {
            match cycle {
                0 => { Foo.Bar(); },
                ...
            }
        }
    }
*/

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn step_arm(core: &mut arm7tdmi::Core, inst: u32) {
    /* I should probably make a struct for this so then I don't have to do it every time */
    /* That would probably be less dumb lol */
    let insttype = decode::decode_arm(inst);
    let cond: u32 = (inst & 0xF0000000) >> 27;
    let i: u32  = (inst & 0x2000000) >> 25;
    let opcode: u32 = (inst & 0x1E00000) >> 21;
    let s: u32 = (inst & 0x100000) >> 20;
    let rn: u32 = (inst & 0xF0000) >> 16;
    let rd: u32 = (inst & 0xF000) >> 12;
    let shift: u32 = (inst & 0xFF0) >> 4;
    let rm: u32 = inst & 0xF;
    let rs: u32 = (inst & 0xF00) >> 8;
    let imm: u32 = inst & 0xFF;
    let a: u32 = (inst & 0x200000) >> 21;
    match insttype {
        ArmInstType::DataProcessingOrPSRTransfer => {
            match core.cycle {
                0   =>  {

                },
                _   =>  unimplemented!() /* Next instruction I guess */
            }
        },
        ArmInstType::SingleDataSwap => {
            match core.cycle {
                0   =>  {
                    /* Calculate the value of the register to be stored */
                    /* Don't forget the prefetch, although this can be done before every instruction */
                },
                1   =>  {
                    /* Fetch data from external memory */
                    /* LMAO I don't even have that set up yet */
                    core.addrbus = core.reg.read(rn as usize); //The address is held in register rn
                    core.datareg = core.addrbus; //read from external memory and latch to data register (TBC)
                },
                2   =>  {
                    /* The contents of the source register are written to the external memory */
                    /* Mem.write(core.reg.read(rm as usize)); //Write the source register to external memory */
                },
                3   =>  {
                    /* The data read during cycle 3 is written into the destination register */
                    core.reg.write(rd as usize, core.datareg); //Write into the destination register
                    /* Address is restored to pc+12 */
                },
                _   =>  unimplemented!() 
            };
        },
        _ => panic!("{} Instructions are not implemented", insttype)
    }
    core.inc_cycle();
}