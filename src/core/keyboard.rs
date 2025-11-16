use crate::Display;
use crate::core::KeyboardError;
use minifb::Key;

pub const NUM_KEYS: usize = 16;

/// CHIP-8 keypad layout:
///
/// 1 2 3 C
/// 4 5 6 D
/// 7 8 9 E
/// A 0 B F
///
/// Mapped to PC keyboard:
///
/// 1 2 3 4
/// Q W E R
/// A S D F
/// Z X C V
///
/// (You can adjust these keys if you prefer)
pub const KEYMAP: [(Key, usize); 16] = [
    (Key::Key1, 0x1),
    (Key::Key2, 0x2),
    (Key::Key3, 0x3),
    (Key::Key4, 0xC),
    (Key::Q, 0x4),
    (Key::W, 0x5),
    (Key::E, 0x6),
    (Key::R, 0xD),
    (Key::A, 0x7),
    (Key::S, 0x8),
    (Key::D, 0x9),
    (Key::F, 0xE),
    (Key::Z, 0xA),
    (Key::X, 0x0),
    (Key::C, 0xB),
    (Key::V, 0xF),
];

/// Converts minifb key → CHIP-8 key (0x0–0xF)
pub fn key_to_chip8(key: Key) -> Result<usize, KeyboardError> {
    for (pc_key, chip8_key) in KEYMAP {
        if pc_key == key {
            return Ok(chip8_key);
        }
    }
    Err(KeyboardError::UnknownKey)
}

#[derive(Debug)]
pub struct Keyboard {
    pub keys: [bool; NUM_KEYS],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; NUM_KEYS],
        }
    }

    pub fn reset(&mut self) {
        self.keys = [false; NUM_KEYS];
    }

    /// Poll all keys from minifb window
    pub fn update(&mut self, display: &Display) {
        self.keys = [false; NUM_KEYS];

        for (pc_key, chip8_key) in KEYMAP {
            if display.window.is_key_down(pc_key) {
                self.keys[chip8_key] = true;
            }
        }
    }

    /// Used for FX0A: wait for a single key press
    pub fn get_key_pressed(&self, display: &Display) -> Option<usize> {
        for (pc_key, chip8_key) in KEYMAP {
            if display.window.is_key_down(pc_key) {
                return Some(chip8_key);
            }
        }
        None
    }

    /// Returns true if CHIP-8 key is pressed
    pub fn is_key_down(&self, key: usize) -> bool {
        if key >= NUM_KEYS {
            return false;
        }
        self.keys[key]
    }
}
