
#[derive(Debug)]
pub struct Register {
    pub value: u8,
    pub loc: u8,
}
impl Register {
    pub fn new (loc: u8) -> Register {
        Self {
            value: 0x00,
            loc,
        }
    }
}