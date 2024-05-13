use bevy::prelude::*;

// Buttons
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub static BUTTON: &str = "button.png";
pub static BUTTON_SAVE_CONFIG: &str = "design_1/button_save_config.png";
pub static BUTTON_SAVE_WALLET: &str = "design_1/button_save_wallet.png";
pub static BUTTON_GENERATE: &str = "design_1/button_generate.png";
pub static BUTTON_CLAIM: &str = "design_1/button_claim.png";
pub static BUTTON_STAKE: &str = "design_1/button_stake.png";
pub static BUTTON_START_MINING: &str = "design_1/button_start_mining.png";
pub static BUTTON_STOP_MINING: &str = "design_1/button_stop_mining.png";
pub static BUTTON_RESET_EPOCH: &str = "design_1/button_reset_epoch.png";

pub static BUTTON_COPY_TEXT: &str = "design_1/copy_icon.png";

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

pub static SETTINGS_ICON: &str = "icons/settings-icon.png";

// Fonts
pub const FONT_SIZE: f32 = 16.0;
pub const FONT_SIZE_LARGE: f32 = 20.0;
pub const FONT_SIZE_TITLE: f32 = 22.0;
pub static FONT_REGULAR: &str = "fonts/Xirod.otf";
pub static FONT_ROBOTO: &str = "fonts/Roboto-Regular.ttf";
pub static FONT_ROBOTO_MEDIUM: &str = "fonts/Roboto-Medium.ttf";


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
