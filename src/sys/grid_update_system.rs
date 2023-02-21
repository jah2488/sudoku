use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    core::value::{from_val, to_val, Value},
    evt::{CornerMarkEvent, FocusModeEvent},
    rsc::game_state::{GameState, Markers},
    ui::{GridMark, Location},
};

#[derive(Component, Clone, Debug, Default)]
pub struct GridCell {
    pub index: u8,
    pub x: u8,
    pub y: u8,
    pub corner_marks: Vec<u8>,
    pub center_marks: Vec<u8>,
    pub value: u8,
    pub mutable: bool,
    pub selected: bool,
    pub focused: bool,
    pub hovered: bool,
    pub invalid: bool,
}

#[derive(Component)]
pub struct GridLabel;

pub fn focus_mode_system(
    mut game_state: ResMut<GameState>,
    mut focus_mode_event: EventReader<FocusModeEvent>,
    mut query: Query<&mut GridCell>,
) {
    for event in focus_mode_event.iter() {
        match event {
            FocusModeEvent(value) => {
                for mut cell in query.iter_mut() {
                    let gc = game_state.graph.index(cell.index).unwrap().to_owned();
                    for p in &gc.neighbors {
                        let neighbor = game_state.graph.at(p.x, p.y).unwrap();
                        if neighbor.value == from_val(*value) {
                            cell.focused = true;
                        }
                    }
                }
            }
        }
    }
}

pub fn corner_mark_system(
    mut corner_mark_event: EventReader<CornerMarkEvent>,
    mut query: Query<(&mut Text, &Location), With<GridLabel>>,
) {
    for event in corner_mark_event.iter() {
        let mut text = query.iter_mut().find(|t| t.1 .0 == event.0).unwrap().0;
        text.sections[0].value = event.1.to_string();
    }
}

pub fn corner_mark_update_system(
    mut query: Query<(&mut Text, &Location, &Name), With<GridLabel>>,
    mut cell_query: Query<&mut GridCell>,
) {
    for (mut text, loc, name) in &mut query.iter_mut() {
        let gc = cell_query.iter_mut().find(|c| c.index == loc.0).unwrap();
        if gc.value == 0 && gc.corner_marks.len() > 0 {
            let mut sorted = gc.corner_marks.clone();
            sorted.sort();
            let corner_marks: Vec<String> = sorted.iter_mut().map(|n| n.to_string()).collect();
            text.sections[0].value = match name.as_str() {
                "TL" => corner_marks.get(0),
                "TR" => corner_marks.get(1),
                "BL" => corner_marks.get(2),
                "BR" => corner_marks.get(3),
                // "Center" => Some(
                //     gc.center_marks
                //         .iter()
                //         .map(|n| n.to_string())
                //         .collect::<Vec<String>>()
                //         .join(""),
                // ),
                _ => None,
            }
            .unwrap_or(&"".to_string())
            .to_string();
        }
    }
}

pub fn grid_update_system(
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut GridCell, &Children)>,
    mut b_query: Query<(&mut Button, &mut BackgroundColor, &Children)>,
    mut t_query: Query<&mut Text, With<GridLabel>>,
    mut m_query: Query<(&mut GridMark, &mut Style, &Children)>,
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
                    *color = game_state.theme.grid.bg.into();

                    if cell.focused {
                        *color = game_state.theme.grid.focused.into();
                    }

                    if invalid {
                        *color = game_state.theme.grid.invalid.into();
                    }

                    if game_state.cursor_pos == cell.index {
                        *color = game_state.theme.grid.cursor.into();
                    }

                    if cell.hovered {
                        *color = game_state.theme.grid.hover.into();
                    }

                    if game_state.selected_cells.contains(&cell.index) {
                        *color = game_state.theme.grid.selected.into();
                    }

                    if game_state.focus_value == to_val(cell.value) && cell.value != 0 {
                        *color = game_state.theme.grid.focused.into();
                    }

                    for &btn_child in btn_children.iter() {
                        let text = t_query.get_mut(btn_child);
                        match text {
                            Ok(mut txt) => {
                                if !cell.mutable {
                                    txt.sections[0].style.color = game_state.theme.grid.text.into();
                                }
                                if cell.value == 0 {
                                    txt.sections[0].value = "".to_string();
                                } else {
                                    txt.sections[0].value = cell.value.to_string();
                                }
                            }
                            Err(_) => {}
                        }

                        for (mut grid_marker, mut style, children) in &mut m_query.iter_mut() {
                            if cell.value == 0 {
                                style.display = Display::Flex;
                            } else {
                                style.display = Display::None;
                            }

                            // for &child in children.iter() {
                            //     match t_query.get_mut(child) {
                            //         Ok(mut txt) => {
                            //             if cell.value == 0 {
                            //                 match grid_marker.0 {
                            //                     Markers::TL => {
                            //                         if cell.corner_marks.len() > 0 {
                            //                             txt.sections[0].value =
                            //                                 cell.corner_marks[0].to_string();
                            //                         }
                            //                     }
                            //                     Markers::TR => {
                            //                         if cell.corner_marks.len() > 1 {
                            //                             txt.sections[0].value =
                            //                                 cell.corner_marks[1].to_string();
                            //                         }
                            //                     }
                            //                     Markers::BL => {
                            //                         if cell.corner_marks.len() > 2 {
                            //                             txt.sections[0].value =
                            //                                 cell.corner_marks[2].to_string();
                            //                         }
                            //                     }
                            //                     Markers::BR => {
                            //                         if cell.corner_marks.len() > 3 {
                            //                             txt.sections[0].value =
                            //                                 cell.corner_marks[3].to_string();
                            //                         }
                            //                     }
                            //                     Markers::Center => {
                            //                         if cell.center_marks.len() > 0 {
                            //                             txt.sections[0].value = cell
                            //                                 .center_marks
                            //                                 .iter()
                            //                                 .map(|x| x.to_string())
                            //                                 .collect::<Vec<String>>()
                            //                                 .join("");
                            //                         }
                            //                     }
                            //                 }
                            //             }
                            //     }
                            //     Err(_) => {}
                            // }
                            // }
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
