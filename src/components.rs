use crate::Emulator;
use std::fs::File;
use std::io::{Read};

#[derive(Debug)]
pub struct Register {
    pub value: u8,
    pub loc: u8,
}
impl Register {
    pub fn new(loc: u8) -> Register {
        Self { value: 0x00, loc }
    }
}

pub enum Resolution {
    High,
    Low,
}
impl Resolution {
    pub fn from(b: bool) -> Resolution {
        if b {
            Self::High
        } else {
            Self::Low
        }
    }
}
impl From<Resolution> for bool {
    fn from(r: Resolution) -> bool {
        match r {
            Resolution::High => true,
            Resolution::Low => false,
        }
    }
}

impl Emulator {
    // memory accessor functions
    pub fn get_ram(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }
    pub fn get_ram_slice(&self, addr_a: u16, addr_b: u16) -> &[u8] {
        let a = addr_a as usize;
        let b = addr_b as usize;
        &self.ram[a..b]
    }
    pub fn set_ram(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }
    pub fn get_rpl(&self, addr: u8) -> u8 {
        // addr HAS to be between 0 and 7, nothing more.
        self.rpl[addr as usize]
    }
    pub fn set_rpl(&mut self, addr: u8, val: u8) {
        // addr is same as get_rpl
        self.rpl[addr as usize] = val;
    }
    pub fn push_callstack(&mut self, val: u16) {
        self.call_stack.push(val);
    }
    pub fn pop_callstack(&mut self) -> u16 {
        self.call_stack
            .pop()
            .expect("there was nothing on the callstack!")
    }

    // ROM loading function
    pub fn load_rom(&mut self, loc: u16, filename: &str) {
        print!("{:?} => ",filename);
        let mut load_file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => panic!("could not open file: {:?}",e)
        };
        let mut buf: Vec<u8> = Vec::new();
        match load_file.read_to_end(&mut buf) {
            Ok(_) => (),
            Err(e) => panic!("could not open file: {:?}",e)
        };
        println!("{} bytes.", buf.len());
        for i in 0..buf.len() {
            self.set_ram(loc + i as u16, buf[i]);
        }
    }

    // register accessor functions
    pub fn get_register_data(&self, x: u8) -> Register {
        Register {
            ..self.registers[x as usize]
        }
    }
    pub fn get_register(&self, x: u8) -> u8 {
        self.registers[x as usize].value
    }
    pub fn set_register(&mut self, x: u8, val: u8) {
        self.registers[x as usize].value = val;
    }

    // graphics accessor functions
    pub fn clear_screen(&mut self) {
        self.display = [[false; 64]; 128];
    }
    pub fn draw_hi(&mut self, x: usize, y: usize) -> bool {
        // returns whether this draw call intersected
        if x >= 128 || y >= 64 {
            return false;
        }
        if self.display[x][y] {
            self.display[x][y] = false;
            true
        } else {
            self.display[x][y] = true;
            false
        }
    }
    pub fn draw_lo(&mut self, x: usize, y: usize) -> bool {
        // returns same as draw_hi, just doubles coordinates
        if x >= 64 || y >= 32 {
            return false;
        }
        let x = x * 2;
        let y = y * 2;
        let mut drew_over = false;

        for xo in 0..2 as usize {
            for yo in 0..2 as usize {
                if self.display[x + xo][y + yo] {
                    self.display[x + xo][y + yo] = false;
                    drew_over = true;
                } else {
                    self.display[x + xo][y + yo] = true;
                }
            }
        }

        drew_over
    }
}
