use crate::components::Resolution;
use crate::instruction::{Instruction, Value};
use crate::Emulator;
use rand;

impl Emulator {
    pub fn execute(&mut self) -> bool {
        // returns a bool for redrawing
        let mut redraw = false;

        let msb = self.get_ram(self.pro_counter);
        let lsb = self.get_ram(self.pro_counter + 1);
        self.pro_counter += 2;

        let inst = Instruction::from(&self, msb, lsb);
        {
            use Instruction::*;
            match inst {
                ClearScreen => {
                    self.clear_screen();
                    redraw = true;
                }

                Jump(addr) => {
                    self.pro_counter = addr;
                }
                JumpPlus { addr, x } => {
                    let result = addr + (x.value as u16);
                    self.pro_counter = result;
                }

                Call(addr) => {
                    self.push_callstack(self.pro_counter);
                    self.pro_counter = addr;
                }
                Return => {
                    self.pro_counter = self.pop_callstack();
                }

                SkipIfEqual { reg, comp } => {
                    let x = reg.value;
                    let y = comp.unwrap();
                    if x == y {
                        self.pro_counter += 2;
                    }
                }
                SkipIfUnequal { reg, comp } => {
                    let x = reg.value;
                    let y = comp.unwrap();
                    if x != y {
                        self.pro_counter += 2;
                    }
                }

                SkipIfKey(reg) => {
                    let x = reg.value as usize;
                    if self.keys[x] {
                        self.pro_counter += 2;
                    }
                }
                SkipIfNotKey(reg) => {
                    let x = reg.value as usize;
                    if !self.keys[x] {
                        self.pro_counter += 2;
                    }
                }
                KeyBlock(reg) => {
                    let x = reg.loc;
                    self.key_block = x;
                }

                Load { reg, value } => {
                    let x = reg.loc;
                    let y = value.unwrap();
                    self.set_register(x, y);
                }

                AddInPlace { reg, byte } => {
                    let x = reg.loc;
                    self.set_register(x, byte);
                }

                Or { x, y } => {
                    let (x_loc, x_val, y_val) = (x.loc, x.value, y.value);
                    self.set_register(x_loc, x_val | y_val);
                }
                And { x, y } => {
                    let (x_loc, x_val, y_val) = (x.loc, x.value, y.value);
                    self.set_register(x_loc, x_val & y_val);
                }
                Xor { x, y } => {
                    let (x_loc, x_val, y_val) = (x.loc, x.value, y.value);
                    self.set_register(x_loc, x_val ^ y_val);
                }

                Add { x, y } => {
                    let (x_loc, x_val, y_val) = (x.loc, x.value, y.value);
                    let (result, overflow) = x_val.overflowing_add(y_val);
                    self.set_register(0xF, u8::from(overflow));
                    self.set_register(x_loc, result);
                }
                Sub { x, y } => {
                    let (x_loc, x_val, y_val) = (x.loc, x.value, y.value);
                    let (result, overflow) = x_val.overflowing_sub(y_val);
                    self.set_register(0xF, u8::from(!overflow));
                    self.set_register(x_loc, result);
                }
                ShiftRight(x) => {
                    let (x_loc, x_val) = (x.loc, x.value);
                    let (result, shift_bit) = (x_val >> 1, x_val & 1);
                    self.set_register(0xF, shift_bit);
                    self.set_register(x_loc, result);
                }
                ShiftLeft(x) => {
                    let (x_loc, x_val) = (x.loc, x.value);
                    let (result, shift_bit) = (x_val << 1, (x_val & 0x80) >> 7);
                    self.set_register(0xF, shift_bit);
                    self.set_register(x_loc, result);
                }

                SetPointer(addr) => {
                    self.mem_pointer = addr;
                }
                AddPointer(x) => {
                    self.mem_pointer += x.value as u16;
                }

                Random { x, byte } => {
                    let random: u8 = rand::random();
                    self.set_register(x.loc, random & byte);
                }

                HighResolution(case) => {
                    use crate::components::Resolution;
                    self.resolution_mode = Resolution::from(case);
                }
                Draw { x, y, byte_count } => {
                    use helpers;
                    redraw = true;
                    let mut collision = false;
                    let bytes =
                        self.get_ram_slice(self.mem_pointer, self.mem_pointer + byte_count as u16);
                    let sprite = helpers::load_sprite(bytes);
                    match self.resolution_mode {
                        Resolution::Low => {
                            // low resolution draw
                            let (x, y) = (x.value as usize % 64, y.value as usize % 32);
                            for x_o in 0..8 as usize {
                                for y_o in 0..byte_count as usize {
                                    let x_pos = x_o + x;
                                    let y_pos = y_o + y;
                                    if sprite[y_o][x_o] {
                                        let collide_once = self.draw_lo(x_pos, y_pos);
                                        collision =
                                            if collision { collision } else { collide_once };
                                    }
                                }
                            }
                        }
                        Resolution::High => {
                            // high resolution draw
                            let (x, y) = (x.value as usize % 128, y.value as usize % 64);
                            for x_o in 0..8 as usize {
                                for y_o in 0..byte_count as usize {
                                    let x_pos = x_o + x;
                                    let y_pos = y_o + y;
                                    if sprite[y_o][x_o] {
                                        let collide_once = self.draw_hi(x_pos, y_pos);
                                        collision =
                                            if collision { collision } else { collide_once };
                                    }
                                }
                            }
                        }
                    }
                    self.set_register(0xF, u8::from(collision));
                }
                DrawLarge { x, y } => {
                    use helpers;
                    let (x, y) = (x.value as usize % 128, y.value as usize % 64);
                    redraw = true;
                    let mut collision = false;
                    let bytes = self.get_ram_slice(self.mem_pointer, self.mem_pointer + 32);
                    let sprite = helpers::load_large_sprite(bytes);
                    for x_o in 0..16 as usize {
                        for y_o in 0..16 as usize {
                            let x_pos = x_o + x;
                            let y_pos = y_o + y;
                            if sprite[y_o][x_o] {
                                let collide_once = self.draw_hi(x_pos, y_pos);
                                collision = if collision { collision } else { collide_once };
                            }
                        }
                    }
                    self.set_register(0xF, u8::from(collision));
                }
                ScrollRight => {
                    for x in (0..124).rev() {
                        let temp = self.display[x];
                        self.display[x + 4] = temp;
                        self.display[x] = [false; 64];
                    }
                }
                ScrollLeft => {
                    for x in 4..128 {
                        let temp = self.display[x];
                        self.display[x - 4] = temp;
                        self.display[x] = [false; 64];
                    }
                }
                ScrollDown(pixels) => {
                    let pixels = pixels as usize;
                    for y in (0..64 - pixels).rev() {
                        for x in 0..128 as usize {
                            let temp = self.display[x][y];
                            self.display[x][y + pixels] = temp;
                            self.display[x][y] = false;
                        }
                    }
                }

                GetTimer(x) => {
                    self.set_register(x.loc, self.delay_timer);
                }
                SetTimer(x) => {
                    self.delay_timer = x.value;
                }
                SetSound(x) => {
                    self.sound_timer = x.value;
                }

                GetDigit(x) => {
                    self.mem_pointer = (x.value as u16) * 5;
                }
                StoreDecimal(x) => {
                    let x = x.value;
                    self.set_ram(self.mem_pointer, x / 100);
                    self.set_ram(self.mem_pointer + 1, (x / 10) % 10);
                    self.set_ram(self.mem_pointer + 2, x % 10);
                }

                StoreRegisters(x) => {
                    let x = x.loc;
                    for i in 0..=x as u16 {
                        let reg = self.get_register(i as u8);
                        self.set_ram(self.mem_pointer + i, reg);
                    }
                }
                LoadRegisters(x) => {
                    let x = x.loc;
                    for i in 0..=x as u16 {
                        let val = self.get_ram(self.mem_pointer + i);
                        self.set_register(i as u8, val);
                    }
                }
                StoreRegistersRPL(x) => {
                    let x = x.loc;
                    for i in 0..=x as u8 {
                        let reg = self.get_register(i as u8);
                        self.set_rpl(i, reg);
                    }
                }
                LoadRegistersRPL(x) => {
                    let x = x.loc;
                    for i in 0..=x as u8 {
                        let val = self.get_rpl(i);
                        self.set_register(i, val);
                    }
                }
                Invalid => {}
            }
        }

        redraw
    }
}

mod helpers {
    pub fn byte_to_bools(byte: u8) -> [bool; 8] {
        let mut ret = [false; 8];
        for j in 0..8 {
            let i = 7 - j;
            let mask = 1 << i;
            ret[j] = (byte & mask) >> i == 1;
        }
        return ret;
    }
    pub fn twobyte_twobools(bytea: u8, byteb: u8) -> [bool; 16] {
        // tried to call it 2byte2bools, but it wouldn't let me XD
        let mut ret = [false; 16];
        let num = ((bytea as u16) << 8) | (byteb as u16);
        for j in 0..16 {
            let i = 15 - j;
            let mask = 1 << i;
            ret[j] = (num & mask) >> i == 1;
        }
        return ret;
    }
    pub fn load_sprite(bytes: &[u8]) -> Vec<[bool; 8]> {
        let mut ret = Vec::new();
        for byte in bytes {
            ret.push(byte_to_bools(*byte));
        }
        return ret;
    }
    pub fn load_large_sprite(bytes: &[u8]) -> Vec<[bool; 16]> {
        let mut ret = Vec::new();
        for j in 0..8 {
            let i = j * 2;
            ret.push(twobyte_twobools(bytes[i], bytes[i + 1]));
        }
        return ret;
    }
}
