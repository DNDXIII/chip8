use gpu::Gpu;
use rand::{thread_rng, Rng};
use sdl2;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub struct Cpu {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    pc: usize,
    sp: usize,
    i: usize,
    stack: [u16; 16],

    //timers
    dt: u8,
    st: u8,

    //keypad
    pub key: [u8; 16],

    //display
    pub gpu: Gpu,
}

impl Cpu {
    pub fn new(_sdlcontext: &sdl2::Sdl) -> Cpu {
        let mut cpu = Cpu {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            key: [0; 16],
            gpu: Gpu::new(_sdlcontext),
        };

        //fill the fontset
        for j in 0..80 {
            cpu.memory[j] = FONTSET[j];
        }

        cpu
    }

    pub fn emulate_cycle(&mut self) {
        self.fetch_opcode();

        self.execute_opcode();

        // Update timers
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // println!("BEEP");
            }
            self.st -= 1;
        }
    }

    pub fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    pub fn execute_opcode(&mut self) {
        /*
        println!("opcode : {:x?}  pc {:x?}", self.opcode, self.pc);
        for i in 0..15 {
            print!("v{}: {:x?} | ", i, self.v[i]);
        }
        println!("");
        println!("i: {:x?}", self.i);
        */

        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x000F {
                0x0000 => {
                    self.gpu.clear_screen();
                    self.pc += 2;
                }
                0x000E => {
                    //RET
                    self.sp -= 1;
                    self.pc = self.stack[self.sp] as usize;
                    self.pc += 2;
                }
                _ => {
                    println!("Not Implemented {:x?}", self.opcode);
                    process::exit(0x0100);
                }
            },

            0x1000 => {
                //JP addr
                self.pc = (self.opcode & 0x0FFF) as usize;
            }

            0x2000 => {
                //CALL addr
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
                self.pc = (self.opcode & 0x0FFF) as usize;
            }

            0x3000 => {
                //SE Vx byte
                let x = self.op_x();
                if self.v[x] == (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            0x4000 => {
                //SNE Vx, byte
                if self.v[self.op_x()] != (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            0x5000 => {
                //SE Vx, Vy
                let x = self.op_x();
                let y = self.op_y();

                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }

                self.pc += 2;
            }

            0x6000 => {
                //LD Vx, byte
                self.v[self.op_x()] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }

            0x7000 => {
                //ADD Vx, byte
                self.v[self.op_x()] = self.v[self.op_x()]
                    .overflowing_add((self.opcode & 0x00FF) as u8)
                    .0;

                self.pc += 2;
            }

            0x8000 => match self.opcode & 0x000F {
                0x0000 => {
                    // Set Vx = Vy
                    self.v[self.op_x()] = self.v[self.op_y()];
                    self.pc += 2;
                }

                0x0002 => {
                    // Set Vx = Vx AND Vy
                    self.v[self.op_x()] &= self.v[self.op_y()];
                    self.pc += 2;
                }

                0x0004 => {
                    // Set Vx = Vx + Vy, set VF = carry.
                    // (result, overflow)
                    let res = self.v[self.op_x()].overflowing_add(self.v[self.op_y()]);

                    self.v[15] = if res.1 { 1 } else { 0 };
                    self.v[self.op_x()] = res.0;
                    self.pc += 2;
                }

                0x0005 => {
                    //Set Vx = Vx - Vy, set VF = NOT borrow
                    let res = self.v[self.op_x()].overflowing_sub(self.v[self.op_y()]);

                    self.v[15] = if res.1 { 0 } else { 1 };
                    self.v[self.op_x()] = res.0;
                    self.pc += 2;
                }

                _ => {
                    println!("Not Implemented {:x?}", self.opcode);
                    process::exit(0x0100);
                }
            },

            0xA000 => {
                //LD I, addr
                self.i = (self.opcode & 0x0FFF) as usize;
                self.pc += 2;
            }

            0xC000 => {
                //RND Vx, byte
                let random_number: u8 = thread_rng().gen();
                self.v[self.op_x()] = (self.opcode & 0x00FF) as u8 & random_number;
                self.pc += 2;
            }

            0xD000 => {
                //DRW Vx, Vy, nibble
                let size = self.opcode & 0x000F;
                let x = self.v[self.op_x()];
                let y = self.v[self.op_y()];
                self.v[15] = self.gpu.draw_sprite(
                    x as usize,
                    y as usize,
                    &self.memory[self.i..(self.i + (size as usize))],
                );
                self.pc += 2;
            }

            0xE000 => {
                match self.opcode & 0x00FF {
                    0x009E => {
                        println!("Not Implemented {:x?}", self.opcode);
                    }

                    0x00A1 => {
                        // SKNP Vx
                        if self.key[self.v[self.op_x()] as usize] == 0 {
                            self.pc += 2;
                        }
                        self.pc += 2;
                    }

                    _ => {
                        println!("Not Implemented {:x?}", self.opcode);
                        process::exit(0x0100);
                    }
                }
            }

            0xF000 => match self.opcode & 0x00FF {
                0x0007 => {
                    //LD Vx, DT
                    self.v[self.op_x()] = self.dt;
                    self.pc += 2;
                }

                0x0015 => {
                    //LD DT, Vx
                    self.dt = self.v[self.op_x()];
                    self.pc += 2;
                }

                0x0029 => {
                    //LD F, Vx
                    self.i = (self.v[self.op_x()] as usize) * 5;
                    self.pc += 2;
                }

                0x0033 => {
                    //LD B, Vx
                    let val = self.v[self.op_x()];

                    self.memory[self.i] = val / 100;
                    self.memory[self.i + 1] = val / 10 % 10;
                    self.memory[self.i + 2] = val % 100 % 10;

                    self.pc += 2;
                }

                0x0065 => {
                    //LD Vx, [I]
                    let n = self.op_x();

                    for i in 0..n {
                        self.v[i] = self.memory[self.i + i];
                    }
                    self.i += n + 1;
                    self.pc += 2;
                }
                0x0018 => {
                    // Set sound timer = Vx
                    self.st = self.v[self.op_x()];
                    self.pc += 2;
                }

                _ => {
                    println!("Not Implemented {:x?}", self.opcode);
                    process::exit(0x0100);
                }
            },

            _ => {
                println!("Not Implemented {:x?}", self.opcode);
                process::exit(0x0100);
            }
        }
    }

    fn op_x(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8) as usize
    }
    fn op_y(&self) -> usize {
        ((self.opcode & 0x00F0) >> 4) as usize
    }

    pub fn load_game(&mut self, s: String) {
        let mut f = File::open(s).unwrap();
        let mut buffer = Vec::new();

        match f.read_to_end(&mut buffer) {
            Ok(result) => {
                println!("Read {} bytes.", result);
                //fill memory with the game
                for i in 0..result {
                    self.memory[self.pc] = buffer[i];
                    self.pc += 1;
                }
            }
            Err(e) => println!("Error Reading {}", e),
        }

        self.pc = 0x200;
    }
}

static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];
