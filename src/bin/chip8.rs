use chip8::CHIP8;
use chip8::cli::RomSelector;
use chip8::emulator::errors::EmuError;

fn main() -> Result<(), EmuError> {
    let mut rom = RomSelector::new();
    match rom.select() {
        Some(rom) => {
            let mut emulator = CHIP8::new();
            emulator.load(rom)?;
            emulator.run()?;
        }
        None => {}
    }
    
    Ok(())
}
