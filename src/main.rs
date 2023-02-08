mod core;
mod rsc;
mod sys;
use crate::core::graph::Graph;
use crate::core::value::Value;
use crate::rsc::game_state::GameState;
use crate::sys::button_system::button_system;
use crate::sys::grid_fill_system::grid_fill_system;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use sys::{
    button_system::NORMAL_BUTTON,
    text::{ColorText, FpsText},
};

fn main() {
    println!("Welcome to Sudoku!");

    let mut g = Graph::new();
    g.generate();

    println!("{:?}", g);

    App::new()
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
        .insert_resource(GameState {
            current_cell: Value::Unknown,
            last_cell: Value::Unknown,
            graph: g,
            grid_filled: false,
            selected_cells: Vec::new(),
        })
        .add_startup_system(setup)
        .add_system(grid_fill_system)
        .add_system(sys::text::text_update_system)
        .add_system(sys::text::text_color_system)
        .add_system(button_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: bevy::ui::Val::Px(5.0),
                right: bevy::ui::Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
        Name::new("Info Panel"),
    ));
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
        Name::new("FPS Text"),
    ));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(::bevy::ui::Val::Px(150.0), bevy::ui::Val::Px(65.0)),
                // center button
                margin: UiRect::all(bevy::ui::Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                border: UiRect::all(bevy::ui::Val::Px(1.0)),
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "9",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(bevy::ui::Val::Percent(100.0), bevy::ui::Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            Name::new("Grid"),
        ))
        .with_children(|parent| {
            let mut i = 1;
            let l = 100; // Left Margin
            let t = 1; // Top Margin
            let w = 100; // Width
            let h = 100; // Height
            let y = 9; // Row Length
            while i <= 81 {
                spawn_cell(
                    parent,
                    i,
                    l + (w * (1 + ((i - 1) % y))),
                    t + (h * (0 + ceil(i, y))),
                    &asset_server,
                );
                i += 1;
            }
        });
}

fn ceil(x: i32, y: i32) -> i32 {
    (x + y - 1) / y
}

fn spawn_cell(parent: &mut ChildBuilder, i: i32, x: i32, y: i32, asset_server: &Res<AssetServer>) {
    let show_right = (i % 3) == 0;
    let show_top = i > 0 && i < 10 || i > 27 && i < 37 || i > 54 && i < 64;
    let show_bottom = i > 18 && i < 28 || i > 45 && i < 55 || i > 72 && i < 82;
    let show_left = ((i % 3) + 1) - 2 == 0;

    let border_width = 5.0;
    let border_mod = 1.0;

    let rect = UiRect {
        left: bevy::ui::Val::Px(if show_left { border_width } else { border_mod }),
        top: bevy::ui::Val::Px(if show_top { border_width } else { border_mod }),
        right: bevy::ui::Val::Px(if show_right { border_width } else { border_mod }),
        bottom: bevy::ui::Val::Px(if (show_bottom) && i != 1 {
            border_width
        } else {
            border_mod
        }),
    };

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(bevy::ui::Val::Px(100.0), bevy::ui::Val::Px(100.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: bevy::ui::Val::Px(x as f32),
                        top: bevy::ui::Val::Px(y as f32),
                        ..default()
                    },
                    border: rect,
                    ..default()
                },
                background_color: Color::rgb(0.4, 0.4, 1.0).into(),
                ..default()
            },
            Name::new(i.to_string()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(
                                bevy::ui::Val::Percent(100.0),
                                bevy::ui::Val::Percent(100.0),
                            ),
                            margin: UiRect::all(bevy::ui::Val::Auto),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceAround,
                            ..default()
                        },
                        background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                        ..default()
                    },
                    Name::new(i.to_string()),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        i.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
