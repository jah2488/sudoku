use bevy::prelude::*;

use crate::rsc::game_state::GameState;

pub fn grid_fill_system(mut game_state: ResMut<GameState>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter_mut() {
        let txt = text.sections[0].value.clone();
        let val = txt.parse::<u8>();

        match val {
            Ok(val) => {
                let cell = game_state.graph.index(val);

                match cell {
                    Some(cell) => {
                        text.sections[0].value = cell.value.to_string();
                    }
                    None => {}
                }
            }
            Err(_) => {}
        }
    }
}
