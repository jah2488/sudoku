use bevy::prelude::*;

use crate::sys::{
    grid_update_system::{GridCell, GridLabel},
    text::{ColorText, FpsText},
};

pub fn debug_panel(asset_server: &Res<AssetServer>) -> (TextBundle, ColorText, Name) {
    (
        TextBundle::from_section(
            "debug",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_LEFT)
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
    )
}

pub fn fps(asset_server: &Res<AssetServer>) -> (TextBundle, FpsText, Name) {
    (
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
    )
}

pub fn board(mut commands: Commands, asset_server: &Res<AssetServer>) {
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
            let l = 200; // Left Margin
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

fn spawn_cell(
    parent: &mut ChildBuilder,
    i: i32,
    x: i32,
    y: i32,
    asset_server: &Res<AssetServer>,
) -> Entity {
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

    let mut cmds = parent.spawn((
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
        GridCell {
            index: i as u8,
            x: (i % 9) as u8,
            y: (i / 9) as u8,
            value: 0,
            selected: false,
            hovered: false,
            mutable: false,
            invalid: false,
        },
        Name::new(i.to_string()),
    ));
    cmds.with_children(|parent| {
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
                parent.spawn((
                    TextBundle::from_section(
                        i.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ),
                    GridLabel,
                ));
            });
    });
    return cmds.id();
}
