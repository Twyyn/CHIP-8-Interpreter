use chip8::{Display, Keyboard};
fn main() {
    let buffer = vec![0xFF000000u32; 64 * 32];
    let mut win = Display::default();
    let mut k = Keyboard::default();
    while win.window.is_open() {
        k.is_key_down(&win, minifb::Key::A);
        win.window
            .update_with_buffer(&buffer, 64, 32)
            .expect("Failed to update buffer");
        println!("{:?}", k);
    }
}
