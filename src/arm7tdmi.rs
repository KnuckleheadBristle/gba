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

/* The default values for when a new Status object is instantiated */
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

/* The same as Status, defining the default values. */
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

/* The different functions of the Reg object to facilitiate different operations */
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

    #[allow(dead_code)]
    pub fn transfer_spsr(&mut self) { /* Transfer the status register */
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

    pub fn read_psr(&mut self, a: u32) -> u32 {
        return if a==0 {
            self.cpsr
        } else {
            match self.cpsr.mode {
                0 => self.cpsr,
                1 => self.spsr_fiq,
                2 => self.spsr_svc,
                3 => self.spsr_abt,
                4 => self.spsr_irq,
                5 => self.spsr_und,
                _ => panic!("Mode {} does not exist", self.cpsr.mode)
            }
        }.into();
    }

    #[allow(dead_code)]
    pub fn write_psr(&mut self, a: u32, data: u32) {
        let data: Status = data.into();
        if a==0 {
            self.cpsr = data;
        } else {
            match self.cpsr.mode {
                0 => self.cpsr = data,
                1 => self.spsr_fiq = data,
                2 => self.spsr_svc = data,
                3 => self.spsr_abt = data,
                4 => self.spsr_irq = data,
                5 => self.spsr_und = data,
                _ => panic!("Mode {} does not exist", self.cpsr.mode)
            };
        }
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        write!(f,
            "R0: {:08x}, R1: {:08x}, R2:  {:08x}, R3:  {:08x}, R4:  {:08x}, R5:  {:08x}, R6:  {:08x}, R7:  {:08x},\nR8: {:08x}, R9: {:08x}, R10: {:08x}, R11: {:08x}, R12: {:08x}, R13: {:08x}, R14: {:08x}, R15: {:08x}"
            ,
            self.gp[0],
            self.gp[1],
            self.gp[2],
            self.gp[3],
            self.gp[4],
            self.gp[5],
            self.gp[6],
            self.gp[7],
            self.gp[8],
            self.gp[9],
            self.gp[10],
            self.gp[11],
            self.gp[12],
            self.gp[13],
            self.gp[14],
            self.gp[15],
        )
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

    pub cycle: u8, //The current instruction cycle (is reset after each instruction)
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

/* The processor 'Core' itself */
impl Core {
    pub fn new() -> Self {
        Core {
            reg: Default::default(), //We need some registers

            /* There are all internal busses used when executing instructions */
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
    pub fn inc_cycle(&mut self) { //increment the instruction cycle
        self.cycle += 1;
    }

    fn set_zn(&mut self, result: u32) { // Set the zero and negative flags
        self.reg.cpsr.z = result == 0;
        self.reg.cpsr.n = (result as i32) < 0;
    }

    fn set_vc_add(&mut self, op1: u32, op2: u32) { // Set the half and full carry flags for an add operation
        self.reg.cpsr.v = (op1 as i32).overflowing_add(op2 as i32).1;
        self.reg.cpsr.c = op1.overflowing_add(op2).1;
    }

    fn set_vc_sub(&mut self, op1: u32, op2: u32) { // As above but for a subtraction operation
        self.reg.cpsr.v = (op1 as i32).overflowing_sub(op2 as i32).1;
        self.reg.cpsr.c = op1.overflowing_sub(op2).1;
    }

    /* Instruction condition codes */
    #[allow(dead_code)]
    pub fn cond_codes(&mut self, code: u32) -> bool {
        match code { //match condition code
            0x0 =>  self.reg.cpsr.z == true,                                            //If the zero flag is set
            0x1 =>  self.reg.cpsr.z == false,                                           //If the zero flag is cleared
            0x2 =>  self.reg.cpsr.c == true,                                            //If the carry flag is set
            0x3 =>  self.reg.cpsr.c == false,                                           //If the carry flag is cleared
            0x4 =>  self.reg.cpsr.n == true,                                            //If the negative flag is set
            0x5 =>  self.reg.cpsr.n == false,                                           //If the negative flag is cleared
            0x6 =>  self.reg.cpsr.v == true,                                            //If the half-carry flag is set
            0x7 =>  self.reg.cpsr.v == false,                                           //If the half-carry flag is cleared
            0x8 =>  self.reg.cpsr.c == true && self.reg.cpsr.z == false,                //If carry is set and zero is cleared
            0x9 =>  self.reg.cpsr.c == false && self.reg.cpsr.z == true,                //If carry is cleared and zero is set
            0xA =>  self.reg.cpsr.n == self.reg.cpsr.v,                                 //If negative equals half-carry
            0xB =>  self.reg.cpsr.n != self.reg.cpsr.v,                                 //If negative does not equal half-carry
            0xC =>  self.reg.cpsr.z == false && (self.reg.cpsr.n == self.reg.cpsr.v),   //If zero is cleared and negative equals half-carry
            0xD =>  self.reg.cpsr.z == true || (self.reg.cpsr.n != self.reg.cpsr.v),    //If zero is set or negative does not equal half-carry
            0xE =>  true,                                                               //Always
            _   =>  false                                                               //No more condition codes! (never)
        } //return true if condition is met
    }

    #[allow(dead_code)]
    pub fn fetch(&mut self) {                                                           //Fetch the next instruction, updating the pipeline
        /* Shift instructions down in the pipeline */
        self.reg.pipeline[2] = self.reg.pipeline[1];
        self.reg.pipeline[1] = self.reg.pipeline[0];
        self.reg.pipeline[0] = self.datareg;       //get new instruction from the data bus
    }

    pub fn alu(&mut self) { /* The alu fuctions */
        let tmp = match self.aluop { /* match the alu operation */
            0   =>  self.abus & self.barrelbus,                                                     //AND
            1   =>  self.abus ^ self.barrelbus,                                                     //EOR
            2   =>  self.abus.wrapping_sub(self.barrelbus),                                         //SUB
            3   =>  self.barrelbus.wrapping_sub(self.abus),                                         //RSB
            4   =>  self.abus.wrapping_add(self.barrelbus),                                         //ADD
            5   =>  self.abus.wrapping_add(self.barrelbus.wrapping_add(self.reg.cpsr.c as u32)),    //ADC
            6   =>  self.abus.wrapping_sub(self.barrelbus.wrapping_add(!self.reg.cpsr.c as u32)),   //SBC
            7   =>  self.barrelbus.wrapping_sub(self.abus.wrapping_add(!self.reg.cpsr.c as u32)),   //RSC
            8   =>  self.abus & self.barrelbus,                                                     //TST
            9   =>  self.abus ^ self.barrelbus,                                                     //TEQ
            10  =>  self.abus.wrapping_sub(self.barrelbus),                                         //CMP
            11  =>  self.abus.wrapping_add(self.barrelbus),                                         //CMN
            12  =>  self.abus | self.barrelbus,                                                     //ORR
            13  =>  self.barrelbus,                                                                 //MOV
            14  =>  self.abus & !self.barrelbus,                                                    //BIC
            15  =>  !self.barrelbus,                                                                //MVN
            _   =>  unreachable!()
        };

        /* Condition codes */
        if self.setcond {
            match self.aluop {
                0 | 1 | 8 | 9 | 12 | 13 | 14 | 15 => self.set_zn(tmp),                              //AND, EOR, TST, TEQ, ORR, MOV, BIC, MVN
                4 | 5 | 11 => { self.set_zn(tmp); self.set_vc_add(self.abus, self.barrelbus); },    //ADD, ADC
                3 | 7 => { self.set_zn(tmp); self.set_vc_sub(self.barrelbus, self.abus); },         //SUB, RSC
                2 | 6 | 10 => {self.set_zn(tmp); self.set_vc_sub(self.abus, self.barrelbus); },     //EOR, SBC, CMP
                _   =>  unreachable!()
            }
        }
        /* Register write-back */
        if self.aluop < 8 || self.aluop > 0xB {                                                     //not TST, TEQ, CMP, CMN
            self.alubus = tmp
        }
    }

    #[allow(dead_code)]
    pub fn mul(&mut self) {
        /* The multiply instruction */

        /*
        This is not done because there is little to no information about how the multiply hardware actually works in the ARM7TDMI.
        The only real information that is a available is that it is an 8x32 booth multiplier, and it takes some form of
        data input from the a and b busses (of which both seem to be bidirectional)

        Even the carry output of this function is deemed 'undefined'. If only ARM actually documented their secrets.
        */
    }

    /* decode shift opcode */
    pub fn decode_shift(&mut self, shift: u32) {
        let shifttype: u8 = ((shift & 0x6) >> 1) as u8;
        self.barrelfunc = shifttype;
        if bitpat!( _ _ _ _ 0 _ _ 1 )(shift) {
            self.shiftamnt = self.reg.read((shift >> 4) as usize) & 0x1F;
        } else if bitpat!( _ _ _ _ _ _ _ 0 )(shift) {
            self.shiftamnt = shift >> 3;
        } else { panic!("shift mode does not exist") }
    }

    /* Decode immediate shift opcode */
    pub fn decode_shift_imm(&mut self, mut shift: u32) {
        shift = (shift >> 8) & 0xF;
        self.barrelfunc = 3;
        self.shiftamnt = shift.rotate_right(2*shift);
    }

    /* the barrel shifter */
    pub fn barrel_shift(&mut self) {
        self.barrelbus = match self.barrelfunc { /* Barrel shifter function */
            0   =>  self.bbus << self.shiftamnt,                                                                                                    //LSL 
            1   =>  self.bbus >> self.shiftamnt,                                                                                                    //LSR 
            2   =>  ((self.bbus as i32) >> self.shiftamnt) as u32,                                                                                  //ASR 
            3   =>  {if self.shiftamnt == 0 {(self.bbus >> 1) | ((self.reg.cpsr.c as u32) << 31)} else {self.bbus.rotate_right(self.shiftamnt)}},   //ROR, RRX 
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

    /* Update the register bank */
    pub fn reg_bank(&mut self) {
        self.abus = self.reg.read(self.asel as usize);
        self.bbus = self.reg.read(self.bsel as usize);
        self.reg.write(self.writesel as usize, match self.writefrom {
            0   =>  self.alubus,
            1   =>  self.incbus,
            _   =>  unreachable!()
        });
    }

    /* Helper function for a block register transfer*/
    #[allow(dead_code)]
    pub fn calc_reg_transfer(&mut self, rlist: u32) {
        self.multicycle = 0;
        self.transferblock = [0; 16];

        for x in 0..16 {
            if (rlist>>(15-x)) & 0b1 == 0b1 {
                self.transferblock[self.multicycle as usize] = (15-x) as u32;
                self.multicycle += 1;
            }
        }
    }
}
