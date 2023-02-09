use bevy::prelude::*;

use crate::{
    core::value::to_val,
    rsc::game_state::{GameState, Modifier, MouseState},
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
        match *interaction {
            Interaction::Clicked => {
                let cell = cell_query.get_mut(parent.get()).unwrap();
                game_state.mouse = MouseState::Pressed;
                game_state.last_cell = game_state.current_cell;
                game_state.current_cell = to_val(cell.value);

                let mut cell = cell_query.get_mut(parent.get()).unwrap();
                cell.selected = true;
                println!("Clicked: {}", cell.index);

                match game_state.modifier {
                    Modifier::Shift => {}
                    _ => {
                        let cell_index = cell.index;
                        for mut c in cell_query.iter_mut() {
                            if c.index == cell_index {
                                continue;
                            } else {
                                c.selected = false;
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => match game_state.mouse {
                MouseState::Pressed => {
                    let mut cell = cell_query.get_mut(parent.get()).unwrap();
                    println!("Hovered(Pressed): {}", cell.index);
                    cell.selected = true;
                }
                MouseState::Released => {
                    let mut cell = cell_query.get_mut(parent.get()).unwrap();
                    println!("Hovered(Released): {}", cell.index);
                    cell.hovered = true;
                }
                MouseState::None => {
                    let mut cell = cell_query.get_mut(parent.get()).unwrap();
                    println!("Hovered(None): {}", cell.index);
                    cell.hovered = true;
                }
            },
            Interaction::None => {
                let mut cell = cell_query.get_mut(parent.get()).unwrap();
                cell.hovered = false;
            }
        }
    }
}
