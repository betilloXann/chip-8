//Todo lo basico para la CPU del CHIP-8

struct Chip8 {
    memory: [u8; 4096], //Memoria RAM
    v: [u8; 16],        //Registros generales (V0 a VF)
    pc: u16,            //Contador (Program Counter) Que programa sigue
    stack: [u16; 16],   //Pila de llamadas
    sp: u8,             //Apuntador
    delay_timer: u8,    //Temporizador demcrementa (60 Mz)
    sound_timer: u8,    //Sistema de sonido
    i: u16,             //Registro de indice (Puntero auxiliar)
}
