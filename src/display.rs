extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct Display {
    canvas: Canvas<Window>,
    scale: u32,
    bg_color: Color,
    fg_color: Color,
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video = sdl_context.video().unwrap();

        let scale = 10;
        let screen_width = 64 * scale;
        let screen_height = 32 * scale;

        let window = video
            .window("chip8", screen_width, screen_height)
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self {
            canvas,
            scale: 10,
            bg_color: Color::RGB(0, 0, 0),
            fg_color: Color::RGB(255, 255, 255),
        }
    }

    pub fn draw(self: &mut Display, screen: &[[bool; 32]; 64]) {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
        self.canvas.set_draw_color(self.fg_color);

        for (x, column) in screen.iter().enumerate() {
            for (y, row) in column.iter().enumerate() {
                if *row {
                    let x = ((x as u32) * self.scale) as i32;
                    let y = ((y as u32) * self.scale) as i32;
                    let width = self.scale;
                    let height = self.scale;

                    self.canvas
                        .fill_rect(Rect::new(x, y, width, height))
                        .expect("Failed to draw on screen")
                }
            }
        }

        self.canvas.present();
    }
}
