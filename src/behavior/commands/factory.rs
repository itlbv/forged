use crate::behavior::commands::commands::MoveToSpotCommand;

pub fn move_to_spot(x: f32, y: f32) -> Box<MoveToSpotCommand> {
    MoveToSpotCommand::boxed(x, y)
}