
use crate::map::Map;
use crate::{InputHandler, Properties, Renderer};


use crate::ecs::Ecs;

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
    pub fn new(
        properties: Properties,
        map: Map,
        ecs: Ecs,
        asset_manager: AssetManager<'assets>,
        renderer: Renderer,
        input_handler: InputHandler,
    ) -> Self {
        Self {
            properties,
            map,
            renderer,
            input_handler,
            assets: asset_manager,
            ecs,
        }
    }
}