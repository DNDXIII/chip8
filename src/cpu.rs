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
    key: [u8; 16],

    //graphics
    gfx: [u8; 2048],
    draw_flag: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
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
            gfx: [0; 2048],
            draw_flag: true,
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
                println!("BEEP");
            }
            self.st -= 1;
        }
    }

    pub fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    pub fn execute_opcode(&mut self) {
        println!("opcode : {:x?}  pc {}", self.opcode, self.pc);
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x000F {
                0x0000 => {
                    //clear screen
                    for i in 0..2048 {
                        self.gfx[i] = 0;
                    }
                    self.draw_flag = true;
                    self.pc += 2;
                }
                0x000E => {
                    //RET
                    self.sp -= 1;
                    self.pc = self.stack[self.sp] as usize;
                    self.pc += 2;
                }
                _ => println!("Not Implemented {}", self.opcode),
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
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                if self.v[x] == (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            0x4000 => {
                //SNE Vx, byte
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                if self.v[x] != (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            }

            0x5000 => {
                //SE Vx, Vy
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                let y = ((self.opcode & 0x00F0) >> 4) as usize;

                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }

                self.pc += 2;
            }

            0x6000 => {
                //LD Vx, byte
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                self.v[x] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }

            0x7000 => {
                //ADD Vx, byte
                let x = ((self.opcode & 0x0F00) >> 8) as usize;
                self.v[x] += (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }

            _ => {
                println!("Not Implemented {:x?}", self.opcode);
                process::exit(0x0100);
            }
        }
        self.pc += 2;
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
