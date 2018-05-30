extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

    'main: loop {
        'event: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::KeyDown { keycode, .. } => println!("ola"),
                    _ => break 'event,
                }
            }
        }

        cpu.emulate_cycle();
        cpu.gpu.render_screen();
    }
}
