
use olc_pge::Key;

const KEYS: [Key;0x10] = [
    Key::X,  // 0
    Key::K1, // 1
    Key::K2, // 2
    Key::K3, // 3
    Key::Q,  // 4
    Key::W,  // 5
    Key::E,  // 6
    Key::A,  // 7
    Key::S,  // 8
    Key::D,  // 9
    Key::Z,  // A
    Key::C,  // B
    Key::K4, // C
    Key::R,  // D
    Key::F,  // E
    Key::V,  // F
];

pub struct KeyHandler {
    pub keys: [bool;0x10],
    pub key_hold: u8,
}
impl KeyHandler {
    pub fn new () -> Self {
        Self {
            keys: [false;0x10],
            key_hold: 0x10,
        }
    }
    pub fn update_keys (&mut self, pge: &olc_pge::PixelGameEngine) {
        for (i, key) in KEYS.iter().enumerate() {
            self.keys[i] = pge.get_key(*key).held;
        }
    }
    pub fn key_block_pressed (&mut self, pge: &olc_pge::PixelGameEngine) -> Option<u8> {
        for (i, key) in KEYS.iter().enumerate() {
            if pge.get_key(*key).pressed {
                return Some(i as u8)
            }
        }
        return None
    }
}