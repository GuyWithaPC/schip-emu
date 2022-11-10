mod components;
mod execution;
mod instruction;
mod keyhandler;
use components::{Register, Resolution};
use instruction::{Instruction, Value};
use keyhandler::KeyHandler;
use olc_pge as olc;
use olc_pge::PixelGameEngine;
use clap::Parser;

const OFF_COLOR: olc::Pixel = olc::BLACK;
const ON_COLOR: olc::Pixel = olc::WHITE;
const FRAME_TIME: f32 = 1.0/6000.0;

pub struct Emulator {
    registers: Vec<Register>,

    resolution_mode: Resolution,
    display: [[bool; 64]; 128],

    ram: [u8; 0x1000],
    rpl: [u8; 8],

    key_handler: KeyHandler,

    call_stack: Vec<u16>,

    pro_counter: u16,
    mem_pointer: u16,

    delay_timer: u8,
    sound_timer: u8,

    frame_time: f32,
    timer_time: f32,
}

fn main() {
    let mut emulator = Emulator::new();
    olc::PixelGameEngine::construct(emulator,128,64,5,5).start();
}

impl olc::PGEApplication for Emulator {
    const APP_NAME: &'static str = "SuperChip Emulator";
    fn on_user_create(&mut self, _pge: &mut PixelGameEngine) -> bool {
        for i in 0..4096 {
            if i % 16 == 0 {
                println!();
                print!("{:#05X} => ",i);
            }
            print!("{:#04X} ",self.get_ram(i));
        }
        println!();
        true
    }

    fn on_user_update(&mut self, pge: &mut PixelGameEngine, elapsed_time: f32) -> bool {
        self.key_handler.update_keys(pge);

        self.timer_time += elapsed_time;
        if self.timer_time >= 1.0/60.0 {
            self.delay_timer = if self.delay_timer == 0 {0} else {self.delay_timer-1};
            self.sound_timer = if self.sound_timer == 0 {0} else {self.sound_timer-1};
            self.timer_time = 0.0;
        }

        if self.key_handler.key_hold == 0x10 {
            self.frame_time += elapsed_time;
            if self.frame_time >= FRAME_TIME {
                let redraw = self.execute();
                if redraw { self.draw_to_screen(pge); }
                self.frame_time = 0.0;
            }
        } else { // wait for a key in this case
            match self.key_handler.key_block_pressed(pge) {
                Some(key) => {
                    self.set_register(self.key_handler.key_hold,key);
                    self.key_handler.key_hold = 0x10;
                }
                None => ()
            }
        }
        true
    }
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut registers = Vec::new();
        for i in 0..0x10 as u8 {
            registers.push(components::Register::new(i));
        }

        let mut ret = Self {
            registers,
            resolution_mode: Resolution::Low,
            display: [[false; 64]; 128],
            ram: [0u8; 0x1000],
            rpl: [0u8; 8],
            key_handler: KeyHandler::new(),
            call_stack: Vec::new(),
            pro_counter: 0x200,
            mem_pointer: 0x000,
            delay_timer: 0x00,
            sound_timer: 0x00,
            frame_time: 0.0,
            timer_time: 0.0,
        };
        ret.load_rom(0x000,"system/font.bin");
        ret.load_rom(0x200,"demo/Sierpinsky.ch8");
        return ret
    }

    pub fn draw_to_screen (&self, pge: &mut olc::PixelGameEngine) {
        pge.clear(OFF_COLOR);
        for x in 0..128 {
            for y in 0..64 {
                if self.display[x][y] {
                    pge.draw(x as i32, y as i32, ON_COLOR);
                } else {
                    pge.draw(x as i32, y as i32, OFF_COLOR);
                }
            }
        }
    }
}
