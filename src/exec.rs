use crate::arm7tdmi;
use crate::decode;

use super::{ ArmInstType, };

pub fn step_arm(core: &mut arm7tdmi::Core, inst: u32) {
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
            core.setcond = s == 1;
            core.asel = rn;
            core.writesel = rd;
            if i==1 {

            } else {
                core.bsel = rm;
                core.decode_shift(shift);
            }
        }
        _ => unimplemented!()
    }
}