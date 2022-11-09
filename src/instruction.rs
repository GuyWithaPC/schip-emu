
pub struct Register {
    value: u8,
    loc: u8,
}

pub enum Value { // to encompass values that are either register or byte
    Register(Register),
    Byte(u8),
}

pub enum Instruction {
    ClearScreen, // clear the screen

    Jump { addr: u16 }, // jump to addr
    JumpPlus { addr: u16, x: Register }, // jump to addr + R{X}

    Call { addr: u16 }, // call a procedure at addr
    Return, // return from a procedure

    SkipIfEqual { reg: Register, comp: Value }, // skip if a == b
    SkipIfUnequal { reg: Register, comp: Value }, // skip if a != b

    SkipIfKey { reg: Register }, // skip if the key with the value of reg is pressed
    SkipIfNotKey { reg: Register }, // skip if the key with the value of reg is not pressed
    KeyBlock { reg: Register }, // block for keypress, store value in register

    Load { reg: Register, value: Value }, // load value into register

    AddInPlace { reg: Register, byte: u8 }, // add byte to register, store result in register

    Or { x: Register, y: Register }, // OR x and y, store in x
    And { x: Register, y: Register }, // AND x and y, store in x
    Xor { x: Register, y: Register }, // XOR x and y, store in x
    Add { x: Register, y: Register }, // ADD x and y, store in x (sets overflow flag)
    Sub { x: Register, y: Register }, // SUB x and y, store in x (sets !overflow flag)
    ShiftRight { x: Register }, // SHR x in-place (sets overflow flag)
    ShiftLeft { x: Register }, // SHL x in-place (sets overflow flag)

    SetPointer { addr: u16 }, // set the memory pointer to addr
    AddPointer { reg: Register }, // adds register to memory pointer

    Random { x: Register, byte: u8 }, // random value & byte, goes in x

    HighResolution { on_off: bool }, // changes the resolution mode
    Draw { x: Register, y: Register, byte_count: u8 }, // draw byte_count bytes at (x,y)
    DrawLarge { x: Register, y: Register }, // draws a 16x16 sprite at (x,y) (only in high-res mode)

    GetTimer { reg: Register }, // set reg to delay timer
    SetTimer { reg: Register }, // set delay timer to reg
    SetSound { reg: Register }, // set sound timer to reg

    GetDigit { reg: Register }, // sets I to the location of the character representing reg
    StoreDecimal { reg: Register }, // stores the decimal representation of reg in RAM

    StoreRegisters { reg: Register }, // stores registers 0..reg in RAM
    LoadRegisters { reg: Register }, // loads registers 0..reg from RAM
    StoreRegistersRPL { reg: Register}, // stores registers 0..reg in RPL memory
    LoadRegistersRPL { reg: Register }, // loads registers 0..reg from RPL memory
}
impl Instruction {
    pub fn from (msb: u8, lsb: u8) -> Instruction {
        let opcode = (msb & 0xF0) >> 4;
        let x_reg = (msb & 0x0F);
        let y_reg = (lsb & 0xF0) >> 4;
        let byte = lsb;
        let addr = (((msb as u16) << 8) | (lsb as u16)) & 0x0FFF;
        match opcode {

        }
    }
}