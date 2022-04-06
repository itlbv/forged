pub struct Properties {
    pub player_id: usize,

    pub delta_time: f32,
    pub quit: bool,

    pub camera: Camera,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            player_id: 0,
            delta_time: 0.0,
            quit: false,
            camera: Camera { x: 0, y: 0, zoom: 50 },
        }
    }
}

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub zoom: usize,
}