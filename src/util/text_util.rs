use crate::asset_manager::AssetManager;

pub fn update_text_in_asset_manager(text_id: &String, text: &String, asset_manager: &mut AssetManager) {
    asset_manager.insert_text_texture(text, text_id);
}