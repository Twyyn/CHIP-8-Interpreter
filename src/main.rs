use chip8::cpu::{self, CPU};
use chip8::memory::Memory;
extern crate hex;

fn main() {
    let mut cpu = CPU::new();
    cpu.execute(0x000EE as u16).unwrap();


    //let rom = hex::decode(data).expect("failed");

    // let mut m = Memory::new();
    // m.load(&rom);
    // for i in 0..rom.len() {
    //     println!("{}", m.RAM[0x200 + i]);
    // }
}
