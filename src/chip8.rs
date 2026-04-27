//Definicion del struct + estado

//Son los fonts (fuentes)
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

//Todo lo basico para la CPU del CHIP-8
pub struct Chip8 {
    pub memory: [u8; 4096], //Memoria RAM
    pub v: [u8; 16],        //Registros generales (V0 a VF)
    pub pc: u16,            //Contador (Program Counter) Que programa sigue
    pub stack: [u16; 16],   //Pila de llamadas
    pub sp: u8,             //Apuntador
    pub delay_timer: u8,    //Temporizador demcrementa (60 Hz)
    pub sound_timer: u8,    //Sistema de sonido
    pub i: u16,             //Registro de indice (Puntero auxiliar)
    pub display: [bool; 64 * 32], //Pantalla
                            //pub keypad: [bool; 16],           //Teclas
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            memory: [0; 4096],
            v: [0; 16],
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            i: 0,
            display: [false; 64 * 32],
            // keypad: [false; 16],
        };

        chip8.memory[..80].copy_from_slice(&FONTSET);

        chip8
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let start = 0x200;
        let end = start + data.len();

        self.memory[start..end].copy_from_slice(data)
    }
}
