use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;

use std::thread;
use std::time::{Duration, Instant};

use crate::Val::*;

#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug, PartialOrd, Ord)]
pub enum Val {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Unknown,
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", from_val(*self))
    }
}

pub fn options() -> HashSet<Val> {
    HashSet::from([One, Two, Three, Four, Five, Six, Seven, Eight, Nine])
}

pub fn to_val(n: u8) -> Val {
    match n {
        1 => One,
        2 => Two,
        3 => Three,
        4 => Four,
        5 => Five,
        6 => Six,
        7 => Seven,
        8 => Eight,
        9 => Nine,
        _ => Unknown,
    }
}

pub fn from_val(v: Val) -> u8 {
    match v {
        One => 1,
        Two => 2,
        Three => 3,
        Four => 4,
        Five => 5,
        Six => 6,
        Seven => 7,
        Eight => 8,
        Nine => 9,
        Unknown => 0,
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Point {
        return Point { x, y };
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Cell {
    x: u8,
    y: u8,
    value: u8,
    neighbors: HashSet<Point>,
}

#[derive(Clone, PartialEq, Eq)]
struct CellView {
    tl: Val,
    tr: Val,
    bl: Val,
    br: Val,
    guesses: HashSet<Val>,
    value: Val,
}

impl CellView {
    pub fn new() -> CellView {
        CellView {
            tl: Unknown,
            tr: Unknown,
            bl: Unknown,
            br: Unknown,
            guesses: HashSet::new(),
            value: Unknown,
        }
    }
}

impl Cell {
    fn is_valid(&self, graph: &Graph) -> bool {
        let mut valid = true;
        for point in &self.neighbors {
            let cell = graph.at(point.x, point.y);
            match cell {
                Some(c) => {
                    if c.value == self.value {
                        valid = false;
                        break;
                    }
                }

                None => {
                    eprintln!("No cell found for {:?}", point);
                }
            }
        }
        return valid;
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Cell {{ x: {}, y: {}, value: {}, neighbors: {:?} }}",
            self.x, self.y, self.value, self.neighbors
        )
    }
}

#[derive(Clone)]
struct Graph {
    cells: Vec<Cell>,
}

impl Graph {
    fn new() -> Graph {
        let mut g = Graph { cells: Vec::new() };
        (1..=9).for_each(|x| {
            (1..=9).for_each(|y| {
                let c = Cell {
                    x,
                    y,
                    value: 0,
                    neighbors: HashSet::new(),
                };
                g.cells.push(c);
            });
        });

        let g_copy = g.clone();
        g.cells.iter_mut().for_each(|cell: &mut Cell| {
            cell.neighbors = g_copy.neighbors_for(cell);
        });
        return g;
    }

    fn at(&self, x: u8, y: u8) -> Option<&Cell> {
        let cell = self.cells.iter().find(|c| c.x == x && c.y == y);
        return cell;
    }

    fn index(&mut self, index: u8) -> Option<&mut Cell> {
        let x = (index - 1) % 9 + 1;
        let y = (index - 1) / 9 + 1;

        let cell = self.cells.iter_mut().find(|c| c.x == x && c.y == y);
        return cell;
    }

    fn neighbors_for(&self, cell: &Cell) -> HashSet<Point> {
        let mut neighbors: HashSet<Point> = HashSet::new();
        let x = cell.x;
        let y = cell.y;

        // Row
        (1..=9).for_each(|i| {
            if i != y {
                neighbors.insert(Point { x, y: i });
            }
            if i != x {
                neighbors.insert(Point { x: i, y });
            }
        });

        // y is the row (vertical)
        // x is the column (horizontal)
        match (x, y) {
            (1..=3, 1..=3) => {
                neighbors.extend(Graph::box_set(1, 3, 1, 3).into_iter().map(|p| p.clone()))
            }

            (3..=6, 1..=3) => {
                neighbors.extend(Graph::box_set(4, 6, 1, 3).into_iter().map(|p| p.clone()))
            }

            (6..=9, 1..=3) => {
                neighbors.extend(Graph::box_set(7, 9, 1, 3).into_iter().map(|p| p.clone()))
            }

            (1..=3, 3..=6) => {
                neighbors.extend(Graph::box_set(1, 3, 4, 6).into_iter().map(|p| p.clone()))
            }

            (3..=6, 3..=6) => {
                neighbors.extend(Graph::box_set(4, 6, 4, 6).into_iter().map(|p| p.clone()))
            }

            (6..=9, 3..=6) => {
                neighbors.extend(Graph::box_set(7, 9, 4, 6).into_iter().map(|p| p.clone()))
            }

            (1..=3, 6..=9) => {
                neighbors.extend(Graph::box_set(1, 3, 7, 9).into_iter().map(|p| p.clone()))
            }

            (3..=6, 6..=9) => {
                neighbors.extend(Graph::box_set(4, 6, 7, 9).into_iter().map(|p| p.clone()))
            }

            (6..=9, 6..=9) => {
                neighbors.extend(Graph::box_set(7, 9, 7, 9).into_iter().map(|p| p.clone()))
            }
            _ => {
                eprintln!("Incorrect X,Y range supplied!");
            }
        }
        return neighbors;
    }

    fn box_set(min_x: u8, max_x: u8, min_y: u8, max_y: u8) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();
        (min_x..=max_x).for_each(|x| {
            (min_y..=max_y).for_each(|y| {
                points.insert(Point::new(x, y));
            });
        });
        return points;
    }

    pub fn generate(&mut self) {
        let max_depth = 1_000_000;
        let mut depth = 0;
        let mut rng = rand::thread_rng();
        let mut graph: Graph;
        let mut i = 0;
        let mut invalid_moves: HashMap<usize, Vec<Val>> = HashMap::new();

        while i < 81 {
            graph = self.clone();
            let vals: HashSet<Val> = HashSet::new();

            // At the start of each loop, we need to reset the values of all cells after the current one
            let mut xi = i;
            while xi < 81 {
                let mut cell = self.cells.get_mut(xi).unwrap();
                cell.value = 0;
                xi += 1;
                match invalid_moves.get(&xi) {
                    Some(_) => {
                        invalid_moves.insert(xi, vec![]);
                    }
                    None => {}
                }
            }

            // Grab all possible valid moves for the current cell, and shuffle them, then pick the first one
            let mut cell = self.cells.get_mut(i).unwrap();
            let set = graph.possible_values(cell, vals);
            let mut v: Vec<Val> = Vec::new();
            for n in set {
                match invalid_moves.get(&(i)) {
                    Some(moves) => {
                        if !moves.contains(&n) {
                            v.push(n);
                        }
                    }
                    None => {
                        v.push(n);
                        invalid_moves.insert(i, vec![]);
                    }
                }
            }
            v.shuffle(&mut rng);
            let choice = v.get(0).cloned();

            match choice {
                Some(num) => {
                    cell.value = from_val(num);
                }
                None => {
                    let last_cell = self.cells.get(i - 1).unwrap();
                    // If we have no valid moves, we need to backtrack, and
                    // add the current cell's value to the list of invalid moves for the previous cell
                    match invalid_moves.get(&(i - 1)) {
                        Some(moves) => {
                            let mut moves = moves.clone();
                            if i > 1 {
                                match to_val(last_cell.value) {
                                    Unknown => {}
                                    n => {
                                        moves.push(n);
                                    }
                                }
                                println!("i: {}, val: {}, moves: {:?}", i, last_cell.value, moves);
                                invalid_moves.insert(i - 1, moves);
                            }
                        }
                        None => match to_val(last_cell.value) {
                            Unknown => {}
                            n => {
                                invalid_moves.insert(i - 1, vec![n]);
                            }
                        },
                    }

                    i -= 1;

                    depth += 1;

                    if depth >= max_depth {
                        break;
                    } else {
                        continue;
                    }
                }
            }

            if cell.is_valid(&graph) {
                i += 1;
                //let ten_millis = Duration::from_millis(20);
                //thread::sleep(ten_millis);
                //print!("{}[2J", 27 as char);
                //println!("{}=>{:?}\n", i, invalid_moves.get(&(i)));
                //println!("{:?}", self);
                //invalid_moves.clone().into_iter().for_each(|(k, v)| {
                //    println!("{}: {:?}", k, v);
                //});
            } else {
                cell.value = 0;
                i = 1;
            }
        }
    }

    fn possible_values(&mut self, cell: &Cell, mut vals: HashSet<Val>) -> HashSet<Val> {
        for n in &cell.neighbors {
            vals.insert(to_val(self.at(n.x, n.y).unwrap().value));
        }
        return options().difference(&vals).cloned().collect();
    }
}

//□  ■  ▲  ▼  ◆  ◇  ●  ○  ★  ☆
fn double_row_top() -> String {
    return String::from("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗");
}

fn double_row_bottom() -> String {
    return String::from("╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝");
}

fn double_row_middle() -> String {
    return String::from("╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣");
}

fn single_row_middle() -> String {
    return String::from("╟───┼───┼───╫───┼───┼───╫───┼───┼───╢");
}

impl Debug for Graph {
    //    "\033[1m#{self}\033[0m" // bold
    //    "\033[#{direction == :up ? 1 : 2}J" // clear screen

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut s = String::new();
        s.push_str(&format!("{}", double_row_top()));
        for i in 0..9 {
            if i == 3 || i == 6 {
                s.push_str(&format!("\n{}\n║", double_row_middle()));
            } else {
                if i > 0 {
                    s.push_str(&format!("\n{}\n║", single_row_middle()));
                } else {
                    s.push_str(&format!("\n║"));
                }
            }
            for j in 0..9 {
                let c = self.cells.iter().find(|c| c.x - 1 == i && c.y - 1 == j);
                if let Some(c) = c {
                    if j == 2 || j == 5 || j == 8 {
                        s.push_str(&format!(" {} ║", c.value));
                    } else {
                        s.push_str(&format!(" {} │", c.value));
                    }
                }
            }
            s.push_str(&format!(""));
        }
        s.push_str(&format!("\n{}\n", double_row_bottom()));
        write!(f, "{}", s)
    }
}

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
            current_cell: Val::Unknown,
            last_cell: Val::Unknown,
            graph: g,
            grid_filled: false,
            selected_cells: Vec::new(),
        })
        .add_startup_system(setup)
        .add_system(grid_fill_system)
        .add_system(text_update_system)
        .add_system(text_color_system)
        .add_system(button_system)
        .run();
}

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, PresentMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

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
#[derive(Resource)]
struct GameState {
    current_cell: Val,
    last_cell: Val,
    graph: Graph,
    grid_filled: bool,
    selected_cells: Vec<i32>,
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

fn cell_node(x: f32, y: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(bevy::ui::Val::Px(100.0), bevy::ui::Val::Px(100.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: bevy::ui::Val::Px(x),
                top: bevy::ui::Val::Px(y),
                ..default()
            },
            border: UiRect::all(bevy::ui::Val::Px(5.0)),
            ..default()
        },
        background_color: Color::rgb(0.4, 0.4, 1.0).into(),
        ..default()
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn grid_fill_system(mut game_state: ResMut<GameState>, mut query: Query<&mut Text>) {
    if game_state.grid_filled == true {
        return;
    }
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
    game_state.grid_filled = true;
}

fn button_system(
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
                let text = text_query
                    .get_mut(children[0])
                    .unwrap()
                    .into_inner()
                    .sections[0]
                    .value
                    .clone();

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

fn text_color_system(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
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

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
