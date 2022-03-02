use std::any::TypeId;
use std::collections::HashMap;
use crate::components::{Color, Recipe, RenderShape};
use crate::items::{Stone, Wood};

pub fn house() -> Recipe {
    let mut ingredients_type_ids = HashMap::new();
    ingredients_type_ids.insert(TypeId::of::<Wood>(), 3);
    ingredients_type_ids.insert(TypeId::of::<Stone>(), 2);

    Recipe {
        ingredients_type_ids,
        render_shape: RenderShape {
            w: 5.0,
            h: 3.0,
            color: Color { r: 100, g: 100, b: 100 },
        },
    }
}