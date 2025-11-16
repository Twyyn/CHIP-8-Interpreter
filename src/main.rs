use chip8::CHIP8;

fn main() {
    let mut emulator = CHIP8::new();
    //let path = "src/test_rom/test_opcode.ch8";
    //let path = "src/test_rom/7-beep.ch8";
    //let path = "src/test_rom/1-chip8-logo.ch8";
    let path = "src/test_rom/snake.ch8";
    emulator.load(path).unwrap();
    let _ = emulator.run();
}
