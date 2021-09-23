use bitpat::bitpat;

pub struct Core {
    pub reg:    Registers,
    pub areg:   u32,
    pub ainc:   u32,
    pub rbank:  [u32; 31],
    pub mult:   u32,
    pub wdr:    u32,
    pub addr:   u32,
    pub inc:    u32,
    pub pcb:    u32,
    pub alu:    u32,
    pub a:      u32,
    pub b:      u32,
    pub data:   u32,
    pub ale:    bool,
    pub abe:    bool,
    pub nenout: bool,
    pub dbe:    bool,
    pub nenin:  bool,
    pub dbgrqi: bool,
    pub eclk:   bool,
    pub nexec:  bool,
    pub isync:  bool,
    pub bl:     u8,
    pub ape:    bool,
    pub mclk:   bool,
    pub nwait:  bool,
    pub nrw:    bool,
    pub mas:    u8,
    pub nirq:   bool,
    pub nfiq:   bool,
    pub nreset: bool,
    pub abort:  bool,
    pub ntrans: bool,
    pub nmreq:  bool,
    pub nopc:   bool,
    pub seq:    bool,
    pub lock:   bool,
    pub ncpi:   bool,
    pub cpa:    bool,
    pub cpb:    bool,
    pub nm:     u8,
    pub tbe:    bool,
    pub tbit:   bool,
    pub highz:  bool,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ArmInstType {
    DataProcessingOrPSRTransfer,
    Multiply,
    MultiplyLong,
    SingleDataSwap,
    BranchAndExchange,
    HalfwordDataTransferRegisterOffset,
    HalfwordDataTransferImmediateOffset,
    SingleDataTransfer,
    Undefined,
    BlockDataTransfer,
    Branch,
    CoprocessorDataTransfer,
    CoprocessorDataOperation,
    CoprocessorRegisterTransfer,
    SoftwareInterrupt
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ThumbInstType {
    MoveShiftedRegister,
    AddSubtract,
    MoveCompareAddSubtractImmediate,
    ALUOperation,
    HiRegisterOperationsBranchExchange,
    PCRelativeLoad,
    LoadStoreWithRegisterOffset,
    LoadStoreSignExtendedByteHalfword,
    LoadStoreWithImmediateOffset,
    LoadStoreHalfword,
    SPRelativeLoadStore,
    LoadAddress,
    AddOffsetToStackPointer,
    PushPopRegisters,
    MultipleLoadStore,
    ConditionalBranch,
    SoftwareInterrupt,
    UnconditionalBranch,
    LongBranchWithLink,
    Undefined
}

pub struct Registers {
    pub r:      [u32; 15],
    pub r_fiq:  [u32; 5],
    pub r13:    [u32; 6],
    pub r14:    [u32; 6],
    pub pc:     u32,
    pub cpsr:   u32,
    pub spsr:   [u32; 5],
    pub sp:     [u32; 6],
    pub lr:     [u32; 6],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            r:      [0; 15],
            r_fiq:  [0; 5],
            r13:    [0; 6],
            r14:    [0; 6],
            pc:     0,
            cpsr:   0,
            spsr:   [0; 5],
            sp:     [0; 6],
            lr:     [0; 6],
        }
    }

    #[allow(dead_code)]
    pub fn read(&self, reg: u8, thumb: bool, mode: u8) -> u32 {
        match reg {
            0 ..= 7 => self.r[reg as usize],
            8       => if thumb {self.sp[mode as usize]} else {if mode==1 {self.r_fiq[0]} else {self.r[8]}},
            9       => if thumb {self.lr[mode as usize]} else {if mode==1 {self.r_fiq[1]} else {self.r[9]}},
            10 ..= 12 => if mode == 1 {self.r_fiq[(reg-8) as usize]} else {self.r[reg as usize]},
            13      => self.r13[mode as usize],
            14      => self.r14[mode as usize],
            _       => panic!("Register {} does not exist!", reg)
        }
    }

}

#[allow(dead_code)]
impl Core {
    pub fn new() -> Self {
        Core {
            reg:    Registers::new(),
            areg:   0,
            ainc:   0,
            rbank:  [0; 31],
            mult:   0,
            wdr:    0,
            addr:   0,
            inc:    0,
            pcb:    0,
            alu:    0,
            a:      0,
            b:      0,
            data:   0,
            ale:    false,
            abe:    false,
            nenout: false,
            dbe:    false,
            nenin:  false,
            dbgrqi: false,
            eclk:   false,
            nexec:  false,
            isync:  false,
            bl:     0,
            ape:    false,
            mclk:   false,
            nwait:  false,
            nrw:    false,
            mas:    0,
            nirq:   false,
            nfiq:   false,
            nreset: false,
            abort:  false,
            ntrans: false,
            nmreq:  false,
            nopc:   false,
            seq:    false,
            lock:   false,
            ncpi:   false,
            cpa:    false,
            cpb:    false,
            nm:     0,
            tbe:    false,
            tbit:   false,
            highz:  false,
        }
    }

