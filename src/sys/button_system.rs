use bevy::prelude::*;

use crate::{
    core::value::{to_val, Value},
    evt::FocusModeEvent,
    rsc::game_state::{GameState, Modifier, MouseState},
    ui::GridButton,
};

use super::grid_update_system::GridCell;

pub fn button_system(
    mut game_state: ResMut<GameState>,
    mut focus_mode_event: EventWriter<FocusModeEvent>,
    mut interaction_query: Query<
        (&Interaction, &Parent),
        (Changed<Interaction>, With<Button>, With<GridButton>),
    >,
    mut cell_query: Query<&mut GridCell>,
) {
    for (interaction, parent) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let cell = cell_query.get_mut(parent.get()).unwrap();
                game_state.mouse = MouseState::Pressed;

                if game_state.selected_cells.contains(&cell.index) {
                    if game_state.focus_value == to_val(cell.value) {
                        game_state.focus_value = Value::Unknown;
                    } else {
                        game_state.focus_value = to_val(cell.value);
                        focus_mode_event.send(FocusModeEvent(game_state.focus_value));
                    }
                } else {
                    game_state.focus_value = Value::Unknown;
                }

                game_state.last_cell = game_state.current_cell;
                game_state.current_cell = to_val(cell.value);

                let cell = cell_query.get_mut(parent.get()).unwrap();
                game_state.selected_cells.insert(cell.index);

                match game_state.modifier {
                    Modifier::Shift => {}
                    _ => {
                        let cell_index = cell.index;
                        for c in cell_query.iter_mut() {
                            if c.index == cell_index {
                                continue;
                            } else {
                                game_state.selected_cells.retain(|&x| x != c.index);
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => match game_state.mouse {
                MouseState::Pressed => {
                    let cell = cell_query.get_mut(parent.get()).unwrap();
                    game_state.selected_cells.insert(cell.index);
                }
                MouseState::Released => {
                    let mut cell = cell_query.get_mut(parent.get()).unwrap();
                    cell.hovered = true;
                }
                MouseState::None => {
                    let mut cell = cell_query.get_mut(parent.get()).unwrap();
                    cell.hovered = true;
                }
            },
            Interaction::None => match cell_query.get_mut(parent.get()) {
                Ok(mut cell) => {
                    cell.hovered = false;
                }
                Err(_) => {}
            },
        }
    }
}
