use crate::components::Texture;

pub fn tree() -> Texture {
    Texture::new(
        String::from("crops"),
        576,
        640,
        96,
        128,
        72,
        170,
        3,
        4,
    )
}

pub fn stone() -> Texture {
    Texture::new(
        String::from("crops"),
        544,
        993,
        32,
        32,
        22,
        26,
        1,
        1,
    )
}

pub fn food() -> Texture {
    Texture::new(
        String::from("crops"),
        0,
        32,
        32,
        32,
        22,
        36,
        1,
        1,
    )
}