use bevy::prelude::*;

// Buttons
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub static TOGGLE_ON: &str = "design_1/toggle_on.png";
pub static TOGGLE_OFF: &str = "design_1/toggle_off.png";

pub static BUTTON: &str = "button.png";
pub static BUTTON_GREEN_MEDIUM: &str = "design_1/button_green_medium.png";
pub static BUTTON_RED_MEDIUM: &str = "design_1/button_red_medium.png";
pub static BUTTON_SAVE_CONFIG: &str = "design_1/button_save_config.png";
pub static BUTTON_SAVE_WALLET: &str = "design_1/button_save_wallet.png";
pub static BUTTON_GENERATE: &str = "design_1/button_generate.png";
pub static BUTTON_CLAIM: &str = "design_1/button_claim.png";
pub static BUTTON_STAKE: &str = "design_1/button_stake.png";
pub static BUTTON_START_MINING: &str = "design_1/button_start_mining.png";
pub static BUTTON_STOP_MINING: &str = "design_1/button_stop_mining.png";
pub static BUTTON_RESET_EPOCH: &str = "design_1/button_reset_epoch.png";

// Icons
pub static CHECKBOX: &str = "design_1/checkbox.png";
pub static CHECK_ICON: &str = "design_1/check_icon_medium.png";
pub static SPINNER_ICON: &str = "design_1/spinner_icon.png";

pub static BUTTON_COPY_TEXT: &str = "design_1/copy_icon.png";
pub static SOLANA_ICON: &str = "solana-icon.png";

pub static FIRE_ICON: &str = "design_1/fire_icon.png";

pub static SETTINGS_ICON: &str = "icons/settings-icon.png";

// Backgrounds
pub static SCREEN_BACKGROUND_1: &str = "screen_backgrounds/background_1.png";
pub static SCREEN_BACKGROUND_2: &str = "screen_backgrounds/background_2.png";

pub static MENU_BACKGROUND: &str = "menu_background.png";
pub static TITLE_BACKGROUND: &str = "title_background.png";

pub static TREASURY_BACKGROUND: &str = "design_1/treasury_background.png";
pub static TX_RESULTS_BACKGROUND: &str = "design_1/tx_status_list_background.png";
pub static PROOF_ACCOUNT_BACKGROUND: &str = "design_1/proof_account_background.png";
pub static SYSTEM_OVERVIEW_BACKGROUND: &str = "design_1/system_overview_background.png";
pub static CURRENT_TX_STATUS_BACKGROUND: &str = "design_1/current_tx_background.png";

pub static TX_POP_UP_BACKGROUND: &str = "design_1/tx_pop_up_background.png";

// HEX COLORS

pub fn hex_black() -> Color {
    Color::hex("#000000").unwrap()
}

pub fn hex_dark_gray() -> Color {
    Color::hex("#2C2C2C").unwrap()
}

pub fn hex_light_gray() -> Color {
    Color::hex("#404040").unwrap()
}

// Design 2

pub static ORE_LOGO_WHITE: &str = "design_2/ore_logo_white.png";

pub fn hex_dark_mode_background() -> Color {
    Color::hex("#323741").unwrap()
}

// Fonts
pub const FONT_SIZE_X_SMALL: f32 = 16.0;
pub const FONT_SIZE_SMALL: f32 = 18.0;
pub const FONT_SIZE_MEDIUM: f32 = 20.0;
pub const FONT_SIZE_LARGE: f32 = 28.0;

pub static FONT_REGULAR: &str = "fonts/OpenSans-Semibold.ttf";
pub static FONT_BOLD: &str = "fonts/OpenSans-Bold.ttf";

// Icons
pub static DASHBOARD_ICON_WHITE: &str = "design_2/dashboard_icon_white.png";
pub static MINING_ICON_GRAY: &str = "design_2/mining_icon_gray.png";
pub static NAV_ARROW_ICON: &str = "design_2/nav_arrow_icon.png";

pub fn hex_dark_mode_nav_title() -> Color {
    Color::hex("#2A2D35").unwrap()
}

pub fn hex_dark_mode_text_white() -> Color {
    Color::hex("#EFEFEF").unwrap()
}

pub fn hex_dark_mode_text_white_2() -> Color {
    Color::hex("#FAFAFA").unwrap()
}

pub fn hex_dark_mode_text_gray() -> Color {
    Color::hex("#FAFAFA").unwrap()
}