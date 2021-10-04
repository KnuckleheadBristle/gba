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
}