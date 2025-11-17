use chip8::CHIP8;
use chip8::core::input::Directory;

fn main() {
    let mut path = Directory::new();
    match path.select() {
        Some(path) => {
            let mut emulator = CHIP8::new();
            let _ = emulator.load(path).unwrap();
            let _ = emulator.run();
        }
        None => {}
    }
}
