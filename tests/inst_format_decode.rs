#[path = "../src/decode.rs"]
mod decode;

/* 
These tests are to make sure that the different instruction groups are being correcly detected. (ARM and thumb)

Refer to arm7 documentation for more details
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    /* Arm tests */
    #[test]
    fn test_arm_branchandex() {
        assert_eq!(decode::decode_arm(0b00000001001011111111111100011010), decode::ArmInstType::BranchAndExchange)
    }

    #[test]
    fn test_arm_brnchandbrnchlnk() {
        assert_eq!(decode::decode_arm(0b00001010010101010101010101010101), decode::ArmInstType::Branch);
    }

    #[test]
    fn test_arm_dataproc() {
        assert_eq!(decode::decode_arm(0b00000001011101001011110011000100), decode::ArmInstType::DataProcessingOrPSRTransfer);
    }

    #[test]
    fn test_arm_mult() {
        assert_eq!(decode::decode_arm(0b00000000001100010010001110010100), decode::ArmInstType::Multiply);
    }

    #[test]
    fn test_arm_multlong() {
        assert_eq!(decode::decode_arm(0b00000000101100010100001010010011), decode::ArmInstType::MultiplyLong);
    }

    #[test]
    fn test_arm_singledattrans() {
        assert_eq!(decode::decode_arm(0b00000110100001000011101010101010), decode::ArmInstType::SingleDataTransfer);
    }

    #[test]
    fn test_arm_halfsinedattrans() {
        assert_eq!(decode::decode_arm(0b00000001001101000101000011111001), decode::ArmInstType::HalfwordDataTransferRegisterOffset);
    }

    #[test]
    fn test_arm_blockdattrans() {
        assert_eq!(decode::decode_arm(0b00001001011001100110011001111001), decode::ArmInstType::BlockDataTransfer);
    }

    #[test]
    fn test_arm_singdatswap() {
        assert_eq!(decode::decode_arm(0b00000001010001001001000010011011), decode::ArmInstType::SingleDataSwap);
    }

    #[test]
    fn test_arm_softint() {
        assert_eq!(decode::decode_arm(0b00001111010101010101010101011101), decode::ArmInstType::SoftwareInterrupt);
    }

    #[test]
    fn test_arm_copdataop() {
        assert_eq!(decode::decode_arm(0b00001110100101111001000110101010), decode::ArmInstType::CoprocessorDataOperation);
    }

    #[test]
    fn test_arm_copdatatrans() {
        assert_eq!(decode::decode_arm(0b00001101001001101101000110101101), decode::ArmInstType::CoprocessorDataTransfer);
    }

    #[test]
    fn test_arm_copregtrans() {
        assert_eq!(decode::decode_arm(0b00001110010001010110000110111011), decode::ArmInstType::CoprocessorRegisterTransfer);
    }

    #[test]
    fn test_arm_undefined() {
        assert_eq!(decode::decode_arm(0b00000110101010101010101010111001), decode::ArmInstType::Undefined);
    }
    
    /* Thumb tests */
    #[test]
    fn test_thumb_moveshifted() {
        assert_eq!(decode::decode_thumb(0b0000011001101010), decode::ThumbInstType::MoveShiftedRegister);
        assert_eq!(decode::decode_thumb(0b0000111001101010), decode::ThumbInstType::MoveShiftedRegister);
        assert_eq!(decode::decode_thumb(0b0001011001101010), decode::ThumbInstType::MoveShiftedRegister);
    }

    #[test]
    fn test_thumb_addsub() {
        assert_eq!(decode::decode_thumb(0b0001100100011000), decode::ThumbInstType::AddSubtract);
        assert_eq!(decode::decode_thumb(0b0001101100011000), decode::ThumbInstType::AddSubtract);
        assert_eq!(decode::decode_thumb(0b0001110110010110), decode::ThumbInstType::AddSubtract);
        assert_eq!(decode::decode_thumb(0b0001111110010110), decode::ThumbInstType::AddSubtract);
    }

    #[test]
    fn test_thumb_movecmp() {
        assert_eq!(decode::decode_thumb(0b0010000010000000), decode::ThumbInstType::MoveCompareAddSubtractImmediate);
        assert_eq!(decode::decode_thumb(0b0010101000111110), decode::ThumbInstType::MoveCompareAddSubtractImmediate);
        assert_eq!(decode::decode_thumb(0b0011000111111111), decode::ThumbInstType::MoveCompareAddSubtractImmediate);
        assert_eq!(decode::decode_thumb(0b0011111010010001), decode::ThumbInstType::MoveCompareAddSubtractImmediate);
    }

    #[test]
    fn test_thumb_aluops() {
        assert_eq!(decode::decode_thumb(0b0100000000011100), decode::ThumbInstType::ALUOperation);
    }

    #[test]
    fn test_thumb_hireg() {
        assert_eq!(decode::decode_thumb(0b0100010010111101), decode::ThumbInstType::HiRegisterOperationsBranchExchange);
    }

    #[test]
    fn test_thumb_pcrelative() {
        assert_eq!(decode::decode_thumb(0b0100101111010011), decode::ThumbInstType::PCRelativeLoad)
    }

    #[test]
    fn test_thumb_loadstorereg() {
        assert_eq!(decode::decode_thumb(0b0101000110010011), decode::ThumbInstType::LoadStoreWithRegisterOffset);
    }

    #[test]
    fn test_thumb_loadstoresign() {
        assert_eq!(decode::decode_thumb(0b0101001000011100), decode::ThumbInstType::LoadStoreSignExtendedByteHalfword);
    }

    #[test]
    fn test_thumb_loadstoreimm() {
        assert_eq!(decode::decode_thumb(0b0111011101101010), decode::ThumbInstType::LoadStoreWithImmediateOffset);
    }

    #[test]
    fn test_thumb_loadstorehalf() {
        assert_eq!(decode::decode_thumb(0b1000011100001110), decode::ThumbInstType::LoadStoreHalfword);
    }

    #[test]
    fn test_thumb_sprelloadstore() {
        assert_eq!(decode::decode_thumb(0b1001010001111011), decode::ThumbInstType::SPRelativeLoadStore);
    }

    #[test]
    fn test_thumb_loadaddr() {
        assert_eq!(decode::decode_thumb(0b1010001010001111), decode::ThumbInstType::LoadAddress);
    }

    #[test]
    fn test_thumb_addoffsp() {
        assert_eq!(decode::decode_thumb(0b1011000001000011), decode::ThumbInstType::AddOffsetToStackPointer);
    }

    #[test]
    fn test_thumb_pushpopreg() {
        assert_eq!(decode::decode_thumb(0b1011010100000100), decode::ThumbInstType::PushPopRegisters);
    }

    #[test]
    fn test_thumb_multloadstore() {
        assert_eq!(decode::decode_thumb(0b1100000000110111), decode::ThumbInstType::MultipleLoadStore);
    }

    #[test]
    fn test_thumb_condbranch() {
        assert_eq!(decode::decode_thumb(0b1101110010101010), decode::ThumbInstType::ConditionalBranch);
    }

    #[test]
    fn test_thumb_softint() {
        assert_eq!(decode::decode_thumb(0b1101111100010010), decode::ThumbInstType::SoftwareInterrupt);
    }

    #[test]
    fn test_thumb_uncndbranch() {
        assert_eq!(decode::decode_thumb(0b1110001010101010), decode::ThumbInstType::UnconditionalBranch);
    }

    #[test]
    fn test_thumb_lngbranchlink() {
        assert_eq!(decode::decode_thumb(0b1111001010101010), decode::ThumbInstType::LongBranchWithLink);
    }
}