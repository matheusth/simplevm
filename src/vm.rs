use std::collections::HashMap;

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
pub type SignalFunction = fn(&mut Machine) -> Result<(), String>;
impl TryFrom<u8> for Register {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::M),
            4 => Ok(Self::SP),
            5 => Ok(Self::PC),
            6 => Ok(Self::BP),
            7 => Ok(Self::Flags),
            _ => Err(format!("invalid register {:X}", value)),
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
    Signal(u8),
}

fn parse_instruction_arg(ins: u16) -> u8 {
    (ins >> 8) as u8
}
impl TryFrom<u16> for Op {
    type Error = String;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let op = (value & 0xff) as u8;
        match op {
            0 => Ok(Op::Nop),
            1 => {
                let arg = parse_instruction_arg(value);
                Ok(Op::Push(arg as u16))
            }
            2 => {
                let arg = parse_instruction_arg(value);
                Ok(Op::PopRegister(Register::try_from(arg)?))
            }
            3 => Ok(Op::AddStack),
            4 => Ok(Op::Signal(parse_instruction_arg(value))),
            _ => Err(format!("invalid instruction {:X}", value)),
        }
    }
}

impl TryFrom<String> for Op {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let splited_value: Vec<&str> = value.split(' ').filter(|v| !v.is_empty()).collect();
        match splited_value[0] {
            "Push" => {
                let arg: u16 = splited_value[1].parse().expect("Invalid argument");
                Ok(Op::Push(arg))
            }
            "PopRegister" => {
                if let Ok(arg) = splited_value[1].parse::<u8>() {
                    return Ok(Op::PopRegister(arg.try_into().unwrap()));
                }
                Err(format!(
                    "Invalid argument {} for PopRegister.",
                    splited_value[1]
                ))
            }
            "AddStack" => Ok(Op::AddStack),
            "Signal" => {
                let arg: u8 = splited_value[1].parse().expect("Invalid argument");
                Ok(Op::Signal(arg))
            }
            _ => Err(format!("Invalid instruction {}", splited_value[0])),
        }
    }
}
impl TryFrom<Op> for u16 {
    type Error = String;

    fn try_from(value: Op) -> Result<Self, Self::Error> {
        match value {
            Op::Push(x) => Ok(1 | x << 8),
            Op::PopRegister(x) => Ok(2 & 0xff | (x as u16) << 8),
            Op::AddStack => Ok(3),
            Op::Signal(x) => Ok(4 & 0xff | (x as u16) << 8),
            _ => Err("Unimplemented op!".to_string()),
        }
    }
}

pub struct Machine {
    registers: [u16; 8],
    signal_handlers: HashMap<u8, SignalFunction>,
    pub halt: bool,
    pub memory: Box<dyn Addressable>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: Box::new(LinearMemory::new(8 * 1024)),
            signal_handlers: HashMap::new(),
            halt: false,
        }
    }
    pub fn define_handler(&mut self, index: u8, f: SignalFunction) {
        self.signal_handlers.insert(index, f);
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
        let instruction = self
            .memory
            .read_word(pc)
            .ok_or(format!("failed to read memory on address {:X}.", pc))?;
        self.registers[Register::PC as usize] = pc + 2;
        // instruction format = [0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0]
        //                          operator | arg(s)
        //                                   | 8 bit literal
        //                                   | REG1 | REG2
        match instruction.try_into() {
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
            Ok(Op::Signal(signal)) => self.signal_handlers.get(&signal).unwrap()(self),
            Err(err) => Err(format!("{} on {}", err, pc)),
        }
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}
