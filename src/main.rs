mod core;
mod evt;
mod rsc;
mod sys;
mod ui;
use crate::core::graph::Graph;
use crate::rsc::game_state::GameState;
use crate::rsc::game_state::Tools;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode, winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::{ToolButton, ToolLabel};

fn main() {
    println!("Welcome to Sudoku!");
    /*
       TODO: --
       TODO: -- Add UI for center marks
       TODO: -- Add UI to generate a new puzzle with a given difficulty
       TODO: -- Add UI to solve the current puzzle
       TODO: -- Add UI to undo/redo
       TODO: -- Add UI to clear/reset the current puzzle
       TODO: -- Add UI for focus modes
       TODO: -- Add keyboard shortcuts for tools
       TODO: -- Extend focus mode to show everywhere a digit cannot go
       TODO: -- Add ability to "pin" a focus mode
       TODO: -- Add focus modes for rows, columns, houses, or digits
       TODO: -- Add ability to save/load puzzles
       TODO: -- Add ability to create a new puzzle
       TODO: -- Add ability to draw lines between cells
       TODO: -- Add ability to pain cells with a color
       TODO: -- Add color palettes and colour schemes
       TODO: -- Add ability to stamp shapes into cells
       TODO: -- Add note field to cells
    */

    let g = Graph::make_puzzle(25);

    println!("{:?}", g);

    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 0.0, 1.0))) //Set obnoxious clear color to ensure UI covers everything
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Sudoku!".to_string(),
                width: 1920.,
                height: 1080.,
                present_mode: PresentMode::AutoVsync,
                decorations: true,
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin)
        .add_event::<evt::ToolSelectedEvent>()
        .add_event::<evt::FocusModeEvent>()
        .add_event::<evt::CornerMarkEvent>()
        .insert_resource(GameState::new(g))
        .add_startup_system(setup)
        .add_startup_system(ui::board.before(sys::grid_fill_system::grid_fill_system))
        .add_startup_system(ui::tool_panel)
        .add_startup_system(sys::grid_fill_system::grid_fill_system)
        .add_system(sys::input::mouse_system)
        .add_system(sys::button_system::button_system)
        .add_system(sys::grid_update_system::grid_update_system)
        .add_system(sys::grid_update_system::focus_mode_system)
        .add_system(sys::grid_update_system::corner_mark_system)
        .add_system(sys::grid_update_system::corner_mark_update_system)
        .add_system(sys::text::text_update_system)
        .add_system(sys::text::text_color_system)
        .add_system(sys::input::keyboard_system)
        .add_system(sys::actions::action_system)
        .add_system(tool_panel_system)
        .add_system(tool_panel_update_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(ui::debug_panel(&asset_server));
    // commands.spawn(ui::fps(&asset_server));
}

fn tool_panel_system(
    mut game_state: ResMut<GameState>,
    mut tool_selected_event: EventWriter<evt::ToolSelectedEvent>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>, With<ToolButton>),
    >,
    mut btn_query: Query<&ToolLabel>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let label = btn_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                match label {
                    ToolLabel { tool } => {
                        game_state.tool = *tool;
                        tool_selected_event.send(evt::ToolSelectedEvent(*tool));
                    }
                }
                *color = BackgroundColor(game_state.theme.tool.selected);
            }

            Interaction::Hovered => {
                *color = BackgroundColor(game_state.theme.tool.hover);
            }
            Interaction::None => {
                if game_state.tool == label.tool {
                    *color = BackgroundColor(game_state.theme.tool.selected);
                } else {
                    *color = BackgroundColor(game_state.theme.tool.bg);
                }
            }
        }
    }
}

fn tool_panel_update_system(
    game_state: ResMut<GameState>,
    mut tool_selected_event: EventReader<evt::ToolSelectedEvent>,
    mut tool_query: Query<(&ToolButton, &mut BackgroundColor)>,
) {
    for event in tool_selected_event.iter() {
        for (tool, mut color) in &mut tool_query.iter_mut() {
            match tool {
                ToolButton(tool) => {
                    if *tool == event.0 {
                        *color = BackgroundColor(game_state.theme.tool.selected);
                    } else {
                        *color = BackgroundColor(game_state.theme.tool.bg);
                    }
                }
            }
        }
    }
}
