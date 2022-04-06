extern crate sdl2;

use self::sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::BlendMode::Blend;
use sdl2::render::Texture;
use sdl2::Sdl;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::{Properties, World};
use self::sdl2::rect::Rect;

const DEFAULT_CLEAR_FRAME_COLOR: Color = Color::RGB(50, 50, 50);

pub struct Renderer {
    pub sdl_canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(sdl: &Sdl) -> Self {
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem.window("Forged", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build().unwrap();

        let mut sdl_canvas = window.into_canvas()
            .present_vsync()
            .build().unwrap();
        sdl_canvas.set_blend_mode(Blend);
        Self { sdl_canvas }
    }

    pub fn start_frame(&mut self, _properties: &Properties) {
        self.sdl_canvas.set_draw_color(DEFAULT_CLEAR_FRAME_COLOR);
        self.sdl_canvas.clear();
    }

    pub fn present_frame(&mut self) {
        self.sdl_canvas.present();
    }

    pub fn render_rect(&mut self,
                       x_world: f32, y_world: f32,
                       w_world: f32, h_world: f32,
                       color: (u8, u8, u8, u8),
                       camera: (i32, i32, usize),
    ) {
        self.sdl_canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, color.3));

        let x = Renderer::world_to_screen(x_world, camera.2) + camera.0;
        let y = Renderer::world_to_screen(y_world, camera.2) + camera.1;
        let w = Renderer::world_to_screen(w_world, camera.2);
        let h = Renderer::world_to_screen(h_world, camera.2);


        self.sdl_canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
    }

    pub fn render_line(&mut self,
                       start: (f32, f32),
                       end: (f32, f32),
                       color: (u8, u8, u8, u8),
                       camera: (i32, i32, usize),
    ) {
        self.sdl_canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, color.3));

        let start_x = Renderer::world_to_screen(start.0, camera.2) + camera.0;
        let start_y = Renderer::world_to_screen(start.1, camera.2) + camera.1;
        let end_x = Renderer::world_to_screen(end.0, camera.2) + camera.0;
        let end_y = Renderer::world_to_screen(end.1, camera.2) + camera.1;

        self.sdl_canvas.draw_line(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
        );
    }

    pub fn render_dot(&mut self, x: i32, y: i32) {
        self.sdl_canvas.set_draw_color(Color::RGB(255, 255, 255)); //white
        self.sdl_canvas.draw_point(Point::new(x, y));
    }

    pub fn render_texture(&mut self,
                          texture: &Texture,
                          sprite_x: i32, sprite_y: i32, sprite_w: u32, sprite_h: u32,
                          pos_x: i32, pos_y: i32, w: u32, h: u32,
    ) {
        self.sdl_canvas.copy(texture,
                             Rect::new(sprite_x, sprite_y, sprite_w, sprite_h),
                             Rect::new(pos_x, pos_y, w, h));
    }

    pub fn world_to_screen(world: f32, zoom_factor: usize) -> i32 {
        (world * zoom_factor as f32) as i32
    }
}