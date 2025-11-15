use crate::{Display, core::KeyboardError};
use minifb::Key;
pub const NUM_KEYS: usize = 16;

pub const CODE_TO_KEY: [Key; NUM_KEYS] = [
    Key::Key0,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Key5,
    Key::Key6,
    Key::Key7,
    Key::Key8,
    Key::Key9,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
];

pub fn from_key(code: usize) -> Result<Key, KeyboardError> {
    CODE_TO_KEY
        .get(code)
        .copied()
        .ok_or(KeyboardError::UnknownKey)
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Keyboard {
    pub keyboard: [bool; NUM_KEYS],
}
impl Default for Keyboard {
    fn default() -> Keyboard {
        Keyboard {
            keyboard: [false; NUM_KEYS],
        }
    }
}
impl Keyboard {
    pub fn is_key_down(&mut self, display: &Display, key: usize) -> bool {
        let key_ = from_key(key).unwrap();
        self.keyboard[key] = display.window.is_key_down(key_);
        self.keyboard[key]
    }
}
