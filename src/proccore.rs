use bitpat::bitpat;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CoreContext {
    pub mode: u32,
    pub alubus: u32,
    pub oldalubus: u32,
    pub pcbus: u32,
    pub oldpcbus: u32,
    pub incbus: u32,
    pub oldincbus: u32,
    pub addrinc: u32,
    pub oldaddrinc: u32,
    pub a: u32,
    pub olda: u32,
    pub abus: u32,
    pub oldabus: u32,
    pub bbus: u32,
    pub oldbbus: u32,
    pub barrel: u32,
    pub oldbarrel: u32,
    pub barrelfunc: u8,
    pub shiftamnt: u8,
    pub d: u32,
    pub oldd: u32,
    pub addrreg: u32,
    pub reg_sela: u32,
    pub reg_selb: u32,
    pub registers: [u32; 37],
    pub alu_func: u8,
    pub carry: u32,
    pub negative: u32,
    pub zero: u32,
    pub overflow: u32,
    pub cpsr: u32,
    pub setcodes: bool,
    pub tbit: bool,
}

/* Register organisation
Address 0 - 14 (general purpose)
Address 15 - 19 (fiq banked)
Address 20 - 35 (r13,r14,spsr for each mode)
Address 36 - r15
Address 37 - CPSR

need a function which maps 0..15 to the desired register for each mode
*/


impl CoreContext {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CoreContext {
            mode: 0,
            alubus: 0,
            oldalubus: 0,
            pcbus: 0,
            oldpcbus: 0,
            incbus: 0,
            oldincbus: 0,
            addrinc: 0,
            oldaddrinc: 0,
            a: 0,
            olda: 0,
            abus: 0,
            oldabus: 0,
            bbus: 0,
            oldbbus: 0,
            barrel: 0,
            oldbarrel: 0,
            barrelfunc: 0,
            shiftamnt: 0,
            d: 0,
            oldd: 0,
            addrreg: 0,
            reg_sela: 0,
            reg_selb: 0,
            registers: [0; 37],
            alu_func: 0,
            carry: 0,
            negative: 0,
            zero: 0,
            overflow: 0,
            cpsr: 0,
            setcodes: false,
            tbit: false,
        }
    }

    /* Input arguments: A sel, B sel, In sel */
    #[allow(dead_code)]
    pub fn reg_bank(&mut self) {
        self.pcbus = self.registers[15];
    }

    /* Input arguments: function, cpsr */
    #[allow(dead_code)]
    pub fn alu(&mut self) {
        let tmp = match self.alu_func {
            0x0 => (self.abus & self.barrel, false),
            0x1 => (self.abus ^ self.barrel, false),
            0x2 => self.abus.overflowing_sub(self.barrel),
            0x3 => self.barrel.overflowing_sub(self.abus),
            0x4 => self.abus.overflowing_add(self.barrel),
            0x5 => self.abus.overflowing_add(self.barrel+self.carry),
            /* 0x6 => self.abus - self.barrel + self.carry - 1,
            0x7 => self.barrel - self.abus + self.carry - 1, */
            0x8 => (self.abus & self.barrel, false),
            0x9 => (self.abus ^ self.barrel, false),
            0xA => self.abus.overflowing_sub(self.barrel),
            0xB => self.abus.overflowing_add(self.barrel),
            0xC => (self.abus | self.barrel, false),
            0xD => (self.barrel, false),
            0xE => (self.abus & !self.barrel, false),
            0xF => (!self.barrel, false),
            _   => panic!("ALU function {} does not write to alubus", self.alu_func)
        };

        if self.alu_func < 0xB {
            self.alubus = tmp.0;
        }

        self.negative = if tmp.0 > 0x80000000 {1} else {0};
        self.zero = if tmp.0 == 0 {1} else {0};
        self.carry = if tmp.1 {1} else {0};
        self.overflow = if tmp.0 > 0x40000000 {1} else {0};
    }

    /* Barrel shifter */
    #[allow(dead_code)]
    pub fn shift(&mut self) {
        self.barrel = match self.barrelfunc {  /* Need to implement these properly (carry, etc) */
            0   => self.bbus << self.shiftamnt, //LSL
            1   => self.bbus >> self.shiftamnt, //LSR
            2   => (self.bbus as i32 >> self.shiftamnt) as u32, //ASR
            3   => (self.bbus >> self.shiftamnt) | (self.bbus << 32-self.shiftamnt), //ROR
            _   => panic!("Shift type does not exist")
        };
        
        self.carry = match self.barrelfunc {
            0   => (self.bbus >> 32-self.shiftamnt) & 0b1,
            1   => (self.bbus >> self.shiftamnt-1) & 0b1,
            2   => (self.bbus >> self.shiftamnt-1) & 0b1,
            3   => (self.bbus >> self.shiftamnt-1) & 0b1,
            _   => panic!("Shift type does not exist")
        };
    }

    /* Increment the address depending on the processor mode */
    #[allow(dead_code)]
    pub fn inc(&mut self) {
        if self.tbit {
            self.incbus = self.addrinc + 2;
        } else {
            self.incbus = self.addrinc + 4;
        }
    }
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

