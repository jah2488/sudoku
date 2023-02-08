use bevy::prelude::*;

use crate::{core::value::to_val, rsc::game_state::GameState};

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut game_state: ResMut<GameState>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let text = text_query
                    .get_mut(children[0])
                    .unwrap()
                    .into_inner()
                    .sections[0]
                    .value
                    .clone();
                game_state.last_cell = game_state.current_cell;
                game_state.current_cell = to_val(text.parse::<u8>().unwrap());

                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // let text = text_query
                //     .get_mut(children[0])
                //     .unwrap()
                //     .into_inner()
                //     .sections[0]
                //     .value
                //     .clone();

                // let cell = game_state.graph.index(text.parse::<u8>().unwrap());

                // match cell {
                //     Some(cell) => {
                //         // println!("[{}]: ({},{})=>{}", text, cell.x, cell.y, cell.value);
                //         text_query.get_mut(children[0]).unwrap().sections[0].value =
                //             cell.value.to_string();
                //     }
                //     None => {}
                // }

                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
