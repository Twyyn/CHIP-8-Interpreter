use std::usize;

use crate::{Display, core::KeyboardError};
use minifb::Key;

pub const NUM_KEYS: usize = 16;

pub const KEYS: [Key; NUM_KEYS] = [
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

#[rustfmt::skip]
#[non_exhaustive]
pub enum Keypad {
    _1, _2, _3, _C,
    _4, _5, _6, _D,
    _7, _8, _9, _E,
    _A, _0, _B, _F,
}

impl Keypad {
    fn from_key(key: Key) -> Result<usize, KeyboardError> {
        let idx = match key {
            Key::Key0 => Keypad::_0 as usize,
            Key::Key1 => Keypad::_1 as usize,
            Key::Key2 => Keypad::_2 as usize,
            Key::Key3 => Keypad::_3 as usize,
            Key::Key4 => Keypad::_4 as usize,
            Key::Key5 => Keypad::_5 as usize,
            Key::Key6 => Keypad::_6 as usize,
            Key::Key7 => Keypad::_7 as usize,
            Key::Key8 => Keypad::_8 as usize,
            Key::Key9 => Keypad::_9 as usize,
            Key::A => Keypad::_A as usize,
            Key::B => Keypad::_B as usize,
            Key::C => Keypad::_C as usize,
            Key::D => Keypad::_D as usize,
            Key::E => Keypad::_E as usize,
            Key::F => Keypad::_F as usize,
            _ => return Err(KeyboardError::UnknownKey),
        };
        Ok(idx)
    }

    pub fn from_keypad(key: usize) -> Result<Key, KeyboardError> {
        if key >= NUM_KEYS {
            return Err(KeyboardError::UnknownKey);
        }
        Ok(KEYS[key])
    }
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
    pub fn new() -> Keyboard {
        Keyboard {
            ..Default::default()
        }
    }
    pub fn is_key_down(&mut self, display: &Display, key: usize) -> bool {
        let key_ = Keypad::from_keypad(key).unwrap();
        let keyboard_key = Keypad::from_key(key_).unwrap();
        self.keyboard[keyboard_key] = display.window.is_key_down(key_);
        self.keyboard[keyboard_key]
    }

    pub fn get_key_pressed(&mut self, display: &Display) -> Option<usize> {
        let key_pressed = display.window.get_keys_pressed(minifb::KeyRepeat::No);
        if let Some(key) = key_pressed.into_iter().next() {
            if let Ok(keyboard_key) = Keypad::from_key(key) {
                self.keyboard[keyboard_key] = true;
                return Some(keyboard_key);
            }
        }
        None
    }
}
