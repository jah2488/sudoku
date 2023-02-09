use bevy::prelude::*;

use crate::{
    core::{cell, value::from_val},
    rsc::game_state::{Action, GameState},
};

use super::grid_update_system::GridCell;

pub fn action_system(mut game_state: ResMut<GameState>, mut cell_query: Query<&mut GridCell>) {
    match game_state.action {
        Action::ClearSelection => {
            game_state.action = Action::None;
            println!("Clearing selection");
            for mut cell in cell_query.iter_mut() {
                cell.selected = false;
            }
        }
        Action::Fill(value) => {
            game_state.action = Action::None;
            let cells = game_state.selected_cells.clone();
            for index in cells {
                //TODO: Player filled cells should be visually different
                // -- Player cannot delete starting cells
                // -- Keep track of all previous board states
                // -- Allow player to undo
                if cell_query
                    .iter()
                    .find(|c| c.index == index)
                    .unwrap()
                    .mutable
                    == true
                {
                    game_state.graph.index(index.clone()).unwrap().value = from_val(value);
                }
            }
        }
        _ => {}
    }
}
