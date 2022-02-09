use crate::ecs::Component;

pub struct Pos {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Component for Pos {}

pub struct Name {
    pub(crate) name: String,
}

impl Component for Name {}

pub struct Health {
    pub(crate) health: usize,
}

impl Component for Health {}
