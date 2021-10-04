#[path = "../src/proccore.rs"]
mod proccore;

/* 
This suit of tests will ensure that the functioning of all the processor 'modules' (the different blocks) operate correctly

Please see the arm7tdmi documentation for more information on their operation (most information can be learnt from instruction documentation)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inst_inc() {
        let mut core = proccore::CoreContext::new();
        
        core.tbit = false;
        core.addrinc = 0x101D;

        core.inc();

        assert_eq!(core.incbus, 0x1021);

        core.tbit = true;
        core.inc();

        assert_eq!(core.incbus, 0x101F);
    }

    #[test]
    fn barrel_shift() {
        let mut core = proccore::CoreContext::new();

        /* These two are constant through tests */
        core.bbus = 0b11001001010010100101011010101100;
        core.shiftamnt = 18;

        core.barrelfunc = 0; // LSL
        core.shift();

        assert_eq!(core.barrel, 0b01011010101100000000000000000000);
        assert_eq!(core.carry, 0b1);

        core.barrelfunc = 1; //RSR
        core.shift();

        assert_eq!(core.barrel, 0b11001001010010);
        assert_eq!(core.carry, 0b1);

        core.barrelfunc = 2; //ASR
        core.shift();

        assert_eq!(core.barrel, 0b11111111111111111111001001010010);
        assert_eq!(core.carry, 0b1);

        core.barrelfunc = 3; //ROR
        core.shift();

        assert_eq!(core.barrel, 0b10010101101010110011001001010010);
        assert_eq!(core.carry, 0b1);
    }

    #[test]
    fn alu_ops() {
        let mut core = proccore::CoreContext::new();

        core.abus = 0x123201AD;
        core.barrel = 0x11231EDA;

        core.alu_func = 0;
        core.alu();
        assert_eq!(core.alubus, 0x10220088);

        core.alu_func = 1;
        core.alu();
        assert_eq!(core.alubus, 0x03111F77);

        core.alu_func = 2;
        core.alu();
        assert_eq!(core.alubus, 0x010EE2D3);
        
        core.alu_func = 3;
        core.alu();
        assert_eq!(core.alubus, 0xFEF11D2D);
        
        core.alu_func = 4;
        core.alu();
        assert_eq!(core.alubus, 0x23552087);

        core.alu_func = 5;
        core.alu();
        assert_eq!(core.alubus, 0x23552087); // carry is not being set, so this is wrong
    }
}