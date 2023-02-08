use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::rsc::game_state::GameState;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
pub struct ColorText;

pub fn text_color_system(
    time: Res<Time>,
    game_state: ResMut<GameState>,
    mut query: Query<&mut Text, With<ColorText>>,
) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        text.sections[0].value = format!(
            "C: {}\n P: {}",
            game_state.current_cell, game_state.last_cell
        );
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

pub fn text_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
