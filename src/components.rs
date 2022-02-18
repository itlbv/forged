pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub struct Name {
    pub(crate) v: String,
}

pub struct RenderShape {
    pub(crate) w: f32,
    pub(crate) h: f32,
    pub(crate) color: Color,
}

pub struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}
