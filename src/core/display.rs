use crate::{Scale, Window, WindowOptions};

pub const WINDOW_WIDTH: usize = 64;
pub const WINDOW_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    pub window: Window,
}
impl Default for Display {
    fn default() -> Self {
        Self {
            window: Window::new(
                "CHIP-8",
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                WindowOptions {
                    // optional: scale it up so you can see the tiny CHIP-8 display
                    scale: Scale::X16,
                    ..WindowOptions::default()
                },
            )
            .expect("Failed"),
        }
    }
}
