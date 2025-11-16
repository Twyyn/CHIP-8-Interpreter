use std::default;

use minifb::{Scale, Window, WindowOptions};

pub const WINDOW_WIDTH: usize = 64;
pub const WINDOW_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    pub pixels: [u32; WINDOW_WIDTH * WINDOW_HEIGHT],
    pub window: Window,
}
impl Default for Display {
    fn default() -> Display {
        Display {
            pixels: [0; WINDOW_WIDTH * WINDOW_HEIGHT],
            window: Window::new(
                "CHIP-8",
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                WindowOptions {
                    scale: Scale::X16,
                    ..WindowOptions::default()
                },
            )
            .expect("Failed to create window."),
        }
    }
}
impl Display {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn get_x_postion(&self, x: usize) -> usize {
        x % WINDOW_WIDTH
    }
    pub fn get_y_postion(&self, y: usize) -> usize {
        y % WINDOW_HEIGHT
    }
    pub fn set_pixels(&mut self, index: usize) {
        self.pixels[index] ^= 1
    }
    pub fn clear(&mut self) {
        self.pixels = [0; WINDOW_WIDTH * WINDOW_HEIGHT];
    }
    pub fn get_pixel(&self, index: usize) -> u32 {
        self.pixels[index]
    }
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.pixels, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
