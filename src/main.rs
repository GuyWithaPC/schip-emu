
mod components;
mod instruction;

pub struct Emulator {
    registers: Vec<components::Register>
}

fn main() {
    let mut emulator = Emulator::new();
    let inst = instruction::Instruction::from(&emulator,0x83,0x25);
    println!("{:?}",inst);
    emulator.set_register(3,0xF0);
    let inst = instruction::Instruction::from(&emulator,0x83,0x24);
    println!("{:?}",inst);
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
    pub fn get_register (&self, x: u8) -> &components::Register {
        &self.registers[x as usize]
    }
    pub fn set_register (&mut self, x: u8, val: u8) {
        self.registers[x as usize].value = val;
    }
}