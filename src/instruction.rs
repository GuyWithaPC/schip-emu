use crate::components::Register;
use crate::Emulator;

#[derive(Debug)]
pub enum Value {
    // to encompass values that are either register or byte
    Register(Register),
    Byte(u8),
}
impl Value {
    pub fn from_reg(val: Register) -> Value {
        Value::Register(val)
    }
    pub fn from_byte(val: u8) -> Value {
        Value::Byte(val)
    }
    pub fn unwrap(&self) -> u8 {
        match self {
            Value::Byte(byte) => *byte,
            Value::Register(reg) => reg.value,
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    ClearScreen, // clear the screen

    Jump(u16), // jump to addr
    JumpPlus {
        addr: u16,
        x: Register,
    }, // jump to addr + R{X}

    Call(u16), // call a procedure at addr
    Return,    // return from a procedure

    SkipIfEqual {
        reg: Register,
        comp: Value,
    }, // skip if a == b
    SkipIfUnequal {
        reg: Register,
        comp: Value,
    }, // skip if a != b

    SkipIfKey(Register),    // skip if the key with the value of reg is pressed
    SkipIfNotKey(Register), // skip if the key with the value of reg is not pressed
    KeyBlock(Register),     // block for keypress, store value in register

    Load {
        reg: Register,
        value: Value,
    }, // load value into register

    AddInPlace {
        reg: Register,
        byte: u8,
    }, // add byte to register, store result in register

    Or {
        x: Register,
        y: Register,
    }, // OR x and y, store in x
    And {
        x: Register,
        y: Register,
    }, // AND x and y, store in x
    Xor {
        x: Register,
        y: Register,
    }, // XOR x and y, store in x
    Add {
        x: Register,
        y: Register,
    }, // ADD x and y, store in x (sets overflow flag)
    Sub {
        x: Register,
        y: Register,
    }, // SUB x and y, store in x (sets !overflow flag)
    ShiftRight(Register), // SHR x in-place (sets overflow flag)
    ShiftLeft(Register),  // SHL x in-place (sets overflow flag)

    SetPointer(u16),      // set the memory pointer to addr
    AddPointer(Register), // adds register to memory pointer

    Random {
        x: Register,
        byte: u8,
    }, // random value & byte, goes in x

    HighResolution(bool), // changes the resolution mode
    Draw {
        x: Register,
        y: Register,
        byte_count: u8,
    }, // draw byte_count bytes at (x,y)
    DrawLarge {
        x: Register,
        y: Register,
    }, // draws a 16x16 sprite at (x,y) (only in high-res mode)
    ScrollRight,
    ScrollLeft,
    ScrollDown(u8),

    GetTimer(Register), // set reg to delay timer
    SetTimer(Register), // set delay timer to reg
    SetSound(Register), // set sound timer to reg

    GetDigit(Register), // sets I to the location of the character representing reg
    StoreDecimal(Register), // stores the decimal representation of reg in RAM

    StoreRegisters(Register),    // stores registers 0..reg in RAM
    LoadRegisters(Register),     // loads registers 0..reg from RAM
    StoreRegistersRPL(Register), // stores registers 0..reg in RPL memory
    LoadRegistersRPL(Register),  // loads registers 0..reg from RPL memory

    Invalid, // this is passed if the instruction didn't exist
}
impl Instruction {
    pub fn from(emulator: &Emulator, msb: u8, lsb: u8) -> Instruction {
        let opcode = (msb & 0xF0) >> 4;
        let x = msb & 0x0F;
        let y = (lsb & 0xF0) >> 4;
        let x_reg = emulator.get_register_data(x);
        let y_reg = emulator.get_register_data(y);
        let byte = lsb;
        let nibble = lsb & 0x0F;
        let addr = (((msb as u16) << 8) | (lsb as u16)) & 0x0FFF;
        use Instruction::*;
        match opcode {
            0x0 => match byte {
                0xE0 => ClearScreen,
                0xEE => Return,
                0xFB => ScrollRight,
                0xFC => ScrollLeft,
                0xFE => HighResolution(false),
                0xFF => HighResolution(true),
                _ => {
                    if y == 0xC {
                        ScrollDown(nibble)
                    } else {
                        Invalid
                    }
                }
            },
            0x1 => Jump(addr),
            0x2 => Call(addr),
            0x3 => SkipIfEqual {
                reg: x_reg,
                comp: Value::from_byte(byte),
            },
            0x4 => SkipIfUnequal {
                reg: x_reg,
                comp: Value::from_byte(byte),
            },
            0x5 => SkipIfEqual {
                reg: x_reg,
                comp: Value::from_reg(y_reg),
            },
            0x6 => Load {
                reg: x_reg,
                value: Value::from_byte(byte),
            },
            0x7 => AddInPlace { reg: x_reg, byte },
            0x8 => match nibble {
                0x0 => Load {
                    reg: x_reg,
                    value: Value::from_reg(y_reg),
                },
                0x1 => Or { x: x_reg, y: y_reg },
                0x2 => And { x: x_reg, y: y_reg },
                0x3 => Xor { x: x_reg, y: y_reg },
                0x4 => Add { x: x_reg, y: y_reg },
                0x5 => Sub { x: x_reg, y: y_reg },
                0x6 => ShiftRight(x_reg),
                0x7 => Sub { x: y_reg, y: x_reg },
                0xE => ShiftLeft(x_reg),
                _ => Invalid,
            },
            0x9 => SkipIfUnequal {
                reg: x_reg,
                comp: Value::from_reg(y_reg),
            },
            0xA => SetPointer(addr),
            0xB => JumpPlus { addr, x: x_reg },
            0xC => Random { x: x_reg, byte },
            0xD => match nibble {
                0x0 => DrawLarge { x: x_reg, y: y_reg },
                _ => Draw {
                    x: x_reg,
                    y: y_reg,
                    byte_count: nibble,
                },
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
