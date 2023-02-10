use bevy::prelude::*;

use crate::{
    core::value::Value,
    rsc::game_state::{Action, GameState, Modifier, MouseState, Tools},
};

pub fn mouse_system(buttons: Res<Input<MouseButton>>, mut game_state: ResMut<GameState>) {
    if buttons.just_pressed(MouseButton::Left) {
        game_state.mouse = MouseState::Pressed;
        game_state.cursor_pos = 0;
    }

    if buttons.just_released(MouseButton::Left) {
        game_state.mouse = MouseState::Released;
    }
}

pub fn keyboard_system(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.mouse = MouseState::None;
        game_state.cursor_pos = 0;
        game_state.action = Action::ClearSelection;
    }

    if keyboard.just_released(KeyCode::Space) {
        println!("Space released");
        game_state.tool = match game_state.tool {
            Tools::Select => Tools::CornerMark,
            Tools::CornerMark => Tools::CenterMark,
            Tools::CenterMark => Tools::Fill,
            Tools::Fill => Tools::Erase,
            Tools::Erase => Tools::Select,
            Tools::None => Tools::Select,
        };
    }

    if keyboard.any_just_released([
        KeyCode::LShift,
        KeyCode::RShift,
        KeyCode::LControl,
        KeyCode::RControl,
        KeyCode::LAlt,
        KeyCode::RAlt,
    ]) {
        game_state.modifier = Modifier::None;
    }

    if keyboard.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        game_state.modifier = Modifier::Shift;
    }

    if keyboard.any_pressed([KeyCode::LControl, KeyCode::RControl]) {
        game_state.modifier = Modifier::Ctrl;
    }

    if keyboard.any_pressed([KeyCode::LAlt, KeyCode::RAlt]) {
        game_state.modifier = Modifier::Alt;
    }

    if keyboard.just_pressed(KeyCode::Left) {
        if game_state.cursor_pos > 0 {
            if (game_state.cursor_pos - 1) % 9 != 0 {
                game_state.cursor_pos = game_state.cursor_pos - 1;
            }
        }
        game_state.action = Action::Undo
    }

    if keyboard.just_pressed(KeyCode::Right) {
        if game_state.cursor_pos < 80 {
            if (game_state.cursor_pos) % 9 != 0 {
                game_state.cursor_pos = game_state.cursor_pos + 1;
            }
        }
        game_state.action = Action::Redo
    }

    if keyboard.just_pressed(KeyCode::Up) {
        if game_state.cursor_pos > 8 {
            game_state.cursor_pos = game_state.cursor_pos - 9;
        }
    }

    if keyboard.just_pressed(KeyCode::Down) {
        if game_state.cursor_pos < 72 {
            game_state.cursor_pos = game_state.cursor_pos + 9;
        }
    }

    let keycodes = [
        ([KeyCode::Key1, KeyCode::Numpad1], Value::One),
        ([KeyCode::Key2, KeyCode::Numpad2], Value::Two),
        ([KeyCode::Key3, KeyCode::Numpad3], Value::Three),
        ([KeyCode::Key4, KeyCode::Numpad4], Value::Four),
        ([KeyCode::Key5, KeyCode::Numpad5], Value::Five),
        ([KeyCode::Key6, KeyCode::Numpad6], Value::Six),
        ([KeyCode::Key7, KeyCode::Numpad7], Value::Seven),
        ([KeyCode::Key8, KeyCode::Numpad8], Value::Eight),
        ([KeyCode::Key9, KeyCode::Numpad9], Value::Nine),
    ];

    for keycode_tuple in keycodes.iter() {
        if keyboard.any_just_pressed(keycode_tuple.0) {
            match game_state.tool {
                Tools::CornerMark => {
                    game_state.action = Action::CornerMark(Value::from(keycode_tuple.1));
                }
                Tools::CenterMark => {
                    game_state.action = Action::CenterMark(Value::from(keycode_tuple.1));
                }
                Tools::Fill => {
                    game_state.action = Action::Fill(Value::from(keycode_tuple.1));
                }
                Tools::Erase => {
                    game_state.action = Action::Erase(Value::from(keycode_tuple.1));
                }
                _ => {}
            }
        }
    }
}
