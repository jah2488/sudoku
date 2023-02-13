use bevy::{prelude::*, utils::HashSet};

use crate::{
    core::{graph::Graph, value::Value},
    sys::grid_update_system::GridCell,
};

use super::colors::Theme;

#[derive(Reflect, Clone, Debug, Default)]
pub enum Modifier {
    Shift,
    Ctrl,
    Alt,
    #[default]
    None,
}

#[derive(Reflect, Clone, Debug, Default)]
pub enum MouseState {
    Pressed,
    Released,
    #[default]
    None,
}

#[derive(Reflect, Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Tools {
    Select,
    CornerMark,
    CenterMark,
    Fill,
    Erase,
    #[default]
    None,
}

#[derive(Reflect, Clone, Debug, Default)]
pub enum Action {
    CornerMark(Value),
    CenterMark(Value),
    Fill(Value),
    Erase(Value),
    ClearSelection,
    Undo,
    Redo,
    Solve,
    Generate,
    #[default]
    None,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct GameState {
    pub action: Action,
    pub current_cell: Value,
    pub cursor_pos: u8,
    pub entities: Vec<Entity>,
    pub focus_value: Value,
    pub graph: Graph,
    pub graph_marked: Vec<GridCell>,
    pub history: Vec<(Graph, Vec<GridCell>)>,
    pub history_cursor: usize,
    pub last_cell: Value,
    pub modifier: Modifier,
    pub mouse: MouseState,
    pub selected_cells: HashSet<u8>,
    pub theme: Theme,
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
            graph_marked: Vec::new(),
            history: Vec::new(),
            history_cursor: 0,
            last_cell: Value::Unknown,
            modifier: Modifier::None,
            mouse: MouseState::None,
            selected_cells: HashSet::new(),
            theme: Theme::default_theme(),
            tool: Tools::Fill,
        }
    }

    pub fn generate(&mut self) {
        let new_graph = Graph::make_puzzle(40);
        println!("Generated new graph:\n{:?}", new_graph);
        self.graph_marked = Vec::new();
        self.history = Vec::new();
        self.history_cursor = 0;
        self.graph = new_graph;
    }

    pub fn solve(&mut self) {
        println!("Solving Graph from: \n{:?}", self.graph);
        self.snapshot();
        self.graph.generate();
        println!("Solved graph:\n{:?}", self.graph);
    }

    pub fn snapshot(&mut self) {
        self.history.truncate(self.history_cursor + 1);
        self.history
            .push((self.graph.clone(), self.graph_marked.clone()));
        self.history_cursor = self.history.len() - 1;
    }

    pub fn undo(&mut self) {
        if self.history_cursor > 0 {
            self.history_cursor -= 1;
            self.graph = self.history[self.history_cursor].0.clone();
            self.graph_marked = self.history[self.history_cursor].1.clone();
        }
    }

    pub fn redo(&mut self) {
        if self.history_cursor < self.history.len() - 1 {
            self.history_cursor += 1;
            self.graph = self.history[self.history_cursor].0.clone();
            self.graph_marked = self.history[self.history_cursor].1.clone();
        }
    }
}
