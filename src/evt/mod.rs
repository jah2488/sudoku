use crate::{
    core::value::Value,
    rsc::game_state::{Action, GameState, Modifier, MouseState, Tools},
};

pub struct ToolSelectedEvent(pub Tools);
pub struct FocusModeEvent(pub Value);

pub struct CornerMarkEvent(pub u8, pub Value);
