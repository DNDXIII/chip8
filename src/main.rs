extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::{Duration, Instant};

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
    let mut start;
    let wait = Duration::from_millis(1000 / 300);

    'event: loop {
        start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'event,

                // ---- KEY DOWN ----
                // 1234
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => cpu.key[0x0] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => cpu.key[0x1] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => cpu.key[0x2] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => cpu.key[0x3] = 1,

                // qwer
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => cpu.key[0x4] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => cpu.key[0x5] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => cpu.key[0x6] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => cpu.key[0x7] = 1,

                // asdf
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => cpu.key[0x8] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => cpu.key[0x9] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => cpu.key[0xA] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => cpu.key[0xB] = 1,

                //zxcv
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => cpu.key[0xC] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => cpu.key[0xD] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => cpu.key[0xE] = 1,
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => cpu.key[0xF] = 1,

                // ---- KEY UP ----
                // 1234
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => cpu.key[0x0] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                } => cpu.key[0x1] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                } => cpu.key[0x2] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                } => cpu.key[0x3] = 0,

                // qwer
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => cpu.key[0x4] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => cpu.key[0x5] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => cpu.key[0x6] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => cpu.key[0x7] = 0,

                // asdf
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => cpu.key[0x8] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => cpu.key[0x9] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => cpu.key[0xA] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => cpu.key[0xB] = 0,

                //zxcv
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => cpu.key[0xC] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    ..
                } => cpu.key[0xD] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => cpu.key[0xE] = 0,
                Event::KeyUp {
                    keycode: Some(Keycode::V),
                    ..
                } => cpu.key[0xF] = 0,

                _ => {}
            }
        }

        cpu.emulate_cycle();
        cpu.gpu.render_screen();

        //to keep a constant fps
        let elapsed = start.elapsed();
        if wait > elapsed {
            thread::sleep(wait - elapsed);
        }
    }
}
