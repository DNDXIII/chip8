extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use cpu::Cpu;

mod cpu;

fn main() {
    let mut cpu = Cpu::new();

    // Setup Graphics
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    //clear screen with white
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    canvas.fill_rect(Rect::new(10, 10, 10, 10));
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => println!("ola"),
                _ => {}
            }
        }
    }

    // Setup Input

    // Initialize CPU

    // Load Game
    cpu.load_game("./games/pong2.c8".to_string());

    loop {
        // Emulate Cycle
        cpu.emulate_cycle()

        // If Draw
        //      Draw

        // Set Keys
    }
}
