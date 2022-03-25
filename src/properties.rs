pub struct Properties {
    pub quit: bool,
    pub viewport_x: i32,
    pub viewport_y: i32,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            quit: false,
            viewport_x: 0,
            viewport_y: 0,
        }
    }
}