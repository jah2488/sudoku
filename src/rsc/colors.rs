use bevy::prelude::*;

#[derive(Reflect, Clone, Debug, Default)]
pub struct Theme {
    pub grid: GridTheme,
    pub tool: ToolTheme,
    pub window_bg: Color,
}

#[derive(Reflect, Clone, Debug, Default)]
pub struct GridTheme {
    pub bg: Color,
    pub border: Color,
    pub cursor: Color,
    pub focused: Color,
    pub hover: Color,
    pub invalid: Color,
    pub selected_hover: Color,
    pub selected: Color,
    pub starter_text: Color,
    pub text: Color,
}

#[derive(Reflect, Clone, Debug, Default)]
pub struct ToolTheme {
    pub bg: Color,
    pub hover: Color,
    pub panel_bg: Color,
    pub selected_hover: Color,
    pub selected: Color,
    pub text: Color,
}

impl Theme {
    pub fn default_theme() -> Theme {
        Theme {
            window_bg: Color::rgb(0.1, 0.1, 0.1),
            grid: GridTheme {
                bg: Color::rgb(0.13, 0.13, 0.15),
                border: Color::rgb(0.4, 0.3, 1.0),
                cursor: Color::rgb(0.35, 0.15, 0.75),
                focused: Color::rgba(0.75, 0.15, 0.15, 0.50),
                hover: Color::rgb(0.25, 0.25, 0.25),
                invalid: Color::rgba(0.85, 0.15, 0.15, 0.80),
                selected_hover: Color::rgb(0.35, 0.75, 0.35),
                selected: Color::rgb(0.35, 0.75, 0.35),
                starter_text: Color::rgb(1.85, 0.05, 0.35),
                text: Color::rgb(0.9, 0.9, 0.9),
            },
            tool: ToolTheme {
                bg: Color::rgb(0.15, 0.15, 0.15),
                hover: Color::rgb(0.25, 0.25, 0.25),
                panel_bg: Color::rgb(0.14, 0.14, 0.18),
                selected_hover: Color::rgb(0.35, 0.75, 0.35),
                selected: Color::rgb(0.35, 0.75, 0.35),
                text: Color::rgb(0.9, 0.9, 0.9),
            },
        }
    }
}

/*

  Grid Cell's
  Node -> background_color: Color::rgb(0.4, 0.4, 1.0).into(),
    Button -> background_color: Color::rgb(0.8, 0.8, 1.0).into(),
        Text color: Color::rgb(0.9, 0.9, 0.9),

  Node -> background_color: BackgroundColor(Color::rgba(0.4, 0.3, 0.1, 0.5)),
    Button -> background_color: BackgroundColor(Color::rgba(0.4, 0.3, 0.4, 1.0)),
        Text -> color: Color::rgb(0.9, 0.9, 0.9),

  ToolPanel -> background_color: BackgroundColor(Color::rgba(0.4, 0.3, 0.1, 0.5)),
    Button -> background_color: BackgroundColor(Color::rgba(0.4, 0.3, 0.4, 1.0)),
        Text -> color: Color::rgb(0.9, 0.9, 0.9),

   Cell -> Default -> *color = Color::rgb(0.15, 0.15, 0.15).into();
    Cell -> Invalid -> *color = Color::rgba(0.85, 0.15, 0.15, 0.80).into();
        Cursor -> *color = Color::rgb(0.35, 0.15, 0.75).into();
        Hovered -> *color = Color::rgb(0.25, 0.25, 0.25).into();
        Selected -> *color = Color::rgb(0.35, 0.75, 0.35).into();
        Focus -> *color = Color::rgb(0.85, 0.15, 0.15).into();
        Mutable ->  Color::rgb(1.85, 0.05, 0.35);
*/
