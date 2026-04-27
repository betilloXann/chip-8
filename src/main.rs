use chip8::Chip8;
use minifb::{Key, Window, WindowOptions};
use std::fs;
use std::time::Duration;

mod chip8;
mod cpu;

fn main() {
    //Configuracion de Pantalla
    let mut window = Window::new(
        "CHIP-8 BetilloEmulator",
        64,
        32,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));

    //Limite de FPS para ventana
    window.limit_update_rate(Some(Duration::from_micros(16667)));

    let mut chip8 = Chip8::new();
    let rom = fs::read("src/roms/BETILLO.ch8").expect("No se puede leer la ROM");
    chip8.load_rom(&rom);

    let mut buffer: Vec<u32> = vec![0; 64 * 32];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Control de velocidad de la CPU
        for _ in 0..8 {
            chip8.cycle();
        }
        // Actualizar timers
        chip8.tick();

        // Dibujar buffer
        for i in 0..chip8.display.len() {
            buffer[i] = if chip8.display[i] {
                0xFFFFFFFF
            } else {
                0x00000000
            };
        }

        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}
