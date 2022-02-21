pub struct Vec {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Vec {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn of(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn set_length(&mut self, new_length: f32) {
        let new_leg_length = new_length / self.length();
        self.x *= new_leg_length;
        self.y *= new_leg_length;
    }

    pub fn normalize(&mut self) {
        self.x /= self.length();
        self.y /= self.length();
    }
}

pub fn distance_between(vec_1: &Vec, vec_2: &Vec) -> f32 {
    (vec_1.x - vec_2.x).hypot(vec_1.y - vec_2.y)
}

pub fn vector_to(source: &Vec, target: &Vec) -> Vec {
    Vec::of(target.x - source.x, target.y - source.y)
}