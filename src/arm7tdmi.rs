use bitpat::bitpat;
use std::fmt;

/* 
Things that need to be added to this:
    Some sort of sign-extension module, which modifies the incoming data using the contents of the S and H bits in an applicable instruction word (possibly in the read data register)
*/

/* The status register */
#[derive(Clone, Copy, Debug)]
pub struct Status {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,

    irq_disable: bool,
    fiq_disable: bool,
    pub state: bool,
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

        /* logical OR each of the condition bits into the condition word */
        word |= (sr.n as u32) << 31;
        word |= (sr.z as u32) << 30;
        word |= (sr.c as u32) << 29;
        word |= (sr.v as u32) << 28;

        word |= (sr.irq_disable as u32) << 7;
        word |= (sr.fiq_disable as u32) << 6;
        word |= (sr.state as u32) << 4;
        word |= sr.mode as u32;
        word // return the condition word
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
    #[allow(dead_code)]
    pub fn write(&mut self, index: usize, data: u32) { /* Write to a register */
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

    pub fn read(&mut self, index: usize) -> u32 {   /* Read from a register */
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

    pub fn transfer_spsr(&mut self) {
        self.cpsr = match self.cpsr.mode {
            0 => self.cpsr,
            1 => self.spsr_fiq,
            2 => self.spsr_svc,
            3 => self.spsr_abt,
            4 => self.spsr_irq,
            5 => self.spsr_und,
            _ => panic!("Mode {} does not exist", self.cpsr.mode)
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
    pub datareg: u32,
    pub instbus: u32,

    pub transferblock: [u32; 16],
    pub multicycle: u8,

    pub cycle: u8,
}

/* Print-formatting so that I can print the contents of the structure */
impl fmt::Display for Core {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, 
            "
            alubus:     {}\n
            aluop:      {}\n
            setcond:    {}\n
            addrbus:    {}\n
            incbus:     {}\n
            asel:       {}\n
            abus:       {}\n
            bsel:       {}\n
            bbus:       {}\n
            writesel:   {}\n
            writefrom:  {}\n
            barrelfunc: {}\n
            shiftamnt:  {}\n
            barrelbus:  {}\n
            databus:    {}\n
            instbus:    {}\n
            cycle:      {}
            "
        ,   self.alubus,
            self.aluop,
            self.setcond,
            self.addrbus,
            self.incbus,
            self.asel,
            self.abus,
            self.bsel,
            self.bbus,
            self.writesel,
            self.writefrom,
            self.barrelfunc,
            self.shiftamnt,
            self.barrelbus,
            self.databus,
            self.instbus,
            self.cycle
        )
    }
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
            datareg: 0,
            instbus: 0,

            transferblock: [0; 16],
            multicycle: 0,
            
            cycle: 0,
        }
    }