    pub fn addr_reg(&self) {

    }

    pub fn addr_inc(&self) {

    }

    pub fn reg_bank(&self) {

    }

    pub fn mult(&self) {

    }

    pub fn barrel_shift(&self) {

    }

    pub fn write_reg(&self) {

    }

    pub fn inst_pipe(&self) {

    }

    fn read_flag(&self, flag: u8) -> u32 {
        match flag {
            0 ..= 3 => (&self.reg.cpsr & (1<<(31-flag))) >> 31-flag,
            4 ..= 6 => (&self.reg.cpsr & (1<<(11-flag))) >> 11-flag,
            7       => &self.reg.cpsr & 0b11111,
            8       => &self.reg.cpsr >> 28,
            _       => panic!("Flag {} does not exist!", flag)
        }
    }

    fn write_flag(&mut self, flag: u8, data: u32) {
        match flag {
            0 ..= 3 => self.reg.cpsr |= data << 31-flag,
            4 ..= 6 => self.reg.cpsr |= data << 11-flag,
            7       => self.reg.cpsr |= data,
            _       => panic!("Flag {} does not exist!", flag)
        }
    }

    pub fn arithmetic(&self, opcode: u32, op1: u32, op2: u32) -> u32 {
        match opcode {
            0 => op1 & op2,
            1 => op1 ^ op2,
            2 => op1 - op2,
            3 => op2 - op1,
            4 => op1 + op2,
            5 => op1 + op2 + self.read_flag(1),
            6 => op1 - op2 + self.read_flag(1) - 1,
            7 => op2 - op1 + self.read_flag(1) - 1,
            12 => op1 | op2,
            13 => op2,
            14 => op1 & !op2,
            15 => !op2,
            _ => panic!("Arithmetic opcode {} not implemented", opcode)
        }
    }
}

#[allow(dead_code)]
pub fn decode_arm(inst: u32) -> ArmInstType {
    if bitpat!( _ _ _ _ 0 0 0 1 0 0 1 0 1 1 1 1 1 1 1 1 1 1 1 1 0 0 0 1 _ _ _ _ )(inst) {ArmInstType::BranchAndExchange}                       else
    if bitpat!( _ _ _ _ 0 0 0 1 0 _ 0 0 _ _ _ _ _ _ _ _ 0 0 0 0 1 0 0 1 _ _ _ _ )(inst) {ArmInstType::SingleDataSwap}                          else
    if bitpat!( _ _ _ _ 0 0 0 0 0 0 _ _ _ _ _ _ _ _ _ _ _ _ _ _ 1 0 0 1 _ _ _ _ )(inst) {ArmInstType::Multiply}                                else
    if bitpat!( _ _ _ _ 0 0 0 0 1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 1 0 0 1 _ _ _ _ )(inst) {ArmInstType::MultiplyLong}                            else
    if bitpat!( _ _ _ _ 0 0 0 _ _ 0 _ _ _ _ _ _ _ _ _ _ 0 0 0 0 1 _ _ 1 _ _ _ _ )(inst) {ArmInstType::HalfwordDataTransferRegisterOffset}      else
    if bitpat!( _ _ _ _ 0 0 0 _ _ 1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ 1 _ _ 1 _ _ _ _ )(inst) {ArmInstType::HalfwordDataTransferImmediateOffset}     else
    if bitpat!( _ _ _ _ 0 1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 1 _ _ _ _ )(inst) {ArmInstType::Undefined}                               else
    if bitpat!( _ _ _ _ 1 1 1 0 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 0 _ _ _ _ )(inst) {ArmInstType::CoprocessorDataOperation}                else
    if bitpat!( _ _ _ _ 1 1 1 0 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 1 _ _ _ _ )(inst) {ArmInstType::CoprocessorRegisterTransfer}             else
    if bitpat!( _ _ _ _ 1 1 1 1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ArmInstType::SoftwareInterrupt}                       else
    if bitpat!( _ _ _ _ 1 0 0 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ArmInstType::BlockDataTransfer}                       else
    if bitpat!( _ _ _ _ 1 0 1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ArmInstType::Branch}                                  else
    if bitpat!( _ _ _ _ 1 1 0 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ArmInstType::CoprocessorDataTransfer}                 else
    if bitpat!( _ _ _ _ 0 1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ArmInstType::SingleDataTransfer}                      else
    if bitpat!( _ _ _ _ 0 0 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ArmInstType::DataProcessingOrPSRTransfer}             else
    {ArmInstType::Undefined}
}

