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
        1.0,
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
        1.0,
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
        1.0,
    )
}

pub fn human() -> Texture {
    Texture::new(
        String::from("villager_woman"),
        0,
        0,
        32,
        32,
        58,
        110,
        1,
        1,
        2.3,
    )
}

pub fn house() -> Texture {
    Texture::new(
        String::from("houses"),
        224,
        0,
        128,
        260,
        20,
        110,
        4,
        7,
        1.2,
    )
}