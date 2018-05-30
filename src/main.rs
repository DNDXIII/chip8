use cpu::Cpu;

mod cpu;

fn main() {
    let mut cpu = Cpu::new();

    // Setup Graphics

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
