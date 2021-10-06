use bitpat::bitpat;

/* The status register */
#[derive(Clone, Copy, Debug)]
pub struct Status {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,

    irq_disable: bool,
    fiq_disable: bool,
    state: bool,
    mode: u8,
}

impl From<u32> for Status {
    fn from(word: u32) -> Status {
        Status {
            n: (1 << 31) & word != 0,
            z: (1 << 30) & word != 0,
            c: (1 << 29) & word != 0,
            v: (1 << 28) & word != 0,

            irq_disable: (1 << 7) & word != 0,
            fiq_disable: (1 << 6) & word != 0,
            state: (1 << 5) & word != 0,
            mode: (word & 0b11111) as u8,
        }
    }
}

impl From<Status> for u32 {
    fn from(sr: Status) -> u32 {
        let mut word: u32 = 0;

        word |= (sr.n as u32) << 31;
        word |= (sr.z as u32) << 30;
        word |= (sr.c as u32) << 29;
        word |= (sr.v as u32) << 28;

        word |= (sr.irq_disable as u32) << 7;
        word |= (sr.fiq_disable as u32) << 6;
        word |= (sr.state as u32) << 4;
        word |= sr.mode as u32;
        word
    }
}

impl Default for Status {
    fn default() -> Status {
        Status {
            n: false,
            z: false,
            c: false,
            v: false,

            irq_disable: false,
            fiq_disable: false,
            state: false,
            mode: 0,
        }
    }
}

/* Registers */
#[derive(Clone, Copy, Debug)]
pub struct Reg {
    pub gp:         [u32; 16],
    pub fiq:        [u32; 7],
    pub svc:        [u32; 2],
    pub abt:        [u32; 2],
    pub irq:        [u32; 2],
    pub und:        [u32; 2],

    pub cpsr:       Status,
    pub spsr_fiq:   Status,
    pub spsr_svc:   Status,
    pub spsr_abt:   Status,
    pub spsr_und:   Status,
    pub spsr_irq:   Status,

    pub address:    u32,
    pub data:       u32,
    pub shift:      u8,

    pub pipeline:   [u32; 3],
}

impl Default for Reg {
    fn default() -> Reg {
        Reg {
            gp:         [0; 16],
            fiq:        [0; 7],
            svc:        [0; 2],
            abt:        [0; 2],
            irq:        [0; 2],
            und:        [0; 2],

            cpsr:       Default::default(),
            spsr_fiq:   Default::default(),
            spsr_svc:   Default::default(),
            spsr_abt:   Default::default(),
            spsr_irq:   Default::default(),
            spsr_und:   Default::default(),

            address:    0,
            data:       0,
            shift:      0,

            pipeline:   [0; 3],
        }
    }
}

impl Reg {
    pub fn write(&mut self, index: usize, data: u32) {
        match self.cpsr.mode {
            0   =>  {
                self.gp[index] = data;
            },
            1   =>  {
                match index {
                    8 ..= 14    => self.fiq[index-8] = data,
                    _           => self.gp[index] = data,
                };
            },
            2   =>  {
                match index {
                    13 ..= 14   => self.svc[index-13] = data,
                    _           => self.gp[index] = data,
                };
            },
            3   =>  {
                match index {
                    13 ..= 14   => self.abt[index-13] = data,
                    _           => self.gp[index] = data,
                };
            }
            4   =>  {
                match index {
                    13 ..= 14   => self.irq[index-13] = data,
                    _           => self.gp[index] = data,
                };
            }
            5   =>  {
                match index {
                    13 ..= 14   => self.und[index-13] = data,
                    _           => self.gp[index] = data,
                };
            }
            _   =>  unreachable!()
        }
    }

    pub fn read(&mut self, index: usize) -> u32 {
        match self.cpsr.mode {
            0   =>  {
                self.gp[index]
            },
            1   =>  {
                match index {
                    8 ..= 14    => self.fiq[index-8],
                    _           => self.gp[index],
                }
            },
            2   =>  {
                match index {
                    13 ..= 14   => self.svc[index-13],
                    _           => self.gp[index],
                }
            },
            3   =>  {
                match index {
                    13 ..= 14   => self.abt[index-13],
                    _           => self.gp[index],
                }
            }
            4   =>  {
                match index {
                    13 ..= 14   => self.irq[index-13],
                    _           => self.gp[index],
                }
            }
            5   =>  {
                match index {
                    13 ..= 14   => self.und[index-13],
                    _           => self.gp[index],
                }
            }
            _   =>  unreachable!()
        }
    }
}

/* The processor core */
#[derive(Clone, Copy, Debug)]
pub struct Core {
    pub reg: Reg,

    pub alubus: u32,
    pub aluop: u8,
    pub setcond: bool,

    pub addrbus: u32,
    pub incbus: u32,

    pub asel: u32,
    pub abus: u32,
    pub bsel: u32,
    pub bbus: u32,

    pub writesel: u32,
    pub writefrom: u8,

    pub barrelfunc: u8,
    pub shiftamnt: u32,
    pub barrelbus: u32,

    pub databus: u32,
    pub instbus: u32,
}

