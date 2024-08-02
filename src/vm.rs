use crate::memory::{Addressable, LinearMemory};

#[repr(u8)]
pub enum Register {
    A,
    B,
    C,
    M,
    SP,
    PC,
    BP,
    Flags,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::M,
            4 => Self::SP,
            5 => Self::PC,
            6 => Self::BP,
            7 => Self::Flags,
            _ => panic!("invalid register"),
        }
    }
}

#[repr(u8)]
pub enum Op {
    Nop,
    Push(u16),
    PopRegister(Register),
    AddStack,
    AddRegister(Register, Register),
}

impl Op {
    fn parse_instruction(value: u16) -> Result<Self, String> {
        let op = (value & 0xff) as u8;
        match op {
            0 => Ok(Op::Nop),
            1 => {
                let arg = (value & 0xff00) >> 8;
                Ok(Op::Push(arg))
            }
            2 => {
                let arg = ((value & 0xff00) >> 8) as u8;
                Ok(Op::PopRegister(Register::from(arg)))
            }
            3 => Ok(Op::AddStack),
            _ => Err(format!("invalid instruction {:X}", value)),
        }
    }
}

pub struct Machine {
    registers: [u16; 8],
    pub memory: Box<dyn Addressable>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: Box::new(LinearMemory::new(8 * 1024)),
        }
    }
    pub fn get_register(&self, register: Register) -> u16 {
        self.registers[register as usize]
    }

    pub fn push(&mut self, value: u16) -> Result<(), String> {
        let sp = self.registers[Register::SP as usize];
        self.memory.write_word(sp, value);
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }
    pub fn pop(&mut self) -> Result<u16, String> {
        let sp = self.registers[Register::SP as usize] - 2;
        if let Some(value) = self.memory.read_word(sp) {
            self.registers[Register::SP as usize] -= 2;
            Ok(value)
        } else {
            Err(format!("memory fault @ 0x{:X}", sp))
        }
    }
    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read_word(pc).unwrap();
        self.registers[Register::PC as usize] = pc + 2;
        // instruction format = [0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0]
        //                          operator | arg(s)
        //                                   | 8 bit literal
        //                                   | REG1 | REG2
        match Op::parse_instruction(instruction) {
            Ok(Op::Nop) => Ok(()),
            Ok(Op::Push(v)) => self.push(v),
            Ok(Op::PopRegister(r)) => {
                self.registers[r as usize] = self.pop()?;
                Ok(())
            }
            Ok(Op::AddStack) => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a + b)
            }
            Ok(Op::AddRegister(r1, r2)) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }
            Err(err) => Err(format!("{} on {}", err, pc)),
        }
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}
