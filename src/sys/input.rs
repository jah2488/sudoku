use bevy::prelude::*;

use crate::rsc::game_state::{GameState, MouseState};

pub fn mouse_system(buttons: Res<Input<MouseButton>>, mut game_state: ResMut<GameState>) {
    if buttons.just_pressed(MouseButton::Left) {
        game_state.mouse = MouseState::Pressed;
    }

    if buttons.just_released(MouseButton::Left) {
        game_state.mouse = MouseState::Released;
    }
}

pub fn keyboard_system(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.mouse = MouseState::None;
    }

    if keyboard.just_released(KeyCode::Space) {
        //change tool/selection mode.
        game_state.mouse = MouseState::Released;
    }

    //todo: add arrow keys and num keys, also add all modifier keys (shift, ctrl, alt, etc.)
}
