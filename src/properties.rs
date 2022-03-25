pub struct Properties {
    pub delta_time: f32,
    pub quit: bool,
    pub viewport_x: i32,
    pub viewport_y: i32,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            quit: false,
            viewport_x: 0,
            viewport_y: 0,
        }
    }
}