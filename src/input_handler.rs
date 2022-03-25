use sdl2::event::Event;
use sdl2::{EventPump, Sdl};
use sdl2::keyboard::Keycode;
use crate::{properties, Properties};

pub struct InputHandler {
    pub sdl_events: EventPump,
}

impl InputHandler {
    pub fn new(sdl: &Sdl) -> Self {
        let sdl_events = sdl.event_pump().unwrap();
        Self { sdl_events }
    }

    pub fn update(&mut self, properties: &mut Properties) -> bool {
        for event in self.sdl_events.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    properties.quit = true;
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {properties.camera_y += 20}
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {properties.camera_y -= 20}
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {properties.camera_x += 20}
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {properties.camera_x -= 20}
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {properties.zoom_factor -= 10}
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {properties.zoom_factor += 10}
                _ => {}
            }
        }

        false
    }
}