use chip8::Chip8;
use minifb::{Key, Window, WindowOptions};
use std::env;
use std::fs;
use std::time::Duration;

mod chip8;
mod cpu;

fn actualizar_teclado(window: &Window, chip8: &mut Chip8) {
    //Teclado limpio
    chip8.keypad.fill(false);

    //Obtenemos teclas presionadas actualmente
    let keys = window.get_keys();

    //Mapeado de tecla
    for key in keys {
        match key {
            Key::Key1 => chip8.keypad[0x1] = true,
            Key::Key2 => chip8.keypad[0x2] = true,
            Key::Key3 => chip8.keypad[0x3] = true,
            Key::Key4 => chip8.keypad[0xC] = true,

            Key::Q => chip8.keypad[0x4] = true,
            Key::W => chip8.keypad[0x5] = true,
            Key::E => chip8.keypad[0x6] = true,
            Key::R => chip8.keypad[0xD] = true,

            Key::A => chip8.keypad[0x7] = true,
            Key::S => chip8.keypad[0x8] = true,
            Key::D => chip8.keypad[0x9] = true,
            Key::F => chip8.keypad[0xE] = true,

            Key::Z => chip8.keypad[0xA] = true,
            Key::X => chip8.keypad[0x0] = true,
            Key::C => chip8.keypad[0xB] = true,
            Key::V => chip8.keypad[0xF] = true,

            _ => (),
        }
    }
}

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
    let args: Vec<String> = env::args().collect();
    let carpeta_roms = "roms/";
    let rom = if args.len() > 1 {
        format!("{}{}", carpeta_roms, args[1])
    } else {
        "roms/BETILLO.ch8".to_string()
    };
    let rom = fs::read(&rom).expect("Error: No se pudo encontrar o leer la ROM jeje :'(");
    chip8.load_rom(&rom);
    let mut buffer: Vec<u32> = vec![0; 64 * 32];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //Teclado
        actualizar_teclado(&window, &mut chip8);

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
