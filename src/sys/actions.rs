use bevy::prelude::*;

use crate::rsc::game_state::{Action, GameState};

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
        _ => {}
    }
}
