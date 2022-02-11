#[path = "../src/arm7tdmi.rs"]
mod arm7tdmi;
#[path = "../src/decode.rs"]
mod decode;
#[path = "../src/exec.rs"]
mod exec;

mod bus {
    use std::io;
    use std::io::Read;
    use std::io::BufReader;
    use std::fs::File;

    pub struct Bus {
        pub mem: [u8; 8816]
    }

    impl Bus {
        pub fn new() -> Self {
            Bus {
                mem: [0; 8816]
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

        pub fn init_mem(&mut self) -> io::Result<()> {
            let f = File::open("../../gba-tests/arm/arm.gba")?;
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();
            
            // Read file into vector.
            reader.read_to_end(&mut buffer)?;
            
            println!("{}", buffer.len());

            for i in 0..buffer.len() {
                self.mem[i] = buffer[i];
            }
            Ok(())
        }
    }
}

pub fn test_inst(core: &mut arm7tdmi::Core, bus: &mut bus::Bus, inst: u32) {
    core.cycle = 0;

    println!("Inst: 0x{:x}, Type: {}, Dissassembly: {}, PC: 0x{:x}", inst, decode::decode_arm(inst), decode::disassemble_arm(inst), core.reg.gp[15]);

    let mut state = None;

    while state == None {
        println!("{}", core.reg);
        state = exec::step_arm(core, bus, inst);
        core.cycle += 1;
    }

    core.reg.gp[15] += 4;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arm() {
        let mut core = arm7tdmi::Core::new();
        let mut bus = bus::Bus::new();

        println!("{:?}", bus.init_mem());
        
        core.reg.write(13, 0x007f0003);

        let mut instructions = 200;
        
        let mut inst_same_counter = 0;

        let mut old_inst = 0;

        while instructions < 300 {
            let instruction = bus.mem_read_32(core.reg.gp[15] as usize);
            test_inst(&mut core, &mut bus, instruction);
            instructions -= 1;
            if old_inst == instruction {
                inst_same_counter += 1;
            }
            if inst_same_counter > 10 {
                panic!("Infinite loop")
            }
            old_inst = instruction;
        }

        assert_eq!(core.reg.gp[12], 0);
    }

}
