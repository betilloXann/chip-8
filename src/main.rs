mod chip8;
mod cpu;

use chip8::Chip8;
use std::fs;

fn main() {
    let mut chip8 = Chip8::new();

    let rom = fs::read("roms/IBM.ch8").expect("No se puede leer la ROM");

    chip8.load_rom(&rom);

    loop {
        chip8.cycle();
        chip8.tick();
    }
}
