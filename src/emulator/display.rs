use minifb::{Scale, Window, WindowOptions};

/*CHIP-8 Window Height and Width */
pub const WINDOW_WIDTH: usize = 64;
pub const WINDOW_HEIGHT: usize = 32;

/* 0x000000 = Black or 0 */
const PIXEL_OFF: u32 = 0x000000;
/* 0xFFFFFF = White or 1 */
const PIXEL_ON: u32 = 0xFFFFFF;

#[derive(Debug)]
pub struct Display {
    pub pixels_buffer: [u32; WINDOW_WIDTH * WINDOW_HEIGHT],
    pub window: Window,
}
impl Default for Display {
    fn default() -> Display {
        Display {
            pixels_buffer: [0; WINDOW_WIDTH * WINDOW_HEIGHT],
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
    #[inline]
    /* Sets display target FPS to 60 */
    pub fn set_target_fps(&mut self) {
        self.window.set_target_fps(60);
    }
    #[inline]
    /* Returns if window is open */
    pub fn window_is_open(&self) -> bool {
        self.window.is_open()
    }
    #[inline]
    /* Returns the current postion of x on the display  */
    pub fn get_x_postion(&self, x: usize) -> usize {
        x % WINDOW_WIDTH
    }
    #[inline]
    /* Returns the current postion of y on the display  */
    pub fn get_y_postion(&self, y: usize) -> usize {
        y % WINDOW_HEIGHT
    }
    /* Sets pixel in pixel buffer */
    pub fn set_pixels(&mut self, index: usize) {
        self.pixels_buffer[index] = if self.pixels_buffer[index] == PIXEL_OFF {
            PIXEL_ON
        } else {
            PIXEL_OFF
        }
    }
    #[inline]
    /* Returns if pixel in pixel buffer is on */
    pub fn is_pixel_on(&self, index: usize) -> bool {
        self.pixels_buffer[index] == PIXEL_ON
    }
    #[inline]
    /* Clears pixel buffer */
    pub fn clear(&mut self) {
        self.pixels_buffer = [PIXEL_OFF; WINDOW_WIDTH * WINDOW_HEIGHT];
    }
    #[inline]
    /* Returns pixel at some index */
    pub fn get_pixel(&self, index: usize) -> u32 {
        self.pixels_buffer[index]
    }
    /* Updates display with pixel buffer */
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.pixels_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
