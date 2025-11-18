use chip8::CHIP8;
use chip8::cli::RomSelector;
use chip8::emulator::errors::EmuError;

fn main() -> Result<(), EmuError> {
    let mut path = RomSelector::new();
    match path.select() {
        Some(path) => {
            let mut emulator = CHIP8::new();
            emulator.load(path)?;
            emulator.run()?;
        }
        None => {}
    }
    Ok(())
}
