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
    pub action: Action,
    pub current_cell: Value,
    pub cursor_pos: u8,
    pub entities: Vec<Entity>,
    pub focus_value: Value,
    pub graph: Graph,
    pub last_cell: Value,
    pub modifier: Modifier,
    pub mouse: MouseState,
    pub selected_cells: Vec<u8>,
    pub tool: Tools,
}

impl GameState {
    pub fn new(graph: Graph) -> Self {
        Self {
            action: Action::None,
            current_cell: Value::Unknown,
            cursor_pos: 0,
            entities: Vec::new(),
            focus_value: Value::Unknown,
            graph: graph,
            last_cell: Value::Unknown,
            modifier: Modifier::None,
            mouse: MouseState::None,
            selected_cells: Vec::new(),
            tool: Tools::None,
        }
    }
}
