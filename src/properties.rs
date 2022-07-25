use crate::ecs::EntityId;

pub struct Properties {
    pub player_id: usize,
    pub selected_entity: Option<EntityId>,

    pub fixed_framerate: bool,
    pub delta_time: f32,
    pub quit: bool,

    pub camera: Camera,
    pub render_options: RenderOptions,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            player_id: 0,
            selected_entity: None,
            fixed_framerate: true,
            delta_time: 0.0,
            quit: false,
            camera: Camera { x: 0, y: 0, zoom: 50 },
            render_options: RenderOptions {
                map_grid: false,
                map_tile_info: false,
                shapes: true,
                map_textures: false,
                textures: true,
                labels: true,
            },
        }
    }
}

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub zoom: usize,
}

pub struct RenderOptions {
    pub map_grid: bool,
    pub map_tile_info: bool,
    pub shapes: bool,

    pub map_textures: bool,
    pub textures: bool,
    pub labels: bool,
}