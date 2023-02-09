use bevy::prelude::*;

use crate::rsc::game_state::GameState;

#[derive(Component)]
pub struct GridCell {
    pub index: u8,
    pub x: u8,
    pub y: u8,
    pub value: u8,
    pub selected: bool,
    pub hovered: bool,
}

#[derive(Component)]
pub struct GridLabel;

pub fn grid_update_system(
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut GridCell, &Children)>,
    mut b_query: Query<(&mut Button, &mut BackgroundColor, &Children)>,
    mut t_query: Query<&mut Text, With<GridLabel>>,
) {
    for (mut cell, children) in &mut query.iter_mut() {
        game_state.graph.index(cell.index).map(|gc| {
            cell.value = gc.value;
        });

        for &child in children.iter() {
            let button = b_query.get_mut(child);
            match button {
                Ok((_, mut color, btn_children)) => {
                    *color = Color::rgb(0.15, 0.15, 0.15).into();

                    if game_state.cursor_pos == cell.index {
                        *color = Color::rgb(0.35, 0.15, 0.75).into();
                    }

                    if cell.hovered {
                        *color = Color::rgb(0.25, 0.25, 0.25).into();
                    }

                    if cell.selected {
                        *color = Color::rgb(0.35, 0.75, 0.35).into();
                    }

                    for &btn_child in btn_children.iter() {
                        let text = t_query.get_mut(btn_child);
                        match text {
                            Ok(mut txt) => {
                                txt.sections[0].value = cell.value.to_string();
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
