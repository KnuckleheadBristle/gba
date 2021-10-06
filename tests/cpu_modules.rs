#[path = "../src/arm7tdmi.rs"]
mod arm7tdmi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alu_functions() {
        let mut core = arm7tdmi::Core::new();
        core.setcond = true; //to set flags

        /* AND */
        core.aluop = 0;
        core.abus = 0xFFFFFFFF;
        core.barrelbus = 0xFFFFFFFF;

        core.alu();

        assert_eq!(core.alubus, 0xFFFFFFFF);
        assert_eq!(u32::from(core.reg.cpsr), 0x80000000);
        
        /* EOR */
        core.aluop = 1;
        core.abus = 0x10101110;
        core.barrelbus = 0x01010101;

        core.alu();

        assert_eq!(core.alubus, 0x11111011);
        assert_eq!(u32::from(core.reg.cpsr), 0);

        /* SUB */
        core.aluop = 2;
        core.abus = 0xF0E0D0C0;
        core.barrelbus = 0xFE000000;

        core.alu();

        assert_eq!(core.alubus, 0xF2E0D0C0);
        assert_eq!(u32::from(core.reg.cpsr), 0xA0000000);

        /* RSB */
        core.aluop = 3;

        core.alu();

        assert_eq!(core.alubus, 0x0D1F2F40);
        assert_eq!(u32::from(core.reg.cpsr), 0);

        /* ADD */
        core.aluop = 4;
        core.abus = 0x01001123;
        core.barrelbus = 0x10321067;

        core.alu();

        assert_eq!(core.alubus, 0x1132218A);
        assert_eq!(u32::from(core.reg.cpsr), 0);

        /* ADC */
        core.aluop = 5;
        core.reg.cpsr.c = true;
        core.abus = 0x12302113;
        core.barrelbus = 0xEFAB1230;

        core.alu();

        assert_eq!(core.alubus, 0x01DB3344);
        assert_eq!(u32::from(core.reg.cpsr), 0x20000000);

        /* SBC */
        core.aluop = 6;
        core.abus = 0xF0E0D0C0;
        core.barrelbus = 0xFE000000;

        core.alu();

        assert_eq!(core.alubus, 0xF2E0D0C0);
        assert_eq!(u32::from(core.reg.cpsr), 0xA0000000);

        /* RSC */
        core.aluop = 7;
        core.abus = 0xFDF00000;
        core.barrelbus = 0x0FEEF001;

        core.alu();

        assert_eq!(core.alubus, 0x11FEF001);
        assert_eq!(u32::from(core.reg.cpsr), 0x20000000);

        core.reg.cpsr.c = false;
        core.alubus = 0;

        /* TST */
        core.aluop = 8;
        core.abus = 0xFFFFFFFF;
        core.barrelbus = 0xFFFFFFFF;

        core.alu();

        assert_eq!(core.alubus, 0);
        assert_eq!(u32::from(core.reg.cpsr), 0x80000000);

        /* TEQ */
        core.aluop = 9;
        core.abus = 3;
        core.barrelbus = 3;

        core.alu();
        
        assert_eq!(core.alubus, 0);
        assert_eq!(u32::from(core.reg.cpsr), 0x40000000);

        /* CMP */
        core.aluop = 0xA;
        core.abus = 0xF0E0D0C0;
        core.barrelbus = 0xFE000000;

        core.alu();

        assert_eq!(core.alubus, 0);
        assert_eq!(u32::from(core.reg.cpsr), 0xA0000000);

        /* CMN */
        core.aluop = 0xB;
        core.abus = 0x01001123;
        core.barrelbus = 0x10321067;

        core.alu();

        assert_eq!(core.alubus, 0);
        assert_eq!(u32::from(core.reg.cpsr), 0);

        /* ORR */
        core.aluop = 0xC;
        core.abus = 0x10100101;
        core.barrelbus = 0x13123132;

        core.alu();
        assert_eq!(core.alubus, 0x13123133);
        assert_eq!(u32::from(core.reg.cpsr), 0);

        /* MOV */
        core.aluop = 0xD;
        
        core.alu();

        assert_eq!(core.alubus, 0x13123132);
        assert_eq!(u32::from(core.reg.cpsr), 0);

        /* BIC */
        core.aluop = 0xE;
        core.abus = 0xFFFFFFFF;
        core.barrelbus = 0xFFFFFFFF;

        core.alu();

        assert_eq!(core.alubus, 0);
        assert_eq!(u32::from(core.reg.cpsr), 0x40000000);

        /* MVN */
        core.aluop = 0xF;
        core.barrelbus = 0x13123132;

        core.alu();

        assert_eq!(core.alubus, 0xECEDCECD);
        assert_eq!(u32::from(core.reg.cpsr), 0x80000000);
    }

    #[test]
    fn barrel_shifting() {
        let mut core = arm7tdmi::Core::new();
        core.bbus = 0b10100101110000111001011001011010;
        /* LSL */
        core.barrelfunc = 0;
        
        core.shiftamnt = 0; //#0
        core.barrel_shift();

        assert_eq!(core.barrelbus, 0b10100101110000111001011001011010);
        assert_eq!(core.reg.cpsr.c, false);

        core.shiftamnt = 0x13; //#19
        core.barrel_shift();

        assert_eq!(core.barrelbus, 0b10110010110100000000000000000000);
        assert_eq!(core.reg.cpsr.c, false);

        /* LSR */
        core.barrelfunc = 1;
        core.barrel_shift();

        assert_eq!(core.barrelbus, 0b1010010111000);
        assert_eq!(core.reg.cpsr.c, false);

        /* ASR */
        core.barrelfunc = 2;
        core.barrel_shift();

        assert_eq!(core.barrelbus, 0b11111111111111111111010010111000);
        assert_eq!(core.reg.cpsr.c, false);

        /* ROR */
        core.barrelfunc = 3;
        core.barrel_shift();

        assert_eq!(core.barrelbus, 0b01110010110010110101010010111000);
        assert_eq!(core.reg.cpsr.c, false);

        /* RRX */
        core.shiftamnt = 0;
        core.barrel_shift();

        assert_eq!(core.barrelbus, 0b01010010111000011100101100101101);
        assert_eq!(core.reg.cpsr.c, false);
    }
}