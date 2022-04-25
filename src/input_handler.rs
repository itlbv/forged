
use sdl2::event::Event;
use sdl2::{EventPump, Sdl};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::{Properties, Renderer};
use crate::ecs::Ecs;
use crate::map::Map;
use crate::properties::Camera;
use crate::util::map_util;

pub struct InputHandler {
    pub sdl_events: EventPump,
}

impl InputHandler {
    pub fn new(sdl: &Sdl) -> Self {
        let sdl_events = sdl.event_pump().unwrap();
        Self { sdl_events }
    }

    pub fn update(&mut self, properties: &mut Properties, map: &Map, ecs: &Ecs) {
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

                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => { left_mouse_click(x, y, map, properties, ecs) }
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, x, y, .. } => { right_mouse_click(x, y) }
                _ => {}
            }
        }
    }
}

fn left_mouse_click(x_screen: i32, y_screen: i32, map: &Map, properties: &mut Properties, ecs: &Ecs) {
    let x = screen_to_world(x_screen, properties.camera.x, properties.camera.zoom);
    let y = screen_to_world(y_screen, properties.camera.y, properties.camera.zoom);
    let entities_around = map.entities_around_tile(x as usize, y as usize);
    let closest_entity = map_util::entity_closest_to_pos(x, y, entities_around, ecs);
    properties.selected_entity = closest_entity;
}

fn right_mouse_click(_x: i32, _y: i32) {}

fn screen_to_world(screen: i32, camera: i32, zoom: usize) -> f32 {
    ((screen - camera) as f32 / zoom as f32) as f32
}
