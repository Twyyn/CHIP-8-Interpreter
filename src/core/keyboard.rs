use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
pub const NUM_KEYS: usize = 16;

pub enum KeyState {
    PRESSED,
    NOTPRESSED,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Keyboard {
    pub KEYS: [bool; NUM_KEYS],
}
impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            KEYS: [false; NUM_KEYS],
        }
    }
}
impl Keyboard {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn press(&mut self, key: u8) {
        if let Some(b) = self.KEYS.get_mut((key & 0x0F) as usize) {
            *b = true;
        }
    }
    pub fn release(&mut self, key: u8) {
        if let Some(slot) = self.KEYS.get_mut((key & 0x0F) as usize) {
            *slot = false;
        }
    }
    pub fn is_pressed(&self, key: u8) -> bool {
        self.KEYS
            .get((key & 0x0F) as usize)
            .copied()
            .unwrap_or(false)
    }
}
