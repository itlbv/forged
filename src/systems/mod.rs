mod sys;
mod render_sys;

pub use render_sys::render;
pub use sys::{behavior, input, remove_entities, update_labels_textures};