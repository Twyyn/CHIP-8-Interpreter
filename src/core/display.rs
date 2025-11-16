use minifb::{Scale, Window, WindowOptions};

pub const WINDOW_WIDTH: usize = 64;
pub const WINDOW_HEIGHT: usize = 32;
/* Black */
const COLOR_OFF: u32 = 0x000000;
/* White */
const COLOR_ON: u32 = 0xFFFFFF; 

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
    pub fn set_target_fps(&mut self) {
        self.window.set_target_fps(60);
    }
    pub fn window_is_open(&self) -> bool {
        self.window.is_open()
    }
    pub fn get_x_postion(&self, x: usize) -> usize {
        x % WINDOW_WIDTH
    }
    pub fn get_y_postion(&self, y: usize) -> usize {
        y % WINDOW_HEIGHT
    }
    pub fn set_pixels(&mut self, index: usize) {
        self.pixels[index] = if self.pixels[index] == COLOR_OFF {
            COLOR_ON
        } else {
            COLOR_OFF
        }
    }
    pub fn clear(&mut self) {
        self.pixels = [COLOR_OFF; WINDOW_WIDTH * WINDOW_HEIGHT];
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
