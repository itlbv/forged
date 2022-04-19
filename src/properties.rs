pub struct Properties {
    pub player_id: usize,

    pub delta_time: f32,
    pub quit: bool,

    pub camera: Camera,
    pub render_flags: RenderFlags,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            player_id: 0,
            delta_time: 0.0,
            quit: false,
            camera: Camera { x: 0, y: 0, zoom: 50 },
            render_flags: RenderFlags {
                map_grid: true,
                map_tile_info: true,
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

pub struct RenderFlags {
    pub map_grid: bool,
    pub map_tile_info: bool,
    pub shapes: bool,

    pub map_textures: bool,
    pub textures: bool,
    pub labels: bool,
}