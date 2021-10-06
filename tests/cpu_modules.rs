#[path = "../src/arm7tdmi.rs"]
mod arm7tdmi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alu_functions() {
        let mut core = arm7tdmi::Core::new();

        core.abus = 0x01001123;
        core.barrelbus = 0x10321067;

        core.aluop = 4;
        core.setcond = true;

        core.alu();

        assert_eq!(core.alubus, 0x1132218A);
        assert_eq!(u32::from(core.reg.cpsr), 0x0);

        core.abus = 3;
        core.barrelbus = 3;

        core.aluop = 9;
        core.alubus = 0;

        core.alu();
        
        assert_eq!(core.alubus, 0);
        assert_eq!(u32::from(core.reg.cpsr), 0x40000000);
    }
}