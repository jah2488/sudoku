use bevy::prelude::*;

use crate::core::{graph::Graph, value::Value};

pub enum MouseState {
    Pressed,
    Released,
    None,
}

#[derive(Resource)]
pub struct GameState {
    pub current_cell: Value,
    pub last_cell: Value,
    pub graph: Graph,
    pub entities: Vec<Entity>,
    pub selected_cells: Vec<i32>,
    pub mouse: MouseState,
}
