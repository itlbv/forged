mod sys;
mod render_sys;

pub use render_sys::render;
pub use crate::behavior::{behavior, react_to_events}; //behavior system
pub use sys::{input, remove_entities, update_labels_textures};