
use crate::components::Register;
use crate::Emulator;

#[derive(Debug)]
pub enum Value<'a> { // to encompass values that are either register or byte
    Register(&'a Register),
    Byte(u8),
}
impl <'a> Value <'a> {
    pub fn from_reg (val: &'a Register) -> Value <'a> {
        Value::Register(val)
    }
    pub fn from_byte (val: u8) -> Value <'a> {
        Value::Byte(val)
    }
}

#[derive(Debug)]
pub enum Instruction <'a> {
    ClearScreen, // clear the screen

    Jump(u16), // jump to addr
    JumpPlus { addr: u16, x: &'a Register }, // jump to addr + R{X}

    Call(u16), // call a procedure at addr
    Return, // return from a procedure

    SkipIfEqual { reg: &'a Register, comp: Value <'a> }, // skip if a == b
    SkipIfUnequal { reg: &'a Register, comp: Value <'a> }, // skip if a != b

    SkipIfKey(&'a Register), // skip if the key with the value of reg is pressed
    SkipIfNotKey(&'a Register), // skip if the key with the value of reg is not pressed
    KeyBlock(&'a Register), // block for keypress, store value in register

    Load { reg: &'a Register, value: Value<'a> }, // load value into register

    AddInPlace { reg: &'a Register, byte: u8 }, // add byte to register, store result in register

    Or { x: &'a Register, y: &'a Register }, // OR x and y, store in x
    And { x: &'a Register, y: &'a Register }, // AND x and y, store in x
    Xor { x: &'a Register, y: &'a Register }, // XOR x and y, store in x
    Add { x: &'a Register, y: &'a Register }, // ADD x and y, store in x (sets overflow flag)
    Sub { x: &'a Register, y: &'a Register }, // SUB x and y, store in x (sets !overflow flag)
    ShiftRight(&'a Register), // SHR x in-place (sets overflow flag)
    ShiftLeft(&'a Register), // SHL x in-place (sets overflow flag)

    SetPointer(u16), // set the memory pointer to addr
    AddPointer(&'a Register), // adds register to memory pointer

    Random { x: &'a Register, byte: u8 }, // random value & byte, goes in x

    HighResolution(bool), // changes the resolution mode
    Draw { x: &'a Register, y: &'a Register, byte_count: u8 }, // draw byte_count bytes at (x,y)
    DrawLarge { x: &'a Register, y: &'a Register }, // draws a 16x16 sprite at (x,y) (only in high-res mode)

    GetTimer(&'a Register), // set reg to delay timer
    SetTimer(&'a Register), // set delay timer to reg
    SetSound(&'a Register), // set sound timer to reg

    GetDigit(&'a Register), // sets I to the location of the character representing reg
    StoreDecimal(&'a Register), // stores the decimal representation of reg in RAM

    StoreRegisters(&'a Register), // stores registers 0..reg in RAM
    LoadRegisters(&'a Register), // loads registers 0..reg from RAM
    StoreRegistersRPL(&'a Register), // stores registers 0..reg in RPL memory
    LoadRegistersRPL(&'a Register), // loads registers 0..reg from RPL memory

    Invalid, // this is passed if the instruction didn't exist
}
impl <'a> Instruction <'a> {
    pub fn from (emulator: &Emulator, msb: u8, lsb: u8) -> Instruction {
        let opcode = (msb & 0xF0) >> 4;
        let x = msb & 0x0F;
        let y = (lsb & 0xF0) >> 4;
        let x_reg = emulator.get_register(x);
        let y_reg = emulator.get_register(y);
        let byte = lsb;
        let nibble = lsb & 0x0F;
        let addr = (((msb as u16) << 8) | (lsb as u16)) & 0x0FFF;
        use Instruction::*;
        match opcode {
            0x0 => {
                match byte {
                    0xE0 => ClearScreen,
                    0xEE => Return,
                    0xFE => HighResolution(false),
                    0xFF => HighResolution(true),
                    _ => Invalid,
                }
            },
            0x1 => Jump(addr),
            0x2 => Call(addr),
            0x3 => SkipIfEqual{ reg: x_reg, comp: Value::from_byte(byte) },
            0x4 => SkipIfUnequal { reg: x_reg, comp: Value::from_byte(byte) },
            0x5 => SkipIfEqual { reg: x_reg, comp: Value::from_reg(y_reg) },
            0x6 => Load { reg: x_reg, value: Value::from_byte(byte) },
            0x7 => AddInPlace { reg: x_reg, byte },
            0x8 => match nibble {
                0x0 => Load { reg: x_reg, value: Value::from_reg(y_reg) },
                0x1 => Or { x: x_reg, y: y_reg},
                0x2 => And { x: x_reg, y: y_reg },
                0x3 => Xor { x: x_reg, y: y_reg },
                0x4 => Add { x: x_reg, y: y_reg },
                0x5 => Sub { x: x_reg, y: y_reg },
                0x6 => ShiftRight(x_reg),
                0x7 => Sub { x: y_reg, y: x_reg },
                0xE => ShiftLeft(x_reg),
                _ => Invalid,
            },
            0x9 => SkipIfUnequal { reg: x_reg, comp: Value::from_reg(y_reg) },
            0xA => SetPointer(addr),
            0xB => JumpPlus { addr, x: x_reg },
            0xC => Random { x: x_reg, byte},
            0xD => match nibble {
                0x0 => DrawLarge { x: x_reg, y: y_reg },
                _ => Draw { x: x_reg, y: y_reg, byte_count: nibble},
            },
            0xE => match byte {
                0x9E => SkipIfKey(x_reg),
                0xA1 => SkipIfNotKey(x_reg),
                _ => Invalid,
            },
            0xF => match byte {
                0x07 => GetTimer(x_reg),
                0x0A => KeyBlock(x_reg),
                0x15 => SetTimer(x_reg),
                0x18 => SetSound(x_reg),
                0x1E => AddPointer(x_reg),
                0x29 => GetDigit(x_reg),
                0x33 => StoreDecimal(x_reg),
                0x55 => StoreRegisters(x_reg),
                0x65 => LoadRegisters(x_reg),
                0x75 => StoreRegistersRPL(x_reg),
                0x85 => LoadRegistersRPL(x_reg),
                _ => Invalid,
            },
            _ => Invalid,
        }
    }
}