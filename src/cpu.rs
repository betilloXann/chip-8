use std::usize;

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
                let nnn = opcode & 0x0FFF;

                self.pc = nnn;
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
            //
            0x4000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let valor = (opcode & 0x00FF) as u8;

                if self.v[x] != valor {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if opcode & 0x000F == 0 {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;

                    if self.v[x] == self.v[y] {
                        self.pc += 2;
                    }
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
            0x8000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;

                match opcode & 0x000F {
                    0x0 => {
                        self.v[x] = self.v[y];
                    }
                    0x1 => {
                        self.v[x] = self.v[x] | self.v[y];
                    }
                    0x2 => {
                        self.v[x] = self.v[x] & self.v[y];
                    }
                    0x3 => {
                        self.v[x] = self.v[x] ^ self.v[y];
                    }
                    0x4 => {
                        let mut u16_x = self.v[x] as u16;
                        let u16_y = self.v[y] as u16;
                        u16_x = u16_x + u16_y;

                        if u16_x > 255 {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }

                        self.v[x] = (u16_x & 0xFF) as u8;
                    }
                    0x5 => {
                        if self.v[x] >= self.v[y] {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }

                        let valor = self.v[x].wrapping_sub(self.v[y]);
                        self.v[x] = valor;
                    }
                    0x6 => {
                        let least_x = self.v[x] & 1;
                        self.v[0xF] = least_x;
                        self.v[x] = self.v[x] >> 1;
                    }
                    0x7 => {
                        if self.v[y] >= self.v[x] {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }

                        let valor = self.v[y].wrapping_sub(self.v[x]);
                        self.v[x] = valor;
                    }
                    0xE => {
                        let most = (self.v[x] >> 7) & 1;
                        self.v[0xF] = most;
                        self.v[x] = self.v[x] << 1;
                    }
                    _ => {
                        println!("Opencode 8 no implementado: {:04X}", opcode);
                    }
                }
            }
            0x9000 => {
                if opcode & 0x000F == 0 {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;

                    if self.v[x] != self.v[y] {
                        self.pc += 2;
                    }
                }
            }
            // ANNN - El mapeador
            0xA000 => {
                self.i = opcode & 0x0FFF;
            }
            0xB000 => {
                let nnn = opcode & 0x0FFF;
                let v0 = self.v[0] as u16;

                self.pc = nnn + v0;
                self.pc = nnn.wrapping_add(v0) & 0x0FFF;
            }
            0xC000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;

                self.semilla = self.semilla.wrapping_mul(1103515245).wrapping_add(12345);

                let random = (self.semilla >> 16) as u8;

                self.v[x] = random & kk;
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
            0xE000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;

                match opcode & 0x00FF {
                    0x9E => {
                        let tecla = self.v[x] as usize;

                        if self.keypad[tecla] {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        let tecla = self.v[x] as usize;

                        if !self.keypad[tecla] {
                            self.pc += 2;
                        }
                    }
                    _ => {}
                }
            }
            //0xF000 Gestion de temporizadores
            0xF000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;

                match opcode & 0x00FF {
                    0x07 => {
                        self.v[x] = self.delay_timer;
                    }
                    0x0A => {
                        let mut press = false;

                        for i in 0..self.keypad.len() {
                            if self.keypad[i] == true {
                                self.v[x] = i as u8;
                                press = true;
                                break;
                            }
                        }
                        if !press {
                            self.pc -= 2;
                        }
                    }
                    0x15 => {
                        self.delay_timer = self.v[x];
                    }
                    0x18 => {
                        self.sound_timer = self.v[x];
                    }
                    0x1E => {
                        self.i = self.i.wrapping_add(self.v[x] as u16);
                    }
                    0x29 => {
                        self.i = (self.v[x] as u16) * 5;
                    }
                    0x33 => {
                        let cen = self.v[x] / 100;
                        let dec = (self.v[x] / 10) % 10;
                        let uni = self.v[x] % 10;

                        self.memory[self.i as usize] = cen;
                        self.memory[(self.i + 1) as usize] = dec;
                        self.memory[(self.i + 2) as usize] = uni;
                    }
                    0x55 => {
                        for i in 0..=x {
                            self.memory[(self.i as usize) + i] = self.v[i];
                        }
                    }
                    0x65 => {
                        for i in 0..=x {
                            self.v[i] = self.memory[(self.i as usize) + i];
                        }
                    }
                    _ => {
                        println!("Opencode F no implementado: {:04X}", opcode);
                    }
                }
            }
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
