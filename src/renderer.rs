extern crate sdl2;

use crate::map::Map;
use self::sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use self::sdl2::rect::Rect;

const DEFAULT_CLEAR_FRAME_COLOR: Color = Color::RGB(50, 50, 50);

pub struct Renderer {
    sdl_canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(sdl_canvas: WindowCanvas) -> Self {
        Self { sdl_canvas }
    }

    pub fn render_map(&mut self, map: &mut Map) {
        self.sdl_canvas.set_draw_color(Color::RGB(85, 64, 50));
        for map_node in &map.nodes {
            self.sdl_canvas.fill_rect(Rect::new(map_node.x * 50, map_node.y * 50, 50, 50));
        }
    }

    pub fn clear_frame(&mut self) {
        self.sdl_canvas.set_draw_color(DEFAULT_CLEAR_FRAME_COLOR);
        self.sdl_canvas.clear();
    }

    pub fn present_frame(&mut self) {
        self.sdl_canvas.present();
    }

    pub fn render_rect(&self, x: i32, y: i32, w: i32, h: i32) {

    }
}