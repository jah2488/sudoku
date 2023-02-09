use bevy::prelude::*;

use crate::{
    core::value::{to_val, Value},
    rsc::game_state::{GameState, Modifier, MouseState},
};

use super::grid_update_system::GridCell;

pub fn button_system(
    mut game_state: ResMut<GameState>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Parent),
        (Changed<Interaction>, With<Button>),
    >,
    mut cell_query: Query<&mut GridCell>,
) {
    for (interaction, mut color, parent) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let cell = cell_query.get_mut(parent.get()).unwrap();
                game_state.mouse = MouseState::Pressed;

                if game_state.selected_cells.contains(&cell.index) {
                    if game_state.focus_value == to_val(cell.value) {
                        game_state.focus_value = Value::Unknown;
                    } else {
                        game_state.focus_value = to_val(cell.value);
                    }
                } else {
                    game_state.focus_value = Value::Unknown;
                }

                game_state.last_cell = game_state.current_cell;
                game_state.current_cell = to_val(cell.value);

                let cell = cell_query.get_mut(parent.get()).unwrap();
                game_state.selected_cells.push(cell.index);

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
                    game_state.selected_cells.push(cell.index);
                }
                MouseState::Released => {
                    let mut cell = cell_query.get_mut(parent.get()).unwrap();
                    cell.hovered = true;
                    *color = Color::rgb(0.85, 0.25, 0.25).into();
                }
                MouseState::None => {
                    let mut cell = cell_query.get_mut(parent.get()).unwrap();
                    cell.hovered = true;
                    *color = Color::rgb(0.25, 0.25, 0.25).into();
                }
            },
            Interaction::None => {
                let mut cell = cell_query.get_mut(parent.get()).unwrap();
                cell.hovered = false;
            }
        }
    }
}
