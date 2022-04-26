pub trait BehaviorEvent {
    fn execute(&self);
}

pub struct OrderMove {
    pub x: f32,
    pub y: f32,
}

impl OrderMove {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl BehaviorEvent for OrderMove {
    fn execute(&self) {
        println!("event order move")
    }
}