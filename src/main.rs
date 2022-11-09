
mod components;
mod instruction;
use instruction::{Instruction,Value};
use components::{Register};

pub struct Emulator {
    registers: Vec<Register>
}

fn main() {
    let mut emulator = Emulator::new();
    let inst = Instruction::from(&emulator,0x63,0x25);
    println!("{:?}",inst);
    emulator.set_register(3,0xF0);
    let inst = Instruction::from(&emulator,0x81,0x30);
    match inst {
        Instruction::Load { ref reg, ref value } => {
            emulator.set_register(reg.loc,match value {
                Value::Byte(byte) => *byte,
                Value::Register(reg) => reg.value,
            });
        },
        _ => {},
    }
    println!("{:?}",inst);
    println!("{:?}",emulator.get_register(1));
}

impl Emulator {
    pub fn new () -> Emulator {
        let mut registers = Vec::new();
        for i in 0..0x10 as u8 {
            registers.push(components::Register::new(i));
        }

        Self {
            registers
        }
    }
    pub fn get_register_data (&self, x: u8) -> Register {
        Register {
            ..self.registers[x as usize]
        }
    }
    pub fn get_register (&self, x: u8) -> u8 {
        self.registers[x as usize].value
    }
    pub fn set_register (&mut self, x: u8, val: u8) {
        self.registers[x as usize].value = val;
    }
}