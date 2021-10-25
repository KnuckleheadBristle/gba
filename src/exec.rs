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

/* This function needs to be expanded to include instructions of the same basic type but different data types (long/short/signed/etc) */
#[allow(unused_variables)]
pub fn step_arm(core: &mut arm7tdmi::Core, inst: u32) {
    /* I should probably make a struct for this so then I don't have to do it every time */
    /* That would probably be less dumb lol */
    let insttype = decode::decode_arm(inst);
    let cond: u32 = (inst & 0xF0000000) >> 27;
    let i: u32  = (inst & 0x2000000) >> 25;
    let opcode: u32 = (inst & 0x1E00000) >> 21;
    let l: u32 = (inst & 0x100000) >> 20;
    let rn: u32 = (inst & 0xF0000) >> 16;
    let rd: u32 = (inst & 0xF000) >> 12;
    let shift: u32 = (inst & 0xFF0) >> 4;
    let rm: u32 = inst & 0xF;
    let rs: u32 = (inst & 0xF00) >> 8;
    let imm: u32 = inst & 0xFF;
    let a: u32 = (inst & 0x200000) >> 21;
    let b: u32 = (inst & 0x400000) >> 22;
    let u: u32 = (inst & 0x800000) >> 23;
    let p: u32 = (inst & 0x1000000) >> 24;
    let s: u32 = (inst & 0x40) >> 6;
    let h: u32 = (inst & 0x20) >> 5;
    let off: u32 = inst & 0xFFF;
    let rlist: u32 = inst & 0xFFFF;
    let cp: u32 = (inst & 0xD) >> 5;
    let boff: u32 = inst & 0xFFFFFFF;
    core.fetch(); //prefetch the next instruction

    /* If the condition is not met, one cycle is added (one step of this function = one cycle) */
    if core.cond_codes(cond) {
        match insttype {
            ArmInstType::Branch => {
                match core.cycle {
                    0   =>  {
                        // Branch destination and core state is extracted
                        // Prefetch performed from current PC
                        core.abus = core.reg.read(15);
                        core.bbus = ((boff << 2) as i32) as u32; // sign extend offset
                        core.aluop = 4;
                        core.alu();
                    },
                    1   =>  {
                        if p == 1 { // return address stored in lr if link bit set
                            core.reg.write(14, core.addrbus);
                        }
                        // Fetch is performed from branch destination
                        core.addrbus = core.alubus;
                        core.fetch();
                        
                    },
                    2   =>  {
                        // Fetch from destination +L, refilling instruction pipeline
                        core.fetch();
                        // If link, subtract four from r14 to simplify return
                        if p == 1 {
                            core.abus = core.reg.read(14);
                            core.bbus = 4;
                            core.aluop = 2;
                            core.alu();
                            core.reg.write(14, core.alubus);
                        }
                    },
                    _   =>  unimplemented!()
                }
            },
            ArmInstType::BranchAndExchange => {
                match core.cycle {
                    0   =>  {
                        // Branch destination and core state is extracted
                        // Prefetch performed from current PC
                    },
                    1   =>  {
                        // Fetch is performed from the branch destination address using new instruction width
                    },
                    2   =>  {
                        // Fetch from destination address plus instruction width (to refill pipeline)
                    }
                    _   =>  unimplemented!()
                }
            },
            ArmInstType::DataProcessingOrPSRTransfer => {
                match core.cycle {
                    0   =>  {
                        if core.shiftamnt > 0 && rd == 0xF { //Shift and destination is pc
    
                        } else if core.shiftamnt > 0 { //Shift
                            
                        } else if rd == 0xF { //Destination is pc
    
                        } else { //normal
                            //end
                        }
                    },
                    1   =>  {
                        if core.shiftamnt > 0 && rd == 0xF { //Shift and destination is pc
    
                        } else if core.shiftamnt > 0 { //Shift
                            //end
                        } else if rd == 0xF { //Destination is pc
    
                        }
                    },
                    2   =>  {
                        if core.shiftamnt > 0 && rd == 0xF { //Shift and destination is pc
    
                        } else if rd == 0xF { //Destination is pc
                            //end
                        }
                    },
                    3   =>  {
                        //Shift and destination is pc
                        //end
                    }
                    _   =>  unimplemented!()
                }
            },
            ArmInstType::SingleDataTransfer => {
                /* This guy has to handle byte and word value types */
                match core.cycle {
                    0   =>  {

                        //Address calculation
                        if rd == 0xF {
    
                        }
                    },
                    1   =>  {
    
                    },
                    2   =>  {
    
                    },
                    3   =>  {
    
                    },
                    4   =>  {
    
                    },
                    _   =>  unimplemented!()
                }
            },
            ArmInstType::BlockDataTransfer => {
    
            },
            ArmInstType::SingleDataSwap => {
                /* This guy has to handle byte and word value types */
                match core.cycle {
                    0   =>  {
                        //prefetch and address things
                        core.fetch();
                    },
                    1   =>  {
                        //Data fetched from external memory
                        core.addrbus = core.reg.read(rn as usize);
                        /* read from memory: core.bus.mem_read() */
                        core.databus = if b == 1 {
                            core.bus.mem_read(core.addrbus as usize) as u32
                        } else {
                            core.bus.mem_read_32(core.addrbus as usize)
                        };
                        core.datareg = core.databus;
                    },
                    2   =>  {
                        //Contents of source register is written to external memory
                        core.databus = core.reg.read(rm as usize);
                        /* write data to memory: core.bus.mem_write() */
                        if b == 1 {
                            core.bus.mem_write(core.addrbus as usize, core.databus as u8);
                        } else {
                            core.bus.mem_write_32(core.addrbus as usize, core.databus);
                        }
                    },
                    3   =>  {
                        //Cycle 2 data is written to destination register
                        core.reg.write(core.addrbus as usize, core.databus);
                    },
                    _   =>  unimplemented!()
                }
            },
            ArmInstType::SoftwareInterrupt => {
                match core.cycle {
                    0   =>  {
                        //Forced address is constructed, mode change may take place
                    },
                    1   =>  {
                        //Modification to return address to facilitate return
                    },
                    2   =>  {
                        //Refill the pipeline
                    },
                    _   =>  unimplemented!()
                }
            },
            ArmInstType::Undefined => {
                match core.cycle {
                    0   =>  {
                        //Good old prefetch
                    },
                    1   =>  {
                        //Idk (probably instruction trap offset calculation)
                    },
                    2   =>  {
                        //Fetch from instruction trap
                    },
                    3   =>  {
                        //Fetch again to fill pipeline
                    },
                    _   =>  unimplemented!()
                }
            }
            _ => panic!("{} Instructions are not implemented", insttype)
        }
    }
    core.inc_cycle();
}