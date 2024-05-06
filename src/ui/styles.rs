use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub static BUTTON: &str = "button.png";

// Backgrounds
pub static SCREEN_BACKGROUND_1: &str = "screen_backgrounds/background_1.png";
pub static SCREEN_BACKGROUND_2: &str = "screen_backgrounds/background_2.png";

pub static MENU_BACKGROUND: &str = "menu_background.png";
pub static TITLE_BACKGROUND: &str = "title_background.png";

pub static SETTINGS_ICON: &str = "icons/settings-icon.png";

// Fonts
pub const FONT_SIZE: f32 = 20.0;
pub const FONT_SIZE_TITLE: f32 = 28.0;
pub static FONT_REGULAR: &str = "fonts/Xirod.otf";
