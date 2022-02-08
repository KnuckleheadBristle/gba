use crate::arm7tdmi;
use crate::decode;
use crate::bus;

use super::{ decode::ArmInstType };

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

/* Return types:
    Some(true) - instruction is complete (move to next instruction)
    Some(false) - Condition was not met (NOP time)
    None - instruction not complete (move to next cycle)
*/
#[allow(unused_variables)]
pub fn step_arm(core: &mut arm7tdmi::Core, bus: &mut bus::Bus, inst: u32) -> Option<bool> {
    /* I should probably make a struct for this so then I don't have to do it every time */
    /* That would probably be less dumb lol */
    let insttype = decode::decode_arm(inst);
    let cond: u32 = (inst & 0xF0000000) >> 28;
    let i: u32  = (inst & 0x2000000) >> 25;
    let opcode: u32 = (inst & 0x1E00000) >> 21;
    let l: u32 = (inst & 0x100000) >> 20;
    let rn: u32 = (inst & 0xF0000) >> 16;
    let rd: u32 = (inst & 0xF000) >> 12;
    let shift: u32 = (inst & 0xFF0) >> 4;
    let rm: u32 = inst & 0x0000000F;
    let rs: u32 = (inst & 0xF00) >> 8;
    let imm: u32 = inst & 0x000000FF;
    let a: u32 = (inst & 0x200000) >> 21;
    let b: u32 = (inst & 0x400000) >> 22;
    let u: u32 = (inst & 0x800000) >> 23;
    let p: u32 = (inst & 0x1000000) >> 24;
    let s: u32 = (inst & 0x40) >> 6;
    let h: u32 = (inst & 0x20) >> 5;
    let off: u32 = inst & 0x00000FFF;
    let rlist: u32 = inst & 0x0000FFFF;
    let cp: u32 = (inst & 0xD) >> 5;
    let boff: u32 = inst & 0x00FFFFFF;
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
                        core.barrelbus = ((boff << 2) as i32) as u32; // sign extend offset
                        core.aluop = 4;
                        core.alu();
                        None
                    },
                    1   =>  {
                        if p == 1 { // return address stored in lr if link bit set
                            core.reg.write(14, core.addrbus);
                        }
                        // Fetch is performed from branch destination
                        core.addrbus = core.alubus;
                        core.databus = bus.mem_read_32(core.addrbus as usize);
                        core.fetch();
                        None
                    },
                    2   =>  {
                        // Fetch from destination +L, refilling instruction pipeline
                        core.fetch();
                        // If link, subtract four from r14 to simplify return
                        if p == 1 {
                            core.abus = core.reg.read(14);
                            core.barrelbus = 4;
                            core.aluop = 2;
                            core.alu();
                            core.reg.write(14, core.alubus);
                        }
                        Some(true)
                    },
                    _   =>  panic!("Branch instruction does not have more than 3 cycles; Found {}", core.cycle+1)
                }
            },
            ArmInstType::BranchAndExchange => {
                match core.cycle {
                    0   =>  {
                        // Branch destination and core state is extracted
                        // Prefetch performed from current PC
                        core.fetch();
                        core.abus = core.reg.read(rm as usize);
                        core.reg.write(15, core.abus);
                        core.reg.cpsr.state = (core.abus & 0b1) == 1; //update the processor mode
                        None
                    },
                    1   =>  {
                        // Fetch is performed from the branch destination address using new instruction width
                        core.fetch();
                        None
                    },
                    2   =>  {
                        // Fetch from destination address plus instruction width (to refill pipeline)
                        core.fetch();
                        Some(true)
                    }
                    _   =>  panic!("Branch and Exchange instruction does not have more than 3 cycles; Found {}", core.cycle+1)
                }
            },
            ArmInstType::DataProcessing => {
                match core.cycle {
                    0   =>  {
                        core.fetch();
                        core.abus = core.reg.read(rn as usize);
                        if i==0 {
                            core.bbus = core.reg.read(rm as usize);
                            core.decode_shift(shift);
                        } else {
                            core.bbus = imm;
                            core.decode_shift_imm(shift);
                        }
                    
                        if core.shiftamnt > 0 {
                            core.barrel_shift();
                            core.aluop = opcode as u8;
                            core.alu();
                        } else {
                            core.barrel_shift();
                            core.aluop = opcode as u8;
                            core.alu();
                            core.reg.write(rd as usize, core.alubus);
                        }
                        
                        if core.shiftamnt > 0 {
                            None
                        }else if rd != 0xF {
                            /* normal end */
                            Some(true)
                        } else {
                            None
                        }
                    },
                    1   =>  {
                        if (core.shiftamnt > 0) && (rd == 0xF) {
                            core.reg.write(rd as usize, core.alubus);
                            None
                        } else if rd == 0xF {
                            core.fetch();
                            None
                        } else {
                            core.reg.write(rd as usize, core.alubus);
                            Some(true)
                        }
                    },
                    2   =>  {
                        core.fetch();
                        if (rd == 0xF) && (core.shiftamnt == 0) {
                            /* End of dest=pc */
                            Some(true)
                        } else {
                            None
                        }
                    },
                    3   =>  {
                        core.fetch();
                        Some(true)
                        //Shift and destination is pc
                        //end of shift(Rs) and dest=pc
                    }
                    _   =>  panic!("Data processing instructions do not have more than 4 cycles; Found {}", core.cycle+1)
                }
            },
            ArmInstType::PSRTransfer => {
                match core.cycle {
                    0 => {
                        core.fetch();
                        core.abus = decode::decode_psr_transfer(inst);
                        if i == 0 {
                            core.bbus = core.reg.read(rm as usize);
                        } else {
                            core.bbus = imm;
                            core.shiftamnt = rs;
                            core.barrelfunc = 3;
                        }
                        None
                    },
                    1 => {
                        if core.abus==0 {
                            let psr: u32 = core.reg.read_psr(b);
                            core.reg.write(rd as usize, psr);
                        } else if core.abus==1 {
                            let psr: u32 = core.reg.read(rm as usize);
                            core.reg.write_psr(b, psr);
                        } else {
                            let word: u32 = if i==0 {   // the value is a register
                                core.reg.read(rm as usize)
                            } else {                    // the value is a shifted immediate value
                                core.barrel_shift();
                                core.barrelbus
                            };
                            core.reg.write_psr(b, word);
                        }
                        None
                    },
                    2 => {
                        Some(true)
                    }
                    _   =>  panic!("PSR Transfer instruction do not have more than 3 cycles; Found {}", core.cycle+1)
                }
            }
            ArmInstType::Multiply => {
                match core.cycle {
                    0 => {
                        core.fetch();
                        /* The only cycle that isn't internal */
                        /* Tbh this is probably just a fetch, but im gonna make it setup shit */
                        None
                    },
                    1 => {
                        None
                    },
                    2 => {
                        /* This cycle is repeated depending on how many multiplies are needed */
                        None
                    }
                    3 => {
                        /* The m+1th cycle (the last for a generic multiply) */
                        if a==1 { //accumulate
                            let result: u32 = core.reg.read(rn as usize).wrapping_add(core.reg.read(rs as usize) * core.reg.read(rm as usize));
                            core.reg.write(rd as usize, result);
                            None
                            /* add rn to the calculated value */
                        } else { //normal multiply
                            let result: u32 = core.reg.read(rs as usize) * core.reg.read(rm as usize);

                            core.reg.write(rd as usize, result);
                            Some(true)
                        }
                    },
                    4 => {
                        /* The m+2th cycle (the last for a multiply accumulate) */
                        Some(true)
                    }
                    _ => panic!("Multiply instruction does not have more than 5 cycles; Found {}", core.cycle+1)
                }
            },
            ArmInstType::MultiplyLong => {
                /* This is the exact same as above, but takes an extra step to shift out the extra bits */
                match core.cycle {
                    0 => {
                        None
                    },
                    1 => {
                        None
                    },
                    2 => {
                        /* Same as above, this is the repeated cycle */
                        None
                    },
                    3 => {
                        None
                    },
                    4 => {
                        None
                    },
                    5 => {
                        if a==1 {
                            let accumulate: i64 = (core.reg.read(rn as usize)|core.reg.read(rd as usize)) as i64;
                            let result: i64 = accumulate + (core.reg.read(rs as usize)*core.reg.read(rm as usize)) as i32 as i64;
                            None
                        } else {
                            Some(true)
                        }
                    },
                    6 => {
                        Some(true)
                    }
                    _ => panic!("Multiply Long instruction does not have more than 7 cycles; Found {}", core.cycle+1)
                }
            }
            ArmInstType::SingleDataTransfer => {
                /* This guy has to handle byte and word value types */
                match core.cycle {
                    0 => {
                        core.abus = core.reg.read(15);
                        core.bbus = 
                        if i == 1 {
                            core.decode_shift(off&0xFF0);
                            core.reg.read(rm as usize)
                        } else {
                            core.shiftamnt = 0;
                            core.barrelfunc = 0;
                            off
                        };

                        core.barrel_shift();
                        println!("abus: {}", core.abus);
                        println!("bbus: {}", core.bbus);
                        println!("barrelbus: {}", core.barrelbus);
                        println!("sum: {}", core.abus + core.barrelbus);
                        core.aluop = 0b10 << (u & 0b1); /* if u==0 add else sub */
                        core.alu();
                        println!("aluop: {}", core.aluop);
                        println!("alubus: {}", core.alubus);
                        println!("addrbus: {:08x}", core.addrbus);
                        None
                    },
                    1 => {
                        if l==0 { /* the store instruction */
                            if a==1 || p==0 { //register write-back
                              core.reg.write(rn as usize, core.alubus);
                            }
                            if b==1 { //byte or word quantity
                              bus.mem_write(core.addrbus as usize, core.reg.read(rd as usize) as u8); //byte
                            } else {
                                println!("addrbus: 0x{:08x}", core.addrbus as usize);
                                println!("Destination reg: 0x{:08x}", core.reg.read(rd as usize));
                                bus.mem_write_32(core.addrbus as usize, core.reg.read(rd as usize));   //word
                            }
                            Some(true)
                            /* end of store */
                        } else { /* The load instruction */
                            if a==1 || p==0 { //register write-back
                              core.reg.write(rn as usize, core.alubus);
                            }
                            if b==1 {
                                core.datareg = bus.mem_read(core.addrbus as usize) as u32; //byte (zero extended)
                                println!("datareg written to with a byte value: 0x{:08x}", core.datareg);
                            } else {
                                println!("addrbus: 0x{:08x}", core.addrbus);
                                core.datareg = bus.mem_read_32(core.addrbus as usize); //word
                                println!("datareg written to with word value: 0x{:08x}", core.datareg);
                                println!("{}", core.addrbus);
                                core.reg.write(rn as usize, core.datareg);
                            }
                            /* End of load unless rn = pc */
                            if rd == 15 { //source/dest is pc
                              core.reg.pipeline = [0,0,0];
                            }
                            None
                        }
                    },
                    2 => {
                        println!("0x{:08x}", rn);
                        println!("0x{:08x}", core.datareg);
                        core.reg.write(rn as usize, core.datareg);
                        if rd == 15 {
                            None
                        } else {
                            Some(true)
                        }
                    },
                    3 => {core.fetch(); None},
                    4 => {core.fetch(); Some(true)}, //end of load pc
                    _ => panic!("Data transfer instruction does not have more than 5 cycles; Found {}", core.cycle+1)
                }
            },
            ArmInstType::HalfwordDataTransferRegisterOffset | ArmInstType::HalfwordDataTransferImmediateOffset => {
                match core.cycle {
                    0 => {
                        core.abus = core.reg.read(rn as usize);
                        core.bbus = if insttype == ArmInstType::HalfwordDataTransferRegisterOffset {
                            core.reg.read(rm as usize)
                        } else {
                            (rs >> 4) | rm
                        };
                        
                        core.shiftamnt = 0;
                        core.barrelfunc = 0;
                        core.barrel_shift();
                        core.aluop = 0b10 << (!u & 0b1);
                        None
                    },
                    1 => {
                        if l==0 { /* Store instruction */
                            if a==1 || p==0 { //base register write-back
                                core.reg.write(rn as usize, core.alubus);
                            }
                            let source = match (s<<1)|h { //Data type
                                1 => core.reg.read(rd as usize) as u16,
                                2 => (core.reg.read(rd as usize) as i8) as u8 as u16,
                                3 => (core.reg.read(rd as usize) as i16) as u16,
                                _ => panic!("Store operation {} does not exist", (s<<1)|h)
                            };
                            if h==1 { //Write type
                                bus.mem_write_16(core.addrbus as usize, source);
                            } else {
                                bus.mem_write(core.addrbus as usize, source as u8);
                            }
                            Some(true)
                            /* End of store */
                        } else { /* Load instruction */
                            if a==1 || p==0 {
                                core.reg.write(rn as usize, core.alubus);
                            }
                            core.datareg = match (s<<1)|h {
                                1 => bus.mem_read_16(core.addrbus as usize) as u32,
                                2 => (bus.mem_read(core.addrbus as usize) as i8) as u32,
                                3 => (bus.mem_read_16(core.addrbus as usize) as i16) as u32,
                                _ => panic!("Load operation {} does not exist", ((s<<1)|h))
                            };
                            if rn == 15 {
                                core.reg.pipeline = [0,0,0];
                                None
                            } else {
                                Some(true)
                            }
                        }
                    },
                    2 => {
                        core.reg.write(rn as usize, core.datareg);
                        None
                    },
                    3 => { core.fetch(); None },
                    4 => { core.fetch(); None },
                    _ => panic!("Halfword data transfer does not have more than 5 cycles; Found {}", core.cycle+1)
                }
            },
            ArmInstType::BlockDataTransfer => {
                match core.cycle {
                    0 => {
                        core.fetch();
                        core.calc_reg_transfer(rlist);
                        core.alubus = core.reg.read(rn as usize) - if u==0 {4*core.multicycle as u32} else {0};
                        core.multicycle -= 1;
                        None
                    },
                    1 => {
                        if p==1 {
                            core.alubus += 4;
                        }
                        if l==1 { //load
                            core.datareg = bus.mem_read_32(core.alubus as usize);
                            None
                        } else { //store
                            if core.multicycle != 0 {
                                bus.mem_write_32(core.alubus as usize, core.reg.read(core.transferblock[core.multicycle as usize] as usize));
                                core.multicycle -= 1;
                                core.cycle -= 1;
                                if p==0 {
                                    core.alubus += 4;
                                }
                                None
                            } else {
                                Some(true)
                            }
                        }
                    },
                    2 => {
                        if core.multicycle != 0 {
                            if p==1 {
                                core.alubus += 4;
                            }
                            core.reg.write(core.transferblock[core.multicycle as usize] as usize,core.datareg);
                            core.datareg = bus.mem_read_32(core.alubus as usize);
                            core.addrbus = core.reg.read(rn as usize);
                            core.multicycle -= 1;
                            core.cycle -= 1;
                            if p==0 {
                                core.alubus += 4;
                            }
                            None
                        } else if core.transferblock[core.multicycle as usize] == 15 {
                            core.fetch();
                            None
                        } else {
                            Some(true)
                        }
                    },
                    3 => {
                        core.fetch();
                        None
                    },
                    4 => {
                        core.fetch();
                        Some(true)
                    }
                    _ => panic!("Block data transfer does not have more than 4 cycles; Found {}", core.cycle+1)
                }
            }
            ArmInstType::SingleDataSwap => {
                match core.cycle {
                    0   =>  {
                        //prefetch and address things
                        core.fetch();
                        None
                    },
                    1   =>  {
                        //Data fetched from external memory
                        core.addrbus = core.reg.read(rn as usize);
                        /* read from memory: bus.mem_read() */
                        core.databus = if b == 1 {
                            bus.mem_read(core.addrbus as usize) as u32
                        } else {
                            bus.mem_read_32(core.addrbus as usize)
                        };
                        core.datareg = core.databus;
                        None
                    },
                    2   =>  {
                        //Contents of source register is written to external memory
                        core.databus = core.reg.read(rm as usize);
                        if b == 1 {
                            bus.mem_write(core.addrbus as usize, core.databus as u8);
                        } else {
                            bus.mem_write_32(core.addrbus as usize, core.databus);
                        }
                        None
                    },
                    3   =>  {
                        //Cycle 2 data is written to destination register
                        core.reg.write(core.addrbus as usize, core.databus);
                        Some(true)
                    },
                    _   =>  panic!("Data swap instruction does not have more than 4 cycles; Found {}", core.cycle+1)
                }
            },
            ArmInstType::SoftwareInterrupt => {
                match core.cycle {
                    0   =>  {
                        //Forced address is constructed, mode change may take place

                        core.abus = core.reg.read(15);
                        core.addrbus = 0x0;
                        core.reg.write(14, core.abus);
                        core.reg.spsr_svc = core.reg.cpsr;
                        None
                    },
                    1   =>  {
                        //Modification to return address to facilitate return
                        None
                    },
                    2   =>  {
                        //Refill the pipeline
                        core.fetch();
                        Some(true)
                    },
                    _   =>  panic!("Software interrupt instruction does not have more than 3 cycles; Found {}", core.cycle+1)
                }
            },
            ArmInstType::Undefined => {
                match core.cycle {
                    0   =>  {
                        //Good old prefetch
                        None
                    },
                    1   =>  {
                        //Idk (probably instruction trap offset calculation)
                        None
                    },
                    2   =>  {
                        //Fetch from instruction trap
                        None
                    },
                    3   =>  {
                        //Fetch again to fill pipeline
                        Some(true)
                    },
                    _   =>  panic!("Undefined instructions do not have more than 4 cycles; Found {}", core.cycle+1)
                }
            }
            //_   =>  panic!("Instruction {} not implemented", insttype)
        }
    } else {
        Some(false)
    }
}
