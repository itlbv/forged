pub struct Properties {
    pub delta_time: f32,
    pub quit: bool,

    pub camera_x: i32,
    pub camera_y: i32,
    pub zoom_factor: u8,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            quit: false,
            camera_x: 0,
            camera_y: 0,
            zoom_factor: 50,
        }
    }
}