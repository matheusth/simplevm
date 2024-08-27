use crate::register::Register;
use crate::op::Op;
use std::collections::HashMap;

use crate::memory::{Addressable, LinearMemory};

pub type SignalFunction = fn(&mut Machine) -> Result<(), String>;


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
