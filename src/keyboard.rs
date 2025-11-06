use winit::keyboard;

use crate::errors::KeyboardError;

pub const NUM_KEYS: usize = 16;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Keyboard {
    pub KEYS: [bool; NUM_KEYS],
}
impl Keyboard {
    pub fn new() -> Self {
        Self {
            KEYS: [false; NUM_KEYS],
        }
    }
    pub fn get_key(&self, key: usize) -> Result<bool, KeyboardError> {
        if key > NUM_KEYS {
            return Err(KeyboardError::UnknownKEY(key));
        }
        Ok(self.KEYS[key])
    }
}
