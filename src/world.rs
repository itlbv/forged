use std::path::Path;
use crate::map::Map;
use crate::{entities, InputHandler, Properties, Renderer, systems};
use crate::behavior::Behavior;
use crate::components::{Food, Inventory, Label, Position, Recipe, Remove, RenderShape, Storage, Target, MainTarget, Destination, Building, Texture};
use crate::ecs::Ecs;
use crate::items::{Item, Stone, Wood};
use crate::asset_manager::AssetManager;

pub struct World<'assets> {
    pub properties: Properties,
    pub map: Map,
    pub renderer: Renderer,
    pub input_handler: InputHandler,
    pub ecs: Ecs,
    pub assets: AssetManager<'assets>,
}

impl<'assets> World<'assets> {
    pub fn new(renderer: Renderer,
               input_handler: InputHandler,
               assets: AssetManager<'assets>,
               ecs: Ecs,
               properties: Properties,
               map: Map,
    ) -> Self {
        Self {
            properties,
            map,
            renderer,
            input_handler,
            assets,
            ecs,
        }
    }

    pub fn tick(&mut self, _delta_time: f32) {
        self.properties.delta_time = 0.016; // fixed framerate for debugging

        systems::input(self);
        systems::process_events(self);
        systems::behavior(self);
        systems::update_labels_textures(self);
        systems::remove_entities(self);
        systems::render(self);
    }
}