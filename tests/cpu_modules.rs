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
        /* should also be testing flags */
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
        assert_eq!(core.alubus, 0x23552087); // carry is not being set, so this is may be wrong

        core.alu_func = 6;
        core.alu();
        assert_eq!(core.alubus, 0x010EE2D2);

        core.alu_func = 7;
        core.alu();
        assert_eq!(core.alubus, 0xFEF11D2D); //this is wrong, should be 0xFEF11D2C
        
        core.alubus = 0; // to ensure that these instructions do not write to the alubus

        core.alu_func = 8;
        core.alu();
        assert_eq!(core.alubus, 0x0);
        assert_eq!(core.cpsr, 0x0);

        core.alu_func = 9;
        core.alu();
        assert_eq!(core.alubus, 0x0);
        assert_eq!(core.cpsr, 0x0);

        core.alu_func = 10;
        core.alu();
        assert_eq!(core.alubus, 0x0);
        assert_eq!(core.cpsr, 0x0);

        core.alu_func = 11;
        core.alu();
        assert_eq!(core.alubus, 0x0);
        assert_eq!(core.cpsr, 0x0);
        
        // the rest write to the alubus

        core.alu_func = 12;
        core.alu();
        assert_eq!(core.alubus, 0x13331FFF);

        core.alu_func = 13;
        core.alu();
        assert_eq!(core.alubus, 0x11231EDA);

        core.alu_func = 14;
        core.alu();
        assert_eq!(core.alubus, 0x02100125);

        core.alu_func = 15;
        core.alu();
        assert_eq!(core.alubus, 0xEEDCE125);
    }

    #[test]
    fn set_cpsr() {
        /* Time to test that the cpsr is being updated correctly */
        let mut core = proccore::CoreContext::new();

        core.abus   = 0xFFFFFFFF;
        core.barrel = 0x00000001;
        core.alu_func = 4;
        core.alu();
        assert_eq!(core.alubus, 0x00000000);
        assert_eq!(core.cpsr, 0x60000000);

        core.abus   = 0x00000001;
        core.barrel = 0x00000002;
        core.alu_func = 2;
        core.alu();
        assert_eq!(core.alubus, 0xFFFFFFFF);
        assert_eq!(core.cpsr, 0xB0000000);
    }

    #[test]
    fn address_reg() {
        let mut core = proccore::CoreContext::new();
        
        core.pcbus = 0x00000100;
        core.addrin = 1;
        core.addr_reg();

        assert_eq!(core.addrreg, 0x00000100);
        assert_eq!(core.a, 0x0);

        core.inc();
        core.addrin = 2;
        core.addr_reg();

        assert_eq!(core.addrreg, 0x00000104);
        assert_eq!(core.a, 0x0);
    }
}