    #[allow(dead_code)]
    pub fn inc_cycle(&mut self) {
        self.cycle += 1;
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

    /* Instruction condition codes */
    #[allow(dead_code)]
    pub fn cond_codes(&mut self, code: u32) -> bool {
        match code { //match condition code
            0x0 =>  self.reg.cpsr.z == true,
            0x1 =>  self.reg.cpsr.z == false,
            0x2 =>  self.reg.cpsr.c == true,
            0x3 =>  self.reg.cpsr.c == false,
            0x4 =>  self.reg.cpsr.n == true,
            0x5 =>  self.reg.cpsr.n == false,
            0x6 =>  self.reg.cpsr.v == true,
            0x7 =>  self.reg.cpsr.v == false,
            0x8 =>  self.reg.cpsr.c == true && self.reg.cpsr.z == false,
            0x9 =>  self.reg.cpsr.c == false && self.reg.cpsr.z == true,
            0xA =>  self.reg.cpsr.n == self.reg.cpsr.v,
            0xB =>  self.reg.cpsr.n != self.reg.cpsr.v,
            0xC =>  self.reg.cpsr.z == false && (self.reg.cpsr.n == self.reg.cpsr.v),
            0xD =>  self.reg.cpsr.z == true || (self.reg.cpsr.n != self.reg.cpsr.v),
            0xE =>  true,
            _   =>  panic!("Condition code: {} does not exist", code)
        } //return true if condition is met
    }

    #[allow(dead_code)]
    pub fn fetch(&mut self) {
        /* Shift instructions down in the pipeline */
        self.reg.pipeline[2] = self.reg.pipeline[1];
        self.reg.pipeline[1] = self.reg.pipeline[0];
        self.reg.pipeline[0] = self.databus;        //get new instruction from the data bus
    }

    pub fn alu(&mut self) { /* The alu fuctions */
        let tmp = match self.aluop { /* match the alu operation */
            0   =>  self.abus & self.barrelbus, //AND
            1   =>  self.abus ^ self.barrelbus, //XOR
            2   =>  self.abus.wrapping_sub(self.barrelbus), //SUB
            3   =>  self.barrelbus.wrapping_sub(self.abus), //
            4   =>  self.abus.wrapping_add(self.barrelbus), //ADD
            5   =>  self.abus.wrapping_add(self.barrelbus.wrapping_add(self.reg.cpsr.c as u32)), //ADC
            6   =>  self.abus.wrapping_sub(self.barrelbus.wrapping_add(!self.reg.cpsr.c as u32)), //SBC
            7   =>  self.barrelbus.wrapping_sub(self.abus.wrapping_add(!self.reg.cpsr.c as u32)),
            8   =>  self.abus & self.barrelbus, //AND
            9   =>  self.abus ^ self.barrelbus, //XOR
            10  =>  self.abus.wrapping_sub(self.barrelbus),
            11  =>  self.abus.wrapping_add(self.barrelbus),
            12  =>  self.abus | self.barrelbus, //XOR
            13  =>  self.barrelbus, //MOV
            14  =>  self.abus & !self.barrelbus, //NAND
            15  =>  !self.barrelbus, //NOT
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
        /* Register write-back */
        if self.aluop < 8 || self.aluop > 0xB {
            self.alubus = tmp
        }
    }

    #[allow(dead_code)]
    pub fn mul(&mut self) {
        /* The multiply instruction */
    }

    /* decode shift opcode */
    #[allow(dead_code)]
    pub fn decode_shift(&mut self, shift: u32) {
        let shifttype: u8 = ((shift & 0x6) >> 1) as u8;
        self.barrelfunc = shifttype;
        if bitpat!( _ _ _ _ 0 _ _ 1 )(shift) {
            println!("Shift register");
            self.shiftamnt = self.reg.read((shift >> 4) as usize) & 0x1F;
        } else if bitpat!( _ _ _ _ _ _ _ 0 )(shift) {
            self.shiftamnt = shift >> 3;
            println!("Shift immediate");
        } else { panic!("shift mode does not exist") }
    }

    #[allow(dead_code)]
    pub fn decode_shift_imm(&mut self, mut shift: u32) {
        shift = (shift >> 8) & 0xF;
        self.barrelfunc = 3;
        self.shiftamnt = shift.rotate_right(2*shift);
    }

    /* the barrel shifter */
    pub fn barrel_shift(&mut self) {
        self.barrelbus = match self.barrelfunc { /* Barrel shifter function */
            0   =>  self.bbus << self.shiftamnt, /* LSL */
            1   =>  self.bbus >> self.shiftamnt, /* LSR */
            2   =>  ((self.bbus as i32) >> self.shiftamnt) as u32, /* ASR */
            3   =>  {if self.shiftamnt == 0 {(self.bbus >> 1) | ((self.reg.cpsr.c as u32) << 31)} else {self.bbus.rotate_right(self.shiftamnt)}}, /* ROR, RRX */
            _   =>  unreachable!()
        };

        if self.setcond { /* set carry flag */
            self.reg.cpsr.c = match self.barrelfunc { /* match function */
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

    #[allow(dead_code)]
    pub fn calc_reg_transfer(&mut self, rlist: u32) {
        self.multicycle = 0;
        println!("0b{:0>16b}", rlist);
        for x in 0..16 {
            if (rlist>>x) & 0b1 == 0b1 {
                println!("writing to list");
                self.transferblock[self.multicycle as usize] = x as u32;
                self.multicycle += 1;
            }
        }
    }
}