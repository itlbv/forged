use std::any::TypeId;
use std::collections::HashMap;
use crate::behavior::behaviors;

use crate::behavior::btree::BehaviorTreeNode;
use crate::ecs::EntityId;
use crate::behavior::needs::{Hunger, Need};
use crate::util::physics::Vect;
use crate::util::util_structs::Color;

pub struct Behavior {
    pub needs: Vec<Box<dyn Need>>,
    pub behavior_tree: Box<dyn BehaviorTreeNode>,
    pub state: BehaviorState,
}

impl Behavior {
    pub fn new(owner: EntityId) -> Self {
        Self {
            needs: vec![
                Box::new(Hunger::new()),
            ],
            behavior_tree: Box::new(behaviors::do_nothing()),
            state: BehaviorState::new(owner),
        }
    }
}

pub struct BehaviorState {
    pub owner: EntityId,
    pub target: Option<EntityId>,
    pub main_target: Option<EntityId>,
    pub destination: Option<Vect>,
}

impl BehaviorState {
    pub fn new(owner: EntityId) -> Self {
        Self {
            owner,
            target: None,
            main_target: None,
            destination: None,
        }
    }
}

pub struct Texture {
    pub sprite_id: String,
    pub sprite_in_tileset_x: i32,
    pub sprite_in_tileset_y: i32,
    pub sprite_w: u32,
    pub sprite_h: u32,
    pub render_offset_x: u32,
    pub render_offset_y: u32,
    pub object_w_tiles: u32,
    pub object_h_tiles: u32,
    pub scale: f32,
}

impl Texture {
    pub fn new(sprite_id: String,
               sprite_in_tileset_x: i32,
               sprite_in_tileset_y: i32,
               sprite_w: u32,
               sprite_h: u32,
               render_offset_x: u32,
               render_offset_y: u32,
               object_w_tiles: u32,
               object_h_tiles: u32,
               scale: f32,
    ) -> Self {
        Self {
            sprite_id,
            sprite_in_tileset_x,
            sprite_in_tileset_y,
            sprite_w,
            sprite_h,
            render_offset_x,
            render_offset_y,
            object_w_tiles,
            object_h_tiles,
            scale,
        }
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Self {
            sprite_id: self.sprite_id.clone(),
            sprite_in_tileset_x: self.sprite_in_tileset_x,
            sprite_in_tileset_y: self.sprite_in_tileset_y,
            sprite_w: self.sprite_w,
            sprite_h: self.sprite_h,
            render_offset_x: self.render_offset_x,
            render_offset_y: self.render_offset_y,
            object_w_tiles: self.object_w_tiles,
            object_h_tiles: self.object_h_tiles,
            scale: self.scale,
        }
    }
}

pub struct Inventory {
    pub item_carried: i32,
    pub items_needed: Vec<usize>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            item_carried: -1,
            items_needed: Vec::new(),
        }
    }
}

pub struct Building {
    pub usable: bool,
    pub entry_x: f32,
    pub entry_y: f32,
}

impl Building {
    pub fn new(entry_x: f32, entry_y: f32) -> Self {
        Self {
            usable: false,
            entry_x,
            entry_y,
        }
    }
}

pub struct Storage {
    pub items: Vec<usize>,
}

impl Storage {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

pub struct Recipe {
    pub ingredients_type_ids: HashMap<TypeId, usize>,
    pub render_shape: RenderShape,
    pub texture: Texture,
}

impl Recipe {
    pub fn new(ingredients_type_ids: HashMap<TypeId, usize>,
               render_shape: RenderShape,
               texture: Texture) -> Self {
        Self {
            ingredients_type_ids,
            render_shape,
            texture,
        }
    }
}

impl Clone for Recipe {
    fn clone(&self) -> Self {
        Self {
            ingredients_type_ids: self.ingredients_type_ids.clone(),
            render_shape: self.render_shape.clone(),
            texture: self.texture.clone(),
        }
    }
}

pub struct Destination {
    pub x: f32,
    pub y: f32,
}

impl Destination {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Target {
    pub target_id: usize,
}

impl Target {
    pub fn new(target_id: usize) -> Self {
        Self { target_id }
    }
}

pub struct MainTarget {
    pub owner: usize,
}

impl MainTarget {
    pub fn new(target_id: usize) -> Self {
        Self { owner: target_id }
    }
}

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub owner: EntityId,
}

impl Position {
    pub fn of(x: f32, y: f32, owner: EntityId) -> Self {
        Self { x, y, owner }
    }
}

pub struct Name {
    pub v: String,
}

pub struct RenderShape {
    pub w: f32,
    pub h: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub color: Color,
}

impl RenderShape {
    pub fn new_with_standard_offset(w: f32, h: f32, color: Color) -> Self {
        Self { w, h, offset_x: -w / 2.0, offset_y: -h / 2.0, color }
    }

    pub fn new_without_offset(w: f32, h: f32, color: Color) -> Self {
        Self { w, h, offset_x: 0.0, offset_y: 0.0, color }
    }

    pub fn from_recipe(recipe: &Recipe) -> Self {
        Self {
            w: recipe.render_shape.w,
            h: recipe.render_shape.h,
            offset_x: 0.0,
            offset_y: 0.0,
            color: Color::new(
                recipe.render_shape.color.r,
                recipe.render_shape.color.g,
                recipe.render_shape.color.b,
                recipe.render_shape.color.a,
            ),
        }
    }

    pub fn set_transparent(&mut self) {
        self.color.a = 140;
    }
}

impl Clone for RenderShape {
    fn clone(&self) -> Self {
        Self {
            w: self.w,
            h: self.h,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            color: Color::new(self.color.r, self.color.g, self.color.b, self.color.a),
        }
    }
}

pub struct Food {}

pub struct Remove {
    pub own_id: usize,
}

impl Remove {
    pub fn new(own_id: usize) -> Self { Self { own_id } }
}