/* This is not the order instruction appear in the technical reference, but are ordered such that decoding works */
#[allow(dead_code)]
pub fn decode_thumb(inst: u16) -> ThumbInstType {
    if bitpat!( 1 1 0 1 1 1 1 1 _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::SoftwareInterrupt} else
    if bitpat!( 1 0 1 1 0 0 0 0 _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::AddOffsetToStackPointer} else
    if bitpat!( 1 0 1 1 _ 1 0 _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::PushPopRegisters} else
    if bitpat!( 0 1 0 1 _ _ 0 _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::LoadStoreWithRegisterOffset} else
    if bitpat!( 0 1 0 1 _ _ 1 _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::LoadStoreSignExtendedByteHalfword} else
    if bitpat!( 0 1 0 0 0 0 _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::ALUOperation} else
    if bitpat!( 0 1 0 0 0 1 _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::HiRegisterOperationsBranchExchange} else
    if bitpat!( 0 0 0 1 1 _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::AddSubtract} else
    if bitpat!( 1 1 1 0 0 _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::UnconditionalBranch} else
    if bitpat!( 0 1 0 0 1 _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::PCRelativeLoad} else
    if bitpat!( 1 0 0 0 _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::LoadStoreHalfword} else
    if bitpat!( 1 0 0 1 _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::SPRelativeLoadStore} else
    if bitpat!( 1 0 1 0 _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::LoadAddress} else
    if bitpat!( 1 1 0 0 _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::MultipleLoadStore} else
    if bitpat!( 1 1 0 1 _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::ConditionalBranch} else
    if bitpat!( 1 1 1 1 _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::LongBranchWithLink} else
    if bitpat!( 0 0 0 _ _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::MoveShiftedRegister} else
    if bitpat!( 0 0 1 _ _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::MoveCompareAddSubtractImmediate} else
    if bitpat!( 0 1 1 _ _ _ _ _ _ _ _ _ _ _ _ _ )(inst) {ThumbInstType::LoadStoreWithImmediateOffset} else
    {ThumbInstType::Undefined}
}

#[allow(dead_code)]
pub fn translate_thumb(inst: u16, insttype: ThumbInstType) -> u32 {
    match insttype {
        ThumbInstType::SoftwareInterrupt => {
            (inst & 0x00FF) as u32 | 0xEF000000
        },
        ThumbInstType::AddOffsetToStackPointer => {
            let s: u32 = ((inst & 0x80)>>7) as u32;
            let sword7 = (inst & 0x7F) as u32;
            0b11100000000011011101000000000000 | sword7 | (1 << 23-s)
        },
        ThumbInstType::PushPopRegisters => {
            let l: u32  = ((inst & 0x800) >> 11) as u32;
            let r: u32  = ((inst & 0x100) >> 8) as u32;
            let rlist: u32 = (inst & 0xFF) as u32;
            0b11101000001011010000000000000000 | rlist | (((r&!l)&0b1)<<14) | ((r&l)<<15) | (l << 20) | (l << 23) | ((!l&0b1)<<24)
        },
        ThumbInstType::LoadStoreWithRegisterOffset => {
            let l: u32  = ((inst & 0x800) >> 11) as u32;
            let b: u32  = ((inst & 0x400) >> 10) as u32;
            let ro: u32 = ((inst & 0x1C0) >> 6) as u32;
            let rb: u32 = ((inst & 0x38) >> 3) as u32;
            let rd: u32 = (inst & 0x7) as u32;
            0b11100111100000000000000000000000 | ro | (rd << 12) | (rb << 16) | (b << 22) | (l << 20)
        },
        ThumbInstType::LoadStoreSignExtendedByteHalfword => {
            let h: u32  = ((inst & 0x800) >> 11) as u32;
            let s: u32  = ((inst & 0x400) >> 10) as u32;
            let ro: u32 = ((inst & 0x1C0) >> 6) as u32;
            let rb: u32 = ((inst & 0x38) >> 3) as u32;
            let rd: u32 = (inst & 0x7) as u32;
            0b11100001100000000000000010010000 | ro | (h << 5) | (s << 6) | (rd << 12) | (rb << 16) | ((s|h) << 20)
        },
        ThumbInstType::ALUOperation => {
            let mut op = ((inst & 0x3C0) >> 6) as u32;
            let rs = ((inst & 0x38) >> 3) as u32;
            let rd = (inst & 0x7) as u32;
            let shift = if op==0x7 {0b0111} else {0b0};
            if op==0xD {
                /* Multiply instruction */
                0b11100000000100000000000010010000 | rs | (rd << 8) | (rd << 16)
            } else if op==0x9 {
                0b11100010011100000000000000000000 | (rd << 12 ) | (rs << 16)
            } else {
                /* Data processing and PSR transfer */
                op = match op {
                    0x2 ..= 0x4 => 0xC,
                    0x7         => 0xD,
                    0x9         => 0x3,
                    _           => op
                };
                0b11100000000100000000000000000000 | rs | (shift << 4) | (rd << 12) | (rd << 16) | (op << 21)
            }
        },
        ThumbInstType::HiRegisterOperationsBranchExchange => {
            let mut op: u32 = ((inst & 0x300) >> 8) as u32;
            let h1: u32 = ((inst & 0x80) >> 7) as u32;
            let h2: u32 = ((inst & 0x40) >> 6) as u32;
            let mut rshs: u32 = ((inst & 0x38) >> 3) as u32;
            let mut rdhd: u32 = (inst & 0x7) as u32;
            op = match op {
                0   =>  0b0100,
                1   =>  0b1010,
                2   =>  0b1101,
                3   =>  0b0000,
                _   =>  panic!("Hi Register thumb opcode {} does not exist", op)
            };
            rshs = rshs | (h2 << 3);
            rdhd = rdhd | (h1 << 3);
            if op == 0 {
                0b11100001001011111111111100010000 | rshs
            } else {
                0b11100000000000000000000000000000 | rshs | (rdhd << 12) | (rdhd << 16) | if op==10 {0b1<<20} else {0x0} | (op << 21)
            }
        },
        ThumbInstType::AddSubtract => {
            let i: u32 = ((inst & 0x400) >> 10) as u32;
            let op: u32 = ((inst & 0x200) >> 9) as u32;
            let rn: u32 = ((inst & 0x1C0) >> 6) as u32;
            let rs: u32 = ((inst & 0x38) >> 3) as u32;
            let rd: u32 = (inst & 0x7) as u32;
            let top: u32 = (op << 1) | ((!op&0b1) << 2);
            0b11100000000100000000000000000000 | rn | (rd << 12) | (rs << 16) | (top<<21) | (i << 25)
        },
        ThumbInstType::UnconditionalBranch => {
            let off11: u32 = (inst &0x7FF) as u32;
            0b11101010000000000000000000000000 | (off11 >> 1)
        },
        ThumbInstType::PCRelativeLoad => {
            let rd: u32 = ((inst & 0x700) >> 8) as u32;
            let word8: u32 = (inst & 0xFF) as u32;
            0b11100101100111110000000000000000 | (word8 << 2) | (rd << 12)
        },
        ThumbInstType::LoadStoreHalfword => {
            let l: u32 = ((inst & 0x800) >> 11) as u32;
            let offset5: u32 = ((inst & 0x7C0) >> 5) as u32;
            let offhi: u32 = (offset5 & 0xF0) >> 4;
            let offlo: u32 = offset5 & 0xF;
            let rb: u32 = ((inst & 0x38) >> 3) as u32;
            let rd: u32 = (inst & 0x7) as u32;
            0b11100001110000000000000010110000 | (rb << 16) | (rd << 12 ) | (l << 20) | (offhi << 8) | offlo
        },
        ThumbInstType::SPRelativeLoadStore => {
            let l: u32 = ((inst & 0x800) >> 11) as u32;
            let rd: u32 = ((inst & 0x700) >> 8) as u32;
            let word8: u32 = (inst & 0xFF) as u32;
            0b11100101100011010000000000000000 | word8 | (rd << 12) | (l << 20)
        },
        ThumbInstType::LoadAddress => {
            let sp: u32 = ((inst & 0x800) >> 11) as u32;
            let rd: u32 = ((inst & 0x700) >> 8) as u32;
            let word8: u32 = (inst & 0xFF) as u32;
            println!("sp {} rd {} word8 {}", sp, rd, word8);
            0b11100010100011010000001000000000 | word8 | (rd << 12) | ((!sp&0b1) << 17)
        },
        ThumbInstType::MultipleLoadStore => {
            let l: u32 = ((inst & 0x800) >> 11) as u32;
            let rb: u32 = ((inst & 0x700) >> 8) as u32;
            let rlist: u32 = (inst & 0xFF) as u32;
            0b11101000101000000000000000000000 | rlist | (rb << 16) | (l << 20)
        },
        ThumbInstType::ConditionalBranch => {
            let cond: u32 = ((inst & 0xF00) >> 8) as u32;
            let soff: u32 = (inst & 0xFF) as u32;
            0b00001010000000000000000000000000 | soff | (cond << 28)
        }
        _   =>  panic!("Thumb instruction is not implemented!")
    }
}