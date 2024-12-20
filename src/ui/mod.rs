use bevy::prelude::*;

use crate::{
    rsc::game_state::{GameState, Markers, Tools},
    sys::{
        grid_update_system::{GridCell, GridLabel},
        text::{ColorText, FpsText},
    },
};
#[derive(Component, Clone, Debug, Default)]
pub struct Location(pub u8);
#[derive(Component)]
pub struct ToolButton(pub Tools);

#[derive(Component)]
pub struct GridMark(pub Markers);

#[derive(Component, Copy, Clone, Debug)]
pub struct ToolLabel {
    pub tool: Tools,
}
#[derive(Component)]
pub struct GridButton;

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

pub fn tool_panel(mut cmd: Commands, game_state: Res<GameState>, asset_server: Res<AssetServer>) {
    cmd.spawn((
        NodeBundle {
            style: Style {
                size: Size::new(bevy::ui::Val::Px(300.0), bevy::ui::Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: bevy::ui::Val::Px(0.0),
                    top: bevy::ui::Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor(game_state.theme.tool.panel_bg),
            z_index: ZIndex::Global(1),
            ..default()
        },
        Name::new("Tool Panel"),
    ))
    .with_children(|parent| {
        let tools = [
            (Tools::Fill, "Fill"),
            (Tools::CornerMark, "Corner"),
            (Tools::CenterMark, "Center"),
            (Tools::Erase, "Erase"),
        ];
        for (tool, name) in tools {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(bevy::ui::Val::Px(100.0), bevy::ui::Val::Px(100.0)),
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: bevy::ui::Val::Px(0.0),
                                top: bevy::ui::Val::Px(110.0 * (tool as i32) as f32),
                                ..default()
                            },
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(game_state.theme.tool.bg),
                        ..default()
                    },
                    ToolButton(tool),
                    Name::new("Tool: ".to_string() + name),
                ))
                .with_children(|button| {
                    button.spawn((
                        TextBundle::from_section(
                            name,
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: game_state.theme.tool.text,
                            },
                        ),
                        ToolLabel { tool: tool },
                    ));
                });
        }
    });
}

pub fn board(mut cmd: Commands, game_state: Res<GameState>, asset_server: Res<AssetServer>) {
    cmd.spawn((
        NodeBundle {
            style: Style {
                size: Size::new(bevy::ui::Val::Percent(100.0), bevy::ui::Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                border: UiRect::all(Val::Px(0.0)),
                ..default()
            },
            background_color: BackgroundColor(game_state.theme.window_bg),
            ..default()
        },
        Name::new("Grid"),
    ))
    .with_children(|parent| {
        let mut i = 1;
        let l = 100; // Left Margin
        let t = 10; // Top Margin
        let w = 100; // Width
        let h = 100; // Height
        let y = 9; // Row Length
        while i <= 81 {
            spawn_cell(
                parent,
                i,
                l + (w * (1 + ((i - 1) % y))),
                t + (h * (0 + ceil(i, y))),
                &game_state,
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
    game_state: &Res<GameState>,
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
                min_size: Size::new(bevy::ui::Val::Px(100.0), bevy::ui::Val::Px(100.0)),
                margin: UiRect::all(Val::Px(0.0)),
                padding: UiRect::all(Val::Px(0.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: bevy::ui::Val::Px(x as f32),
                    top: bevy::ui::Val::Px(y as f32),
                    ..default()
                },
                border: rect,
                ..default()
            },
            background_color: game_state.theme.grid.border.into(),
            ..default()
        },
        GridCell {
            index: i as u8,
            x: (i % 9) as u8,
            y: (i / 9) as u8,
            value: 0,
            ..Default::default()
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
                    background_color: game_state.theme.grid.bg.into(),
                    z_index: ZIndex::Global(1),
                    ..default()
                },
                GridButton,
                Name::new(i.to_string()),
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        i.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: game_state.theme.grid.text,
                        },
                    ),
                    GridLabel,
                ));
                let corners = [
                    (
                        "TL",
                        GridMark(Markers::TL),
                        UiRect {
                            top: bevy::ui::Val::Px(0.0),
                            left: bevy::ui::Val::Px(0.0),
                            ..default()
                        },
                    ),
                    (
                        "TR",
                        GridMark(Markers::TR),
                        UiRect {
                            top: bevy::ui::Val::Px(0.0),
                            right: bevy::ui::Val::Px(0.0),
                            ..default()
                        },
                    ),
                    (
                        "BL",
                        GridMark(Markers::BL),
                        UiRect {
                            bottom: bevy::ui::Val::Px(0.0),
                            left: bevy::ui::Val::Px(0.0),
                            ..default()
                        },
                    ),
                    (
                        "BR",
                        GridMark(Markers::BR),
                        UiRect {
                            bottom: bevy::ui::Val::Px(0.0),
                            right: bevy::ui::Val::Px(0.0),
                            ..default()
                        },
                    ),
                ];
                for corner in corners {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::new(
                                        bevy::ui::Val::Px(33.0),
                                        bevy::ui::Val::Px(33.0),
                                    ),
                                    margin: UiRect::all(Val::Px(0.0)),
                                    padding: UiRect::all(Val::Px(0.0)),
                                    position_type: PositionType::Absolute,
                                    position: corner.2,
                                    align_items: AlignItems::FlexEnd,
                                    justify_content: JustifyContent::SpaceAround,
                                    ..default()
                                },
                                background_color: game_state.theme.grid.hover.into(),
                                ..default()
                            },
                            corner.1,
                            Location(i as u8),
                            Name::new(corner.0),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: game_state.theme.grid.text,
                                    },
                                ),
                                Location(i as u8),
                                GridLabel,
                                Name::new(corner.0),
                            ));
                        });
                }
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(bevy::ui::Val::Px(93.0), bevy::ui::Val::Px(33.3)),
                                margin: UiRect::all(Val::Px(0.0)),
                                padding: UiRect::all(Val::Px(0.0)),
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    left: bevy::ui::Val::Px(0.0),
                                    bottom: bevy::ui::Val::Px(30.0),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceAround,
                                ..default()
                            },
                            background_color: game_state.theme.grid.invalid.into(),
                            ..default()
                        },
                        GridMark(Markers::Center),
                        Location(i as u8),
                        Name::new("Center Mark"),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: game_state.theme.grid.text,
                                },
                            ),
                            Location(i as u8),
                            GridLabel,
                            Name::new("Center"),
                        ));
                    });
            });
    });
    return cmds.id();
}
