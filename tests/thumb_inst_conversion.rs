#[path = "../src/proccore.rs"]
mod proccore;

/* 
Contained within are the tests for the translation of thumb instructions into ARM instructions

It's probably a lot of unnecessary work, but oh well, I want to make sure

The names should be fairly self-explanatory, but note that they do not follow the same order as they
appear in documentation.

Effort also needs to be made to test every possible instruction, as to ensure that things are definitely
being translated correctly
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_soft_int() {
        let thumbinst: u16  = 0b1101111101000101;
        let arminst: u32    = 0b11101111000000000000000001000101;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_add_sp() {
        let thumbinst: u16  = 0b1011000001100110;
        let arminst: u32    = 0b11100000100011011101000001100110;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b1011000011100110;
        let arminst: u32    = 0b11100000010011011101000001100110;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }
    #[test]

    fn convert_push_pop_reg() {
        let thumbinst: u16  = 0b1011010000011111;
        let arminst: u32    = 0b11101001001011010000000000011111;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b1011010100011111;
        let arminst: u32    = 0b11101001001011010100000000011111;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b1011110001000100;
        let arminst: u32    = 0b11101000101111010000000001000100;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b1011110101000100;
        let arminst: u32    = 0b11101000101111011000000001000100;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_ld_store_reg_offset() {
        let thumbinst: u16  = 0b0101000110010011;
        let arminst: u32    = 0b11100111100000100011000000000110;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0101110111000010;
        let arminst: u32    = 0b11100111110100000010000000000111;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_ldstoresignextend() {
        //arm format 8 = arm format 10
        let thumbinst: u16  = 0b0101001000011100;
        let arminst: u32    = 0b11100001100000110100000010010000;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0101101010101011;
        let arminst: u32    = 0b11100001100101010011000010110010;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0101011001111010;
        let arminst: u32    = 0b11100001100101110010000011010001;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0101111010100011;
        let arminst: u32    = 0b11100001100101000011000011110010;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        /* Need to test the other operations */
    }

    #[test]
    fn convert_alu_operations() {
        let thumbinst: u16  = 0b0100000001100011;
        let arminst: u32    = 0b11100000001100110011000000000100;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0100000111000001;
        let arminst: u32    = 0b11100001101100010001000001110000;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0100001001011101;
        let arminst: u32    = 0b11100010011100110101000000000000;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0100001010110010;
        let arminst: u32    = 0b11100001010100100010000000000110;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0100001101111000;
        let arminst: u32    = 0b11100000000100000000000010010111;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_hi_reg_operations() {
        let thumbinst: u16  = 0b0100010010101111;
        let arminst: u32    = 0b11100000100011111111000000000101;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0100010101100100;
        let arminst: u32    = 0b11100001010101000100000000001100;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0100011011110111;
        let arminst: u32    = 0b11100001101011111111000000001110;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0100011101011000;
        let arminst: u32    = 0b11100001001011111111111100011011;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_add_sub() {
        let thumbinst: u16  = 0b0001100100011000;
        let arminst: u32    = 0b11100000100100110000000000000100;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b0001111110010110;
        let arminst: u32    = 0b11100010010100100110000000000110;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_uncond_branch() {
        let thumbinst: u16  = 0b1110011111111110;
        let arminst: u32    = 0b11101010000000000000001111111111;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_pc_rel_load() {
        let thumbinst: u16  = 0b0100101111010011;
        let arminst: u32    = 0b11100101100111110011001101001100;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_ld_str_halfword() {
        let thumbinst: u16  = 0b1000011100001110;
        let arminst: u32    = 0b11100001110000010110001110111000;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b1000100010111100;
        let arminst: u32    = 0b11100001110101110100000010110100;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_sp_rel_ldstr() {
        let thumbinst: u16  = 0b1001010001111011;
        let arminst: u32    = 0b11100101100011010100000001111011;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_load_address() {
        let thumbinst: u16  = 0b1010001010001111;
        let arminst: u32    = 0b11100010100011110010001010001111;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
        let thumbinst: u16  = 0b1010111000110101;
        let arminst: u32    = 0b11100010100011010110001000110101;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }

    #[test]
    fn convert_load_mult_load_store() {
        let thumbinst: u16  = 0b1010111000110101;
        let arminst: u32    = 0b11100010100011010110001000110101;
        let converted: u32  = proccore::translate_thumb(thumbinst, proccore::decode_thumb(thumbinst));
        assert_eq!(arminst, converted);
    }
}