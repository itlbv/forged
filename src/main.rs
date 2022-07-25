extern crate core;

mod map;
mod renderer;
mod input_handler;
mod ecs;
mod components;
mod systems;
mod constants;
mod world;
mod entities;
mod recipes;
mod items;
mod asset_manager;
mod textures;
mod properties;
mod util;
mod behavior;
mod actions;

use std::path::Path;
use std::time::{Duration, Instant};
use sdl2::render::BlendMode::Blend;
use crate::asset_manager::AssetManager;
use crate::behavior::Behavior;
use crate::components::{Building, Destination, Food, Inventory, Label, MainTarget, Position, Recipe, Remove, RenderShape, Storage, Target, Texture};
use crate::constants::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use crate::ecs::Ecs;
use crate::input_handler::InputHandler;
use crate::items::{Item, Stone, Wood};
use crate::map::Map;
use crate::properties::Properties;
use crate::renderer::Renderer;
use crate::world::World;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let sdl_video = sdl_context.video()?;
    let sdl_window = sdl_video
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build().map_err(|e| e.to_string())?;

    let mut sdl_canvas = sdl_window
        .into_canvas()
        .present_vsync()
        .build().map_err(|e| e.to_string())?;
    sdl_canvas.set_blend_mode(Blend);

    let sdl_texture_creator = sdl_canvas.texture_creator();
    let sdl_ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut asset_manager = AssetManager::new(&sdl_texture_creator, &sdl_ttf_context);

    let renderer = Renderer::new(sdl_canvas);

    let sdl_event_pump = sdl_context.event_pump()?;
    let input_handler = InputHandler::new(sdl_event_pump);

    let properties = Properties::new();
    let map = Map::new();
    let mut ecs = Ecs::new();

    load_assets(&mut asset_manager);
    register_components(&mut ecs);
    create_entities(&mut ecs, &map);

    let mut world = World::new(
        properties,
        map,
        ecs,
        asset_manager,
        renderer,
        input_handler,
    );

    let mut instant = Instant::now();
    while !&world.properties.quit {
        world.properties.delta_time = if world.properties.fixed_framerate {
            0.016
        } else {
            let frame_time = Instant::now() - instant;
            if frame_time < Duration::from_millis(16) { continue; }
            instant = Instant::now();
            frame_time.as_millis() as f32 / 1000.0
        };

        systems::input(&mut world);
        systems::process_events(&mut world);
        systems::behavior(&mut world);
        systems::update_labels_textures(&mut world);
        systems::remove_entities(&mut world);
        systems::render(&mut world);
    }

    Ok(())
}

fn load_assets(asset_manager: &mut AssetManager) {
    asset_manager.load_texture("map_tileset", Path::new("assets/map/CL_MainLev.png"));
    asset_manager.load_texture("crops", Path::new("assets/CL_Crops_Mining.png"));
    asset_manager.load_texture("villager_woman", Path::new("assets/MiniVillagerWoman.png"));
    asset_manager.load_texture("houses", Path::new("assets/houses.png"));
}

fn register_components(ecs: &mut Ecs) {
    ecs.register_component::<Position>();
    ecs.register_component::<Label>();
    ecs.register_component::<RenderShape>();
    ecs.register_component::<Behavior>();
    ecs.register_component::<Food>();
    ecs.register_component::<Remove>();
    ecs.register_component::<Target>();
    ecs.register_component::<Destination>();
    ecs.register_component::<MainTarget>();
    ecs.register_component::<Item>();
    ecs.register_component::<Wood>();
    ecs.register_component::<Stone>();
    ecs.register_component::<Inventory>();
    ecs.register_component::<Storage>();
    ecs.register_component::<Recipe>();
    ecs.register_component::<Building>();
    ecs.register_component::<Texture>();
}

fn create_entities(ecs: &Ecs, map: &Map) {
    entities::human(1.5, 1.5, "Alice", ecs, map);

    entities::food(5, 8, ecs, map);
    entities::food(4, 1, ecs, map);
    entities::food(2, 5, ecs, map);
    entities::food(9, 6, ecs, map);
    entities::food(6, 6, ecs, map);
    entities::food(5, 7, ecs, map);

    entities::tree(3, 4, ecs, map);
    entities::tree(7, 1, ecs, map);
    entities::tree(8, 2, ecs, map);
    entities::tree(1, 3, ecs, map);
    entities::tree(3, 2, ecs, map);
    entities::tree(5, 3, ecs, map);
    entities::tree(6, 2, ecs, map);

    entities::stone(13, 6, ecs, map);
    entities::stone(14, 9, ecs, map);
    entities::stone(12, 7, ecs, map);
}