impl Core {
    pub fn new() -> Self {
        Core {
            reg: Default::default(),

            alubus: 0,
            aluop: 0,
            setcond: false,

            addrbus: 0,
            incbus: 0,

            asel: 0,
            abus: 0,
            bsel: 0,
            bbus: 0,

            writesel: 0,
            writefrom: 0,

            barrelfunc: 0,
            shiftamnt: 0,
            barrelbus: 0,

            databus: 0,
            instbus: 0,
        }
    }

    fn set_zn(&mut self, result: u32) {
        self.reg.cpsr.z = result == 0;
        self.reg.cpsr.n = (result as i32) < 0;
    }

    fn set_vc_add(&mut self, op1: u32, op2: u32) {
        self.reg.cpsr.v = (op1 as i32).overflowing_add(op2 as i32).1;
        self.reg.cpsr.c = op1.overflowing_add(op2).1;
    }

    fn set_vc_sub(&mut self, op1: u32, op2: u32) {
        self.reg.cpsr.v = (op1 as i32).overflowing_sub(op2 as i32).1;
        self.reg.cpsr.c = op1.overflowing_sub(op2).1;
    }

    pub fn alu(&mut self) {
        let tmp = match self.aluop {
            0   =>  self.abus & self.barrelbus,
            1   =>  self.abus ^ self.barrelbus,
            2   =>  self.abus.wrapping_sub(self.barrelbus),
            3   =>  self.barrelbus.wrapping_sub(self.abus),
            4   =>  self.abus.wrapping_add(self.barrelbus),
            5   =>  self.abus.wrapping_add(self.barrelbus.wrapping_add(self.reg.cpsr.c as u32)),
            6   =>  self.abus.wrapping_sub(self.barrelbus.wrapping_add(!self.reg.cpsr.c as u32)),
            7   =>  self.barrelbus.wrapping_sub(self.abus.wrapping_add(!self.reg.cpsr.c as u32)),
            8   =>  self.abus & self.barrelbus,
            9   =>  self.abus ^ self.barrelbus,
            10  =>  self.abus.wrapping_sub(self.barrelbus),
            11  =>  self.abus.wrapping_add(self.barrelbus),
            12  =>  self.abus | self.barrelbus,
            13  =>  self.barrelbus,
            14  =>  self.abus & !self.barrelbus,
            15  =>  !self.barrelbus,
            _   =>  unreachable!()
        };
        /* Condition codes */
        if self.setcond {
            match self.aluop {
                0 | 1 | 8 | 9 | 12 | 13 | 14 | 15 => self.set_zn(tmp),
                4 | 5 | 11 => { self.set_zn(tmp); self.set_vc_add(self.abus, self.barrelbus); },
                3 | 7 => { self.set_zn(tmp); self.set_vc_sub(self.barrelbus, self.abus); },
                2 | 6 | 10 => {self.set_zn(tmp); self.set_vc_sub(self.abus, self.barrelbus); },
                _   =>  unreachable!()
            }
        }
        /* 'Register' write-back */
        if self.aluop < 8 || self.aluop > 0xB {
            self.alubus = tmp
        }
    }

    pub fn decode_shift(&mut self, shift: u32) {
        let shifttype: u8 = ((shift & 0x6) >> 1) as u8;
        self.barrelfunc = shifttype;
        if bitpat!( _ _ _ _ _ _ _ 0 )(shift) {
            self.shiftamnt = shift >> 3;
        } else if bitpat!( _ _ _ _ 0 _ _ 1 )(shift) {
            self.shiftamnt = self.reg.read((shift >> 4) as usize) & 0x1F;
        } else { panic!("shift mode does not exist")}
    }

    pub fn barrel_shift(&mut self) {
        self.barrelbus = match self.barrelfunc {
            0   =>  self.bbus << self.shiftamnt,
            1   =>  self.bbus >> self.shiftamnt,
            2   =>  ((self.bbus as i32) >> self.shiftamnt) as u32,
            3   =>  {if self.shiftamnt == 0 {(self.bbus >> 1) | ((self.reg.cpsr.c as u32) << 31)} else {self.bbus.rotate_right(self.shiftamnt)}},
            _   =>  unreachable!()
        };

        if self.setcond {
            self.reg.cpsr.c = match self.barrelfunc {
                0   =>  (self.bbus >> 32-self.shiftamnt) & 0b1,
                1   =>  (self.bbus >> self.shiftamnt-1) & 0b1,
                2   =>  ((self.bbus as i32) >> (self.shiftamnt - 1)) as u32 & 0b1,
                3   =>  if self.shiftamnt == 0 {self.bbus & 0b1} else {(self.bbus >> ((self.shiftamnt - 1) & 0x1F)) & 0b1}
                _   =>  unreachable!()
            } != 0;
        }
    }

    pub fn reg_bank(&mut self) {
        self.abus = self.reg.read(self.asel as usize);
        self.bbus = self.reg.read(self.bsel as usize);
        self.reg.write(self.writesel as usize, match self.writefrom {
            0   =>  self.alubus,
            1   =>  self.incbus,
            _   =>  unreachable!()
        });
    }
}