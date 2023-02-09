use bevy::prelude::*;

use crate::{
    core::value::to_val,
    rsc::game_state::{GameState, MouseState},
};

use super::grid_update_system::GridCell;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut game_state: ResMut<GameState>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Parent),
        (Changed<Interaction>, With<Button>),
    >,
    mut cell_query: Query<&mut GridCell>,
) {
    for (interaction, mut color, parent) in &mut interaction_query {
        let mut cell = cell_query.get_mut(parent.get()).unwrap();
        match *interaction {
            Interaction::Clicked => {
                game_state.last_cell = game_state.current_cell;
                game_state.current_cell = to_val(cell.value);

                cell.selected = true;
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => match game_state.mouse {
                MouseState::Pressed => {
                    cell.selected = true;
                }
                MouseState::Released => {
                    cell.hovered = true;
                }
                MouseState::None => {
                    cell.hovered = true;
                }
            },
            Interaction::None => {
                cell.hovered = false;
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
