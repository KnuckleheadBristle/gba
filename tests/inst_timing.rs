#[path = "../src/arm7tdmi.rs"]
mod arm7tdmi;
#[path = "../src/decode.rs"]
mod decode;
#[path = "../src/exec.rs"]
mod exec;

mod bus {
    pub struct Bus {
        mem: [u8; 0x400]
    }

    impl Bus {
        pub fn new() -> Self {
            Bus {
                mem: [0; 0x400]
            }
        }

        pub fn mem_read(&self, addr: usize) -> u8 {
            self.mem[addr]
        }

        pub fn mem_write(&mut self, addr: usize, data: u8) {
            self.mem[addr] = data;
        }

        pub fn mem_read_16(&mut self, addr: usize) -> u16 {
            let lo = self.mem_read(addr) as u16;
            let hi = self.mem_read(addr+1) as u16;
            lo | (hi << 8)
        }
    
        pub fn mem_read_32(&mut self, addr: usize) -> u32 {
            let lo = self.mem_read(addr) as u32;
            let lo1 = self.mem_read(addr+1) as u32;
            let hi = self.mem_read(addr+2) as u32;
            let hi1 = self.mem_read(addr+3) as u32;
            lo | (lo1 << 8) | (hi << 16) | (hi1 << 24)
        }
    
        pub fn mem_write_16(&mut self, addr: usize, data: u16) {
            let lo = (data & 0xFF) as u8;
            let hi = (data >> 8) as u8;
            self.mem_write(addr, lo);
            self.mem_write(addr+1, hi);
        }
    
        pub fn mem_write_32(&mut self, addr: usize, data: u32) {
            let lo = (data & 0xFF) as u8;
            let lo1 = ((data >> 8) & 0xFF) as u8;
            let hi = ((data >> 16) & 0xFF) as u8;
            let hi1 = ((data >> 24) & 0xFF) as u8;
            self.mem_write(addr, lo);
            self.mem_write(addr+1, lo1);
            self.mem_write(addr+2, hi);
            self.mem_write(addr+3, hi1);
        }
    }
}

pub fn test_inst(core: &mut arm7tdmi::Core, bus: &mut bus::Bus, inst: u32) -> u8 {
    let mut cycles = 0;
    core.cycle = 0;

    let mut state = None;

    while state == None {
        state = exec::step_arm(core, bus, inst);
        core.cycle += 1;
        cycles += 1;
    }

    cycles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_proc_cycle_timing() {
        let mut core = arm7tdmi::Core::new();
        let mut bus = bus::Bus::new();

        let instruction = 0xE0922001; //normal
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 1);

        let instruction = 0xE0922301; //shift
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 2);

        let instruction = 0xE092F001; //dest=pc
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 3);

        let instruction = 0xE092F301; //shift, dest=pc
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 4);
    }

    #[test]
    fn branch_and_exchange_cycle_timing() {
        let mut core = arm7tdmi::Core::new();
        let mut bus = bus::Bus::new();

        let instruction = 0xE12FFF12;
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 3);
    }

    #[test]
    fn branch_cycle_timing() {
        let mut core = arm7tdmi::Core::new();
        let mut bus = bus::Bus::new();

        let instruction = 0xEB14A94C;
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 3);
    }

    #[test]
    fn single_transfer_cycle_timing() {
        let mut core = arm7tdmi::Core::new();
        let mut bus = bus::Bus::new();

        /* Load register */
        let instruction = 0xE5B222D5;
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 3);

        let instruction = 0xE5B2F2D5; //dest=pc
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 5);

        /* Store register */
        let instruction = 0xE5A222D5;
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 2);
    }

    #[test]
    fn single_data_swap_cycle_timing() {
        let mut core = arm7tdmi::Core::new();
        let mut bus = bus::Bus::new();

        /* Load register */
        let instruction = 0xE1028092;
        assert_eq!(test_inst(&mut core, &mut bus, instruction), 4);
    }
}
