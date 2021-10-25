/* Memory Map */
/* 
  00000000-00003FFF   BIOS - System ROM         (16 KBytes)
  00004000-01FFFFFF   Not used
  02000000-0203FFFF   WRAM - On-board Work RAM  (256 KBytes) 2 Wait
  02040000-02FFFFFF   Not used
  03000000-03007FFF   WRAM - On-chip Work RAM   (32 KBytes)
  03008000-03FFFFFF   Not used
  04000000-040003FE   I/O Registers
  04000400-04FFFFFF   Not used
  05000000-050003FF   BG/OBJ Palette RAM        (1 Kbyte)
  05000400-05FFFFFF   Not used
  06000000-06017FFF   VRAM - Video RAM          (96 KBytes)
  06018000-06FFFFFF   Not used
  07000000-070003FF   OAM - OBJ Attributes      (1 Kbyte)
  07000400-07FFFFFF   Not used
  08000000-09FFFFFF   Game Pak ROM/FlashROM (max 32MB) - Wait State 0
  0A000000-0BFFFFFF   Game Pak ROM/FlashROM (max 32MB) - Wait State 1
  0C000000-0DFFFFFF   Game Pak ROM/FlashROM (max 32MB) - Wait State 2
  0E000000-0E00FFFF   Game Pak SRAM    (max 64 KBytes) - 8bit Bus width
  0E010000-0FFFFFFF   Not used
  10000000-FFFFFFFF   Not used (upper 4bits of address bus unused)
*/

const MEM_START: usize    = 0x00000000;
const BIOS_END: usize     = 0x00003FFF;
const WRAM0_START: usize  = 0x02000000;
const WRAM0_END: usize    = 0x0203FFFF;
const WRAM1_START: usize  = 0x03000000;
const WRAM1_END: usize    = 0x03007FFF;
const IO_START: usize     = 0x04000000;
const IO_END: usize       = 0x040003FE;
const OBJ_START: usize    = 0x05000000;
const OBJ_END: usize      = 0x050003FF;
const VRAM_START: usize   = 0x06000000;
const VRAM_END: usize     = 0x06017FFF;
const OAM_START: usize    = 0x07000000;
const OAM_END: usize      = 0x070003FF;
const GPK0_START: usize   = 0x08000000;
/* Need to include code to handle wait-states */
#[allow(dead_code)] //for now
const GPK0_END: usize     = 0x09FFFFFF;
#[allow(dead_code)]
const GPK1_START: usize   = 0x0A000000;
#[allow(dead_code)]
const GPK1_END: usize     = 0x0BFFFFFF;
#[allow(dead_code)]
const GPK2_START: usize   = 0x0C000000;
const GPK2_END: usize     = 0x0DFFFFFF;
const GPKSRAM_START: usize= 0x0E000000;
const GPKSRAM_END: usize  = 0x0E00FFFF;

#[derive(Clone,Copy,Debug)]
pub struct Bus {
	bios: 	[u8; BIOS_END-MEM_START],
	wram0:	[u8; WRAM0_END-WRAM0_START],
	wram1:	[u8; WRAM1_END-WRAM1_START],
	io:	    [u8; IO_END-IO_START],
	obj:	[u8; OBJ_END-OBJ_START],
	vram:	[u8; VRAM_END-VRAM_START],
	oam:	[u8; OAM_END-OAM_START],
	gpk:	[u8; GPK2_END-GPK0_START],
	gsrm:	[u8; GPKSRAM_END-GPKSRAM_START],
}

#[allow(dead_code)]
impl Bus {
    pub fn new() -> Self {
        Bus {
            bios: 	[0; BIOS_END-MEM_START],
            wram0:	[0; WRAM0_END-WRAM0_START],
            wram1:	[0; WRAM1_END-WRAM1_START],
            io:	    [0; IO_END-IO_START],
            obj:	[0; OBJ_END-OBJ_START],
            vram:	[0; VRAM_END-VRAM_START],
            oam:	[0; OAM_END-OAM_START],
            gpk:	[0; GPK2_END-GPK0_START],
            gsrm:	[0; GPKSRAM_END-GPKSRAM_START],
        }
    }

    pub fn mem_read(&self, addr: usize) -> u8 {
        match addr {
            MEM_START ..= BIOS_END => {
                self.bios[addr]
            },
            WRAM0_START ..= WRAM0_END => {
                self.wram0[(addr - WRAM0_START)]
            },
            WRAM1_START ..= WRAM1_END => {
                self.wram1[(addr - WRAM1_START)]
            },
            IO_START ..= IO_END => {
                self.io[(addr - IO_START)]
            },
            OBJ_START ..= OBJ_END => {
                self.obj[(addr - OBJ_START)]
            },
            VRAM_START ..= VRAM_END => {
                self.vram[(addr - VRAM_START)]
            },
            OAM_START ..= OAM_END => {
                self.oam[(addr - OAM_START)]
            },
            GPK0_START ..= GPK2_END => {
                self.gpk[(addr - GPK0_START)]
            },
            GPKSRAM_START ..= GPKSRAM_END => {
                self.gsrm[(addr - GPKSRAM_START)]
            },
            _   => panic!("Illegal memory read at {}", addr)
        }
    }

    pub fn mem_write(&mut self, addr: usize, data: u8) {
        match addr {
            MEM_START ..= BIOS_END => {
                self.bios[addr] = data;
            },
            WRAM0_START ..= WRAM0_END => {
                self.wram0[(addr - WRAM0_START)] = data;
            },
            WRAM1_START ..= WRAM1_END => {
                self.wram1[(addr - WRAM1_START)] = data;
            },
            IO_START ..= IO_END => {
                self.io[(addr - IO_START)] = data;
            },
            OBJ_START ..= OBJ_END => {
                self.obj[(addr - OBJ_START)] = data;
            },
            VRAM_START ..= VRAM_END => {
                self.vram[(addr - VRAM_START)] = data;
            },
            OAM_START ..= OAM_END => {
                self.oam[(addr - OAM_START)] = data;
            },
            GPK0_START ..= GPK2_END => {
                self.gpk[(addr - GPK0_START)] = data;
            },
            GPKSRAM_START ..= GPKSRAM_END => {
                self.gsrm[(addr - GPKSRAM_START)] = data;
            },
            _   => panic!("Illegal memory write at {}", addr)
        }
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
