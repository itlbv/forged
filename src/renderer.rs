extern crate sdl2;

use crate::map::Map;
use self::sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use self::sdl2::rect::Rect;

const DEFAULT_CLEAR_FRAME_COLOR: Color = Color::RGB(50, 50, 50);

pub struct Renderer {
    sdl_canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(sdl_canvas: WindowCanvas) -> Self {
        Self { sdl_canvas }
    }

    pub fn clear_frame(&mut self) {
        self.sdl_canvas.set_draw_color(DEFAULT_CLEAR_FRAME_COLOR);
        self.sdl_canvas.clear();
    }

    pub fn present_frame(&mut self) {
        self.sdl_canvas.present();
    }

    pub fn render_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8) {
        self.sdl_canvas.set_draw_color(Color::RGB(r, g, b));
        self.sdl_canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
    }

    pub fn render_dot(&mut self, x: i32, y: i32) {
        self.sdl_canvas.set_draw_color(Color::RGB(255, 255, 255)); //white
        self.sdl_canvas.draw_point(Point::new(x, y));
    }

    pub fn world_to_screen(world: f32) -> i32 {
        (world * 50.0) as i32
    }
}