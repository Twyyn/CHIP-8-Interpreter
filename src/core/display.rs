use crate::{Scale, Window, WindowOptions};

pub const WINDOW_WIDTH: usize = 64;
pub const WINDOW_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    pub window: Window,
    pub pixels: [u8; WINDOW_WIDTH * WINDOW_HEIGHT],
}
impl Display {
    pub fn new() -> Self {
        Self {
            window: Window::new(
                "CHIP-8",
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                WindowOptions {
                    scale: Scale::X16,
                    ..WindowOptions::default()
                },
            )
            .expect("Failed to create Window"),
            pixels: [0; WINDOW_WIDTH * WINDOW_HEIGHT],
        }
    }
}
