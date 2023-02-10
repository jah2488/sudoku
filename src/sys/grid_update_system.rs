use bevy::prelude::*;

use crate::{core::value::to_val, rsc::game_state::GameState};

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct GridCell {
    pub index: u8,
    pub x: u8,
    pub y: u8,
    pub value: u8,
    pub mutable: bool,
    pub selected: bool,
    pub hovered: bool,
    pub invalid: bool,
}

#[derive(Component)]
pub struct GridLabel;

pub fn grid_update_system(
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut GridCell, &Children)>,
    mut b_query: Query<(&mut Button, &mut BackgroundColor, &Children)>,
    mut t_query: Query<&mut Text, With<GridLabel>>,
) {
    let invalid_cells = game_state.graph.invalid_cells();

    for (mut cell, children) in &mut query.iter_mut() {
        game_state.graph.index(cell.index).map(|gc| {
            cell.value = gc.value;
            if gc.value == 0 {
                cell.mutable = true;
            }
        });

        let invalid = invalid_cells.contains(&cell.index);

        for &child in children.iter() {
            let button = b_query.get_mut(child);
            match button {
                Ok((_, mut color, btn_children)) => {
                    *color = Color::rgb(0.15, 0.15, 0.15).into();

                    if invalid {
                        *color = Color::rgba(0.85, 0.15, 0.15, 0.80).into();
                    }

                    if game_state.cursor_pos == cell.index {
                        *color = Color::rgb(0.35, 0.15, 0.75).into();
                    }

                    if cell.hovered {
                        *color = Color::rgb(0.25, 0.25, 0.25).into();
                    }

                    if game_state.selected_cells.contains(&cell.index) {
                        *color = Color::rgb(0.35, 0.75, 0.35).into();
                    }

                    if game_state.focus_value == to_val(cell.value) && cell.value != 0 {
                        *color = Color::rgb(0.85, 0.15, 0.15).into();
                    }

                    for &btn_child in btn_children.iter() {
                        let text = t_query.get_mut(btn_child);
                        match text {
                            Ok(mut txt) => {
                                if !cell.mutable {
                                    txt.sections[0].style.color = Color::rgb(1.85, 0.05, 0.35);
                                }
                                if cell.value == 0 {
                                    txt.sections[0].value = "".to_string();
                                } else {
                                    txt.sections[0].value = cell.value.to_string();
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(err) => {
                    println!("err: {:?}", err);
                }
            }
        }
    }
}
