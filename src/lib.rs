use crate::core::CPU;

pub mod core;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct CHIP8 {
    CPU: core::CPU,
}
