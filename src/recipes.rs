use std::any::TypeId;
use std::collections::HashMap;
use crate::components::{Recipe, RenderShape};
use crate::items::{Stone, Wood};
use crate::textures;
use crate::util_structs::Color;

pub fn house() -> Recipe {
    let mut ingredients_type_ids = HashMap::new();
    ingredients_type_ids.insert(TypeId::of::<Wood>(), 1);
    ingredients_type_ids.insert(TypeId::of::<Stone>(), 1);

    Recipe::new(ingredients_type_ids,
                RenderShape::new_without_offset(
                    4.0,
                    7.0,
                    Color::new(100, 100, 100, 255)),
                textures::house(),
    )
}