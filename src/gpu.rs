use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

static SCALE: u32 = 10;

pub struct Gpu {
    //graphics
    gfx: [u8; 2048],
    draw_flag: bool,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Gpu {
    pub fn new(_sdlcontext: &sdl2::Sdl) -> Gpu {
        let mut gpu = Gpu {
            draw_flag: true,
            gfx: [0; 2048],
            canvas: _sdlcontext
                .video()
                .unwrap()
                .window("rust-sdl2 demo: Video", 64 * SCALE, 32 * SCALE)
                .position_centered()
                .opengl()
                .build()
                .unwrap()
                .into_canvas()
                .build()
                .unwrap(),
        };

        gpu
    }

    pub fn clear_screen(&mut self) {
        //clear screen with white
        for i in 0..2048 {
            self.gfx[i] = 0
        }
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision: u8 = 0;
        let n = sprite.len() as usize;
        let start: usize = y * 64 + x;

        println!("x:{}  y:{} ", x, y);

        for j in 0..n {
            for i in 0..8 {
                if (sprite[j] & (0x80 >> i)) != 0 {
                    if self.gfx[(start + 64 * n) + i] == 1 {
                        collision = 1
                    }
                    self.gfx[(start + 64 * n) + 1] ^= 1;
                }
            }
        }

        self.draw_flag = true;
        collision
    }

    pub fn render_screen(&mut self) {
        if self.draw_flag {
            return;
        }
    }
}
