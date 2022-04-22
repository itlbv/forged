
use sdl2::event::Event;
use sdl2::{EventPump, Sdl};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::{Properties};

pub struct InputHandler {
    pub sdl_events: EventPump,
}

impl InputHandler {
    pub fn new(sdl: &Sdl) -> Self {
        let sdl_events = sdl.event_pump().unwrap();
        Self { sdl_events }
    }

    pub fn update(&mut self, properties: &mut Properties) {
        for event in self.sdl_events.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    properties.quit = true;
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { properties.camera.y += 20 }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { properties.camera.y -= 20 }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { properties.camera.x += 20 }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { properties.camera.x -= 20 }
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => { properties.camera.zoom -= 5 }
                Event::KeyDown { keycode: Some(Keycode::X), .. } => { properties.camera.zoom += 5 }

                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => { left_mouse_click(x, y) }
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, x, y, .. } => { right_mouse_click(x, y) }
                _ => {}
            }
        }
    }
}

fn left_mouse_click(_x: i32, _y: i32) {}

fn right_mouse_click(_x: i32, _y: i32) {}
