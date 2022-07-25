extern crate sdl2;

use self::sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureQuery};
use sdl2::Sdl;
use crate::{Properties};
use crate::properties::Camera;
use self::sdl2::rect::Rect;

const DEFAULT_CLEAR_FRAME_COLOR: Color = Color::RGB(50, 50, 50);

pub struct Renderer {
    pub sdl_canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(sdl_canvas: WindowCanvas) -> Self {
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
                       camera: &Camera,
    ) {
        self.sdl_canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, color.3));

        let x = Renderer::world_to_screen(x_world, camera.zoom) + camera.x;
        let y = Renderer::world_to_screen(y_world, camera.zoom) + camera.y;
        let w = Renderer::world_to_screen(w_world, camera.zoom);
        let h = Renderer::world_to_screen(h_world, camera.zoom);

        self.sdl_canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
    }

    pub fn render_empty_rect(&mut self,
                             x_world: f32, y_world: f32,
                             w_world: f32, h_world: f32,
                             color: (u8, u8, u8, u8),
                             camera: &Camera,
    ) {
        self.sdl_canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, color.3));

        let x = Renderer::world_to_screen(x_world, camera.zoom) + camera.x;
        let y = Renderer::world_to_screen(y_world, camera.zoom) + camera.y;
        let w = Renderer::world_to_screen(w_world, camera.zoom);
        let h = Renderer::world_to_screen(h_world, camera.zoom);

        self.sdl_canvas.draw_rect(Rect::new(x, y, w as u32, h as u32));
    }

    pub fn render_line(&mut self,
                       start: (f32, f32),
                       end: (f32, f32),
                       color: (u8, u8, u8, u8),
                       camera: &Camera,
    ) {
        self.sdl_canvas.set_draw_color(Color::RGBA(color.0, color.1, color.2, color.3));

        let start_x = Renderer::world_to_screen(start.0, camera.zoom) + camera.x;
        let start_y = Renderer::world_to_screen(start.1, camera.zoom) + camera.y;
        let end_x = Renderer::world_to_screen(end.0, camera.zoom) + camera.x;
        let end_y = Renderer::world_to_screen(end.1, camera.zoom) + camera.y;

        self.sdl_canvas.draw_line(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
        );
    }

    pub fn render_dot(&mut self,
                      world_x: f32,
                      world_y: f32,
                      camera: &Camera
    ) {
        self.sdl_canvas.set_draw_color(Color::RGB(255, 255, 255)); //white

        let x = Renderer::world_to_screen(world_x, camera.zoom) + camera.x;
        let y = Renderer::world_to_screen(world_y, camera.zoom) + camera.y;
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

    pub fn render_label_texture(&mut self,
                                label_texture: &Texture,
                                x_world: f32,
                                y_world: f32,
                                camera: &Camera,
    ) {
        let TextureQuery { width, height, .. } = label_texture.query();
        self.sdl_canvas.copy(
            &label_texture,
            None,
            // 0, 0, 500, 100,
            Some(Rect::new(
                Renderer::world_to_screen(x_world, camera.zoom) + camera.x,
                Renderer::world_to_screen(y_world, camera.zoom) + camera.y,
                width, height)),
        );
    }

    pub fn world_to_screen(world: f32, zoom_factor: usize) -> i32 {
        (world * zoom_factor as f32) as i32
    }
}