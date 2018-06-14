extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{thread, time};

use cpu::Cpu;

mod cpu;
mod gpu;

fn main() {
    // Setup Graphics
    let sdl_context = sdl2::init().unwrap();

    //set up cpu
    let mut cpu = Cpu::new(&sdl_context);
    cpu.gpu.clear_screen();
    cpu.load_game("./games/pong2.c8".to_string());

    let mut event_pump = sdl_context.event_pump().unwrap();

    'event: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'event,
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => cpu.key[0x1] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => cpu.key[0x4] = 1,
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => cpu.key[0x1] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => cpu.key[0x4] = 0,
                _ => {}
            }
        }

        cpu.emulate_cycle();
        cpu.gpu.render_screen();
        for _ in 0..30000 {}
    }
}
