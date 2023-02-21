use std::collections::HashSet;

use bevy::prelude::*;
use bevy_inspector_egui::egui::plot::Corner;

use crate::{
    core::value::from_val,
    evt::CornerMarkEvent,
    rsc::game_state::{Action, GameState},
};

use super::grid_update_system::GridCell;
fn remove_duplicates(vector: &mut Vec<u8>) {
    let set: HashSet<_> = vector.drain(..).collect();
    vector.extend(set);
}

pub fn action_system(
    mut game_state: ResMut<GameState>,
    mut corner_mark_event: EventWriter<CornerMarkEvent>,
    mut cell_query: Query<&mut GridCell>,
) {
    match game_state.action {
        Action::ClearSelection => {
            game_state.action = Action::None;
            println!("Clearing selection");
            for mut cell in cell_query.iter_mut() {
                cell.selected = false;
                cell.focused = false;
                cell.hovered = false;
            }
        }
        Action::Fill(value) => {
            game_state.action = Action::None;
            let cells = game_state.selected_cells.clone();
            for index in cells {
                if cell_query
                    .iter()
                    .find(|c| c.index == index)
                    .unwrap()
                    .mutable
                {
                    game_state.snapshot();
                    game_state.graph.index(index.clone()).unwrap().value = from_val(value);
                }
            }
        }
        Action::CornerMark(value) => {
            game_state.action = Action::None;
            let cells = game_state.selected_cells.clone();
            for index in cells {
                let mut cell = cell_query.iter_mut().find(|c| c.index == index).unwrap();
                if cell.mutable {
                    cell.corner_marks.push(from_val(value));
                    remove_duplicates(&mut cell.corner_marks);

                    game_state.snapshot();
                    game_state.graph_marked.push(cell.clone());
                    corner_mark_event.send(CornerMarkEvent(index, value));
                }
            }
        }
        Action::Undo => {
            game_state.action = Action::None;
            game_state.undo();
        }
        Action::Redo => {
            game_state.action = Action::None;
            game_state.redo();
        }

        Action::Generate => {
            game_state.action = Action::None;
            game_state.generate();
        }

        Action::Solve => {
            game_state.action = Action::None;
            game_state.solve();
        }

        _ => {}
    }
}
