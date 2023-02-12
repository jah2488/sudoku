mod core;
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

struct ToolSelectedEvent(Tools);

fn main() {
    println!("Welcome to Sudoku!");
    /*
       TODO: -- Add UI to generate a new puzzle with a given difficulty
       TODO: -- Add UI to solve the current puzzle
       TODO: -- Add UI to undo/redo
       TODO: -- Add UI to clear/reset the current puzzle
       TODO: -- Add UI for focus modes
       TODO: -- Add UI for picking a tool
       TODO: -- -- Add keyboard shortcuts for tools
       TODO: -- Add ability to save/load puzzles
       TODO: -- Add ability to create a new puzzle
       TODO: -- Add ability to draw lines between cells
       TODO: -- Add ability to pain cells with a color
       TODO: -- Add color palettes and colour schemes
       TODO: -- Add ability to stamp shapes into cells
       TODO: -- Add note field to cells
       TODO: -- Add UI for corner marks
       TODO: -- Add UI for center marks
    */

    let g = Graph::make_puzzle(25);

    println!("{:?}", g);

    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Sudoku!".to_string(),
                width: 1920.,
                height: 1080.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin)
        .add_event::<ToolSelectedEvent>()
        .insert_resource(GameState::new(g))
        .add_startup_system(setup)
        .add_startup_system(ui::board.before(sys::grid_fill_system::grid_fill_system))
        .add_startup_system(ui::tool_panel)
        .add_startup_system(sys::grid_fill_system::grid_fill_system)
        .add_system(sys::input::mouse_system)
        .add_system(sys::button_system::button_system)
        .add_system(sys::grid_update_system::grid_update_system)
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
    commands.spawn(ui::debug_panel(&asset_server));
    commands.spawn(ui::fps(&asset_server));
}

fn tool_panel_system(
    mut game_state: ResMut<GameState>,
    mut tool_selected_event: EventWriter<ToolSelectedEvent>,
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
                        tool_selected_event.send(ToolSelectedEvent(*tool));
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
    mut game_state: ResMut<GameState>,
    mut tool_selected_event: EventReader<ToolSelectedEvent>,
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
