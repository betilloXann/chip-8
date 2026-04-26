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
struct Chip8 {
    memory: [u8; 4096],       //Memoria RAM
    v: [u8; 16],              //Registros generales (V0 a VF)
    pc: u16,                  //Contador (Program Counter) Que programa sigue
    stack: [u16; 16],         //Pila de llamadas
    sp: u8,                   //Apuntador
    delay_timer: u8,          //Temporizador demcrementa (60 Hz)
    sound_timer: u8,          //Sistema de sonido
    i: u16,                   //Registro de indice (Puntero auxiliar)
    display: [bool; 64 * 32], //Pantalla
    keypad: [bool; 16],       //Teclas
}

impl Chip8 {
    fn new() -> Self {
        let mut new_chip8 = Self {
            memory: [0; 4096],
            v: [0; 16],
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            i: 0,
            display: [false; 64 * 32],
            keypad: [false; 16],
        };

        new_chip8.memory[..80].copy_from_slice(&FONTSET);

        new_chip8
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let start = 0x200;
        let end = start + data.len();

        self.memory[start..end].copy_from_slice(data)
    }
}
