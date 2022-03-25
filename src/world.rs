use std::path::Path;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use crate::map::Map;
use crate::{entity_factory, InputHandler, properties, Properties, Renderer, systems};
use crate::components::{Behavior, Food, Inventory, Name, Position, Recipe, Remove, RenderShape, Storage, Target, MainTarget, Destination, Building, Texture};
use crate::ecs::Ecs;
use crate::items::{Item, Stone, Wood};
use crate::resources::AssetManager;

pub struct World<'assets> {
    pub properties: Properties,
    pub delta_time: f32,
    pub map: Map,
    pub renderer: Renderer,
    pub input_handler: InputHandler,
    pub ecs: Ecs,
    pub assets: AssetManager<'assets>,
}

impl<'assets> World<'assets> {
    pub fn new(renderer: Renderer, input_handler: InputHandler, texture_creator: &'assets TextureCreator<WindowContext>) -> Self {
        Self {
            properties: Properties::new(),
            delta_time: 0.0,
            map: Map::new(),
            renderer,
            input_handler,
            ecs: Ecs::new(),
            assets: AssetManager::new(texture_creator),
        }
    }

    pub fn setup(&mut self) {
        self.assets.load_texture("map_tileset", Path::new("assets/map/CL_MainLev.png"));
        self.assets.load_texture("crops", Path::new("assets/CL_Crops_Mining.png"));
        self.assets.load_texture("villager_woman", Path::new("assets/MiniVillagerWoman.png"));
        self.assets.load_texture("houses", Path::new("assets/houses.png"));

        self.ecs.register_component::<Position>();
        self.ecs.register_component::<Name>();
        self.ecs.register_component::<RenderShape>();
        self.ecs.register_component::<Behavior>();
        self.ecs.register_component::<Food>();
        self.ecs.register_component::<Remove>();
        self.ecs.register_component::<Target>();
        self.ecs.register_component::<Destination>();
        self.ecs.register_component::<MainTarget>();
        self.ecs.register_component::<Item>();
        self.ecs.register_component::<Wood>();
        self.ecs.register_component::<Stone>();
        self.ecs.register_component::<Inventory>();
        self.ecs.register_component::<Storage>();
        self.ecs.register_component::<Recipe>();
        self.ecs.register_component::<Building>();
        self.ecs.register_component::<Texture>();

        entity_factory::human(1.5, 1.5, "Alice", self);

        entity_factory::food(5, 8, self);
        entity_factory::food(4, 1, self);
        entity_factory::food(2, 5, self);
        entity_factory::food(9, 6, self);
        entity_factory::food(6, 6, self);
        entity_factory::food(5, 7, self);

        entity_factory::tree(3, 4, self);
        entity_factory::tree(7, 1, self);
        entity_factory::tree(8, 2, self);
        entity_factory::tree(1, 3, self);
        entity_factory::tree(3, 2, self);
        entity_factory::tree(5, 3, self);
        entity_factory::tree(6, 2, self);

        entity_factory::stone(13, 6, self);
        entity_factory::stone(14, 9, self);
        entity_factory::stone(12, 7, self);
    }

    pub fn tick(&mut self, _delta_time: f32) {
        self.delta_time = 0.016; // fixed framerate for debugging

        systems::remove_entities(self);
        systems::behavior(self);
        systems::input(self);
        systems::render(self);
    }
}