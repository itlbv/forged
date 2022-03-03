use std::any::TypeId;
use std::collections::HashMap;
use crate::components::{Color, Recipe, RenderShape};
use crate::items::{Stone, Wood};

pub fn house() -> Recipe {
    let mut ingredients_type_ids = HashMap::new();
    ingredients_type_ids.insert(TypeId::of::<Wood>(), 1);
    // ingredients_type_ids.insert(TypeId::of::<Stone>(), 1);

    Recipe::new(ingredients_type_ids,
                RenderShape::new(
                    5.0,
                    3.0,
                    Color::new(100, 100, 100))
    )
}