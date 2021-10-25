use bitpat::bitpat;
use std::fmt;

/* 
Enums and pattern matching to decode instructions - plus conversion of thumb instructions into arm instructions where applicable
*/

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

impl fmt::Display for ArmInstType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ArmInstType::*;
        let s = match *self {
            DataProcessingOrPSRTransfer         => "Data Processing",
            Multiply                            => "Multiply",
            MultiplyLong                        => "Multiply Long",
            SingleDataSwap                      => "Single Data Swap",
            BranchAndExchange                   => "Branch and Exchange",
            HalfwordDataTransferRegisterOffset  => "Halfword data transfer with register offset",
            HalfwordDataTransferImmediateOffset => "Halfword data transfer with immediate offset",
            SingleDataTransfer                  =>  "Single Data Transfer",
            Undefined                           => "Undefined",
            BlockDataTransfer                   => "Block Data Transfer",
            Branch                              => "Branch",
            CoprocessorDataTransfer             => "Coprocessor Data Transfer",
            CoprocessorDataOperation            => "Coprocessor Data Operation",
            CoprocessorRegisterTransfer         => "Coprocessor Register Transfer",
            SoftwareInterrupt                   => "Software Interrupt"
        };

        write!(f, "{}", s)
    }
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

/* The following two decoding functions are not in the order they appear in documentation, but are ordered such that pattern matching functions properly */
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

/* Same as above, these are not in the order they appear */
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

/* Translate a thumb instruction into it's arm equivalent */
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
    /* The actual decoding and translation of each instruction */
    match insttype {    //from the thumbinsttype enum
        ThumbInstType::SoftwareInterrupt => {
            Some(off8 | 0xEF000000)
        },
        ThumbInstType::AddOffsetToStackPointer => {
            /* Decoding will logical OR (after some extra computation if needed) each field to a 'base' instruction word */
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