pub mod core;
use crate::core::CPU;
pub use core::display::Display;
pub use core::keyboard::Keyboard;
pub use minifb::{Scale, Window, WindowOptions};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct CHIP8 {
    cpu: CPU,
}
impl CHIP8 {}