/* The following two decoding functions are not in order of instruction formats, but are ordered such that they work */
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
pub fn translate_thumb(inst: u16) -> Option<u32> { /* output is Some(x) if there is an equivalent arm inst, otherwise None */
    let insttype = decode_thumb(inst);                  //getting the type of the instruction
    let mut op0: u32 = ((inst & 0x1800) >> 11) as u32;  //declaring all of the unique fields in each instruction
    let off5: u32 = ((inst & 0x7C0) >> 6) as u32;
    let mut rs: u32 = ((inst & 0x38) >> 3) as u32;
    let mut rd0: u32 = (inst & 0x7) as u32;
    let i: u32 = ((inst & 0x400) >> 10) as u32;
    let op1: u32 = ((inst & 0x200) >> 9) as u32;
    let rn: u32 = ((inst & 0x1C0) >> 6) as u32;
    let rd1: u32 = ((inst & 0x700) >> 8) as u32;
    let off8: u32 = (inst & 0xFF) as u32;
    let mut op2: u32 = ((inst & 0x3C0) >> 6) as u32;
    let mut op3: u32 = ((inst & 0x300) >> 8) as u32;
    let h1: u32 = ((inst & 0x80) >> 7) as u32;
    let h2: u32 = ((inst & 0x40) >> 6) as u32;
    let l: u32 = ((inst & 0x800) >> 11) as u32;
    let b: u32 = ((inst & 0x1000) >> 12) as u32;
    let word7: u32 = (inst & 0x7F) as u32;
    let r: u32 = ((inst & 0x100) >> 8) as u32;
    let cond: u32 = ((inst & 0xF00) >> 8) as u32;
    let off11: u32 = (inst & 0x7FF) as u32;
    match insttype {    //from the thumbinsttype enum
        ThumbInstType::SoftwareInterrupt => {
            Some(off8 | 0xEF000000)
        },
        ThumbInstType::AddOffsetToStackPointer => {
            Some(0b11100000000011011101000000000000 | word7 | (1 << 23-h1))
        },
        ThumbInstType::PushPopRegisters => {
            Some(0b11101000001011010000000000000000 | off8 | (((r&!l)&0b1)<<14) | ((r&l)<<15) | (l << 20) | (l << 23) | ((!l&0b1)<<24))
        },
        ThumbInstType::LoadStoreWithRegisterOffset => {
            Some(0b11100111100000000000000000000000 | rn | (rd0 << 12) | (rs << 16) | (i << 22) | (l << 20))
        },
        ThumbInstType::LoadStoreSignExtendedByteHalfword => {
            Some(0b11100001100000000000000010010000 | rn | (l << 5) | (i << 6) | (rd0 << 12) | (rs << 16) | ((i|l) << 20))
        },
        ThumbInstType::ALUOperation => {
            let shift = if op2==0x7 {0b0111} else {0b0};
            if op2==0xD {
                /* Multiply instruction */
                Some(0b11100000000100000000000010010000 | rs | (rd0 << 8) | (rd0 << 16))
            } else if op2==0x9 {
                Some(0b11100010011100000000000000000000 | (rd0 << 12 ) | (rs << 16))
            } else {
                /* Data processing and PSR transfer */
                op2 = match op2 {
                    0x2 ..= 0x4 => 0xC,
                    0x7         => 0xD,
                    0x9         => 0x3,
                    _           => op2
                };
                Some(0b11100000000100000000000000000000 | rs | (shift << 4) | (rd0 << 12) | (rd0 << 16) | (op2 << 21))
            }
        },
        ThumbInstType::HiRegisterOperationsBranchExchange => {
            op3 = match op3 {
                0   =>  0b0100,
                1   =>  0b1010,
                2   =>  0b1101,
                3   =>  0b0000,
                _   =>  panic!("Hi Register thumb opcode {} does not exist", op3)
            };
            rs = rs | (h2 << 3);
            rd0 = rd0 | (h1 << 3);
            if op3 == 0 {
                Some(0b11100001001011111111111100010000 | rs)
            } else {
                Some(0b11100000000000000000000000000000 | rs | (rd0 << 12) | (rd0 << 16) | if op3==10 {0b1<<20} else {0x0} | (op3 << 21))
            }
        },
        ThumbInstType::AddSubtract => {
            let top: u32 = (op1 << 1) | ((!op1&0b1) << 2);
            Some(0b11100000000100000000000000000000 | rn | (rd0 << 12) | (rs << 16) | (top<<21) | (i << 25))
        },
        ThumbInstType::UnconditionalBranch => {
            Some(0b11101010000000000000000000000000 | (off11 >> 1))
        },
        ThumbInstType::PCRelativeLoad => {
            Some(0b11100101100111110000000000000000 | (off8 << 2) | (rd1 << 12))
        },
        ThumbInstType::LoadStoreHalfword => {
            let offhi: u32 = (off5<<1 & 0xF0) >> 4;
            let offlo: u32 = off5<<1 & 0xF;
            Some(0b11100001110000000000000010110000 | (rs << 16) | (rd0 << 12 ) | (l << 20) | (offhi << 8) | offlo)
        },
        ThumbInstType::SPRelativeLoadStore => {
            Some(0b11100101100011010000000000000000 | off8 | (rd1 << 12) | (l << 20))
        },
        ThumbInstType::LoadAddress => {
            Some(0b11100010100011010000001000000000 | off8 | (rd1 << 12) | ((!l&0b1) << 17))
        },
        ThumbInstType::MultipleLoadStore => {
            Some(0b11101000101000000000000000000000 | off8 | (rd1 << 16) | (l << 20))
        },
        ThumbInstType::ConditionalBranch => {
            Some(0b00001010000000000000000000000000 | off8 | (cond << 28))
        },
        ThumbInstType::LongBranchWithLink => {
            /* No equivalent */
            None
        },
        ThumbInstType::MoveShiftedRegister => {
            Some(0b11100001101100000000000000000000 | rs | (op0 << 5) | (off5 << 7) | (rd0 << 12))
        },
        ThumbInstType::MoveCompareAddSubtractImmediate => {
            /* Data and PSR transfer */
            op0 = match op0 {
                0   => 0b1101,
                1   => 0b1010,
                2   => 0b0100,
                3   => 0b0010,
                _   => panic!("Format 3 opcode {} does not exist", op0)
            };
            Some(0b11100010000100000000000000000000 | off8 | (rd1 << 12) | (rd1 << 16) | (op0 << 21))
        },
        ThumbInstType::LoadStoreWithImmediateOffset => {
            Some(0b11100101100000000000000000000000 | off5 | (rd0 << 12) | (rs << 16) | (l << 20) | (b << 22))
        },
        ThumbInstType::Undefined => {
            Some(0b11100110000000000000000000010000)
        }
    }
}