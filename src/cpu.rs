use crate::chip8::Chip8;

//Lógica del CPU (fetch, decode, execute)
impl Chip8 {
    //Buscar el ingrediente
    fn fetch(&mut self) -> u16 {
        let first = self.memory[self.pc as usize];
        let second = self.memory[(self.pc + 1) as usize];

        let opcode = ((first as u16) << 8) | (second as u16);

        self.pc += 2;

        opcode
    }

    //Leer la receta
    //Es como gritar: Siguiente
    pub fn cycle(&mut self) {
        let opcode = self.fetch(); // Primero busca el ingrediente
        self.decode(opcode); // Busca entenderlo
    }

    //Piensa que hacer
    fn decode(&mut self, opcode: u16) {
        self.execute(opcode);
    }

    //Ya sabe que receta es
    fn execute(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            //Borrado de pantalla
            0x0000 => match opcode {
                0x00E0 => {
                    self.display.fill(false);
                }
                0x00EE => {
                    if self.sp == 0 {
                        panic!("Satck underflow");
                    }
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => {
                    // no - op
                }
            },
            //1NN - Salto
            0x1000 => {
                let direccion = opcode & 0x0FFF;

                self.pc = direccion;
            }
            // 2NN - call
            0x2000 => {
                if self.sp as usize >= self.stack.len() {
                    panic!("Stack overflow");
                }

                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            }
            0x3000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let valor = (opcode & 0x00FF) as u8;

                if self.v[x] == valor {
                    self.pc += 2;
                }
            }
            // 6XKK - Mochila
            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let valor = (opcode & 0x00FF) as u8;
                self.v[x] = valor;
            }
            // 7XKK - El sumador
            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let valor = (opcode & 0x00FF) as u8;

                self.v[x] = self.v[x].wrapping_add(valor);
            }
            // ANNN - El mapeador
            0xA000 => {
                self.i = opcode & 0x0FFF;
            }
            //El pintor
            0xD000 => {
                let x = self.v[((opcode & 0x0F00) >> 8) as usize] as usize;
                let y = self.v[((opcode & 0x00F0) >> 4) as usize] as usize;
                let n = (opcode & 0x000F) as usize;

                self.v[0xF] = 0;

                for fila in 0..n {
                    let byte = self.memory[self.i as usize + fila];

                    for columna in 0..8 {
                        if (byte & (0x80u8 >> columna)) != 0 {
                            let px = (x + columna) % 64;
                            let py = (y + fila) % 32;
                            let index = py * 64 + px;

                            let before = self.display[index];
                            self.display[index] ^= true;

                            if before && !self.display[index] {
                                self.v[0xF] = 1;
                            }
                        }
                    }
                }
            }
            //0xF000 Gestion de temporizadores
            0xF000 => match opcode & 0x00FF {
                0x1E => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.i = self.i.wrapping_add(self.v[x] as u16);
                }
                _ => {
                    println!("Opencode F no implementado: {:04X}", opcode);
                }
            },

            _ => {
                println!("Opcode no implementado, no lo tengo bro: {:04X}", opcode);
            }
        }
    }
    pub fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
