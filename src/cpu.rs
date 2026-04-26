//Lógica del CPU (fetch, decode, execute)
impl Chip8 {
    fn fetch(&mut self) -> u16 {
        let firts = self.memory[self.pc as usize]; // 170
        let second = self.memory[(self.pc + 1) as usize]; // 187

        let opcode = ((first as u16) << 8) | (second as u16);

        self.pc += 2;

        opcode
    }

    fn decode() {}

    fn execute() {}
}
