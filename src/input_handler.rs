use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

pub struct InputHandler {
    pub sdl_events: EventPump,
}

impl InputHandler {
    pub fn update(&mut self) -> bool{
        for event in self.sdl_events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true;
                },
                _ => {}
            }
        }

        false
    }
}