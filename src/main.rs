
mod components;
mod instruction;
mod execution;
use instruction::{Instruction,Value};
use components::{Register,Resolution};

pub struct Emulator {
    registers: Vec<Register>,
    resolution_mode: Resolution,
    display: [[bool; 64]; 128],
    ram: [u8; 0x1000],
    rpl: [u8; 8],
    keys: [bool; 0x10],
    key_block: u8, // the register to put the next keypress into
    call_stack: Vec<u16>,
    pro_counter: u16,
    mem_pointer: u16,
    delay_timer: u8,
    sound_timer: u8,
}

fn main() {
    let mut emulator = Emulator::new();

}

impl Emulator {
    pub fn new () -> Emulator {
        let mut registers = Vec::new();
        for i in 0..0x10 as u8 {
            registers.push(components::Register::new(i));
        }

        Self {
            registers,
            resolution_mode: Resolution::Low,
            display: [[false; 64]; 128],
            ram: [0u8; 0x1000],
            rpl: [0u8; 8],
            keys: [false; 0x10],
            key_block: 0x10,
            call_stack: Vec::new(),
            pro_counter: 0x200,
            mem_pointer: 0x000,
            delay_timer: 0x00,
            sound_timer: 0x00,
        }
    }
}