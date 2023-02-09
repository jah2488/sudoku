use bevy::prelude::*;

use crate::core::{graph::Graph, value::Value};

pub enum Modifier {
    Shift,
    Ctrl,
    Alt,
    None,
}

pub enum MouseState {
    Pressed,
    Released,
    None,
}

pub enum Tools {
    Select,
    CornerMark,
    CenterMark,
    Fill,
    Erase,
    None,
}

pub enum Action {
    CornerMark(Value),
    CenterMark(Value),
    Fill(Value),
    Erase(Value),
    ClearSelection,
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
    pub tool: Tools,
    pub modifier: Modifier,
    pub cursor_pos: u8,
    pub action: Action,
}

impl GameState {
    pub fn new(graph: Graph) -> Self {
        Self {
            current_cell: Value::Unknown,
            last_cell: Value::Unknown,
            graph: graph,
            entities: Vec::new(),
            selected_cells: Vec::new(),
            mouse: MouseState::None,
            tool: Tools::None,
            modifier: Modifier::None,
            cursor_pos: 0,
            action: Action::None,
        }
    }
}
