use bevy::prelude::*;

use crate::core::{graph::Graph, value::Value};

#[derive(Resource)]
pub struct GameState {
    pub current_cell: Value,
    pub last_cell: Value,
    pub graph: Graph,
    pub grid_filled: bool,
    pub selected_cells: Vec<i32>,
}
