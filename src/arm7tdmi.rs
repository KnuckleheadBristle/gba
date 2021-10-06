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

pub const SP: usize = 13;
pub const LR: usize = 14;
pub const PC: usize = 15;
/* Registers */
#[derive(Clone, Copy, Debug)]
pub struct Reg {
    pub gp: [u32; 16],
    pub cpsr: Status,
    
    pub address: u32,
    pub data: u32,
    pub shift: u8,

    pub pipeline: [u32; 3],

    pub sp_fiq: u32,
    pub lr_fiq: u32,
    pub spsr_fiq: Status,

    pub sp_svc: u32,
    pub lr_svc: u32,
    pub spsr_svc: Status,

    pub sp_abt: u32,
    pub lr_abt: u32,
    pub spsr_abt: Status,

    pub sp_irq: u32,
    pub lr_irq: u32,
    pub spsr_irq: Status,

    pub sp_und: u32,
    pub lr_und: u32,
    pub spsr_und: Status,
}

impl Default for Reg {
    fn default() -> Reg {
        Reg {
            gp: [0; 16],
            cpsr: Default::default(),
            
            address: 0,
            data: 0,
            shift: 0,

            pipeline: [0; 3],

            sp_fiq: 0,
            lr_fiq: 0,
            spsr_fiq: Default::default(),

            sp_svc: 0,
            lr_svc: 0,
            spsr_svc: Default::default(),

            sp_abt: 0,
            lr_abt: 0,
            spsr_abt: Default::default(),

            sp_irq: 0,
            lr_irq: 0,
            spsr_irq: Default::default(),

            sp_und: 0,
            lr_und: 0,
            spsr_und: Default::default(),
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
    pub abus: u32,
    pub bbus: u32,

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
            abus: 0,
            bbus: 0,

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

    pub fn barrel_shift(&mut self) {
        self.barrelbus = match self.barrelfunc {
            0   =>  self.bbus >> self.shiftamnt,
            1   =>  self.bbus << self.shiftamnt,
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
}