use bevy::prelude::*;

use crate::{AppScreenState, AppWallet, NavItemScreen};

use super::{components::NavItem, styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON}};

pub fn nav_item_interactions(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor, &NavItem),
        Changed<Interaction>,
    >,
    app_wallet: Res<AppWallet>,
    mut next_state: ResMut<NextState<AppScreenState>>,
) {
    for (_entity, interaction, mut color, mut border_color, nav_item) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                // event_writer.send(EventNavItemClicked);
                if let Some(_) = &app_wallet.wallet {
                    match nav_item.0 {
                        NavItemScreen::Dashboard => {
                            next_state.set(AppScreenState::Dashboard);
                        },
                        NavItemScreen::Mining => {
                            next_state.set(AppScreenState::Mining);
                        },
                        NavItemScreen::SettingsWallet => {
                            next_state.set(AppScreenState::SettingsWallet);
                        },
                        NavItemScreen::SettingsConfig => {
                            next_state.set(AppScreenState::SettingsConfig);
                        },
                        NavItemScreen::SettingsGeneral => {
                            next_state.set(AppScreenState::SettingsGeneral);
                        },
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::rgba(1.0, 1.0, 1.0, 0.05).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::rgba(0.0, 0.0, 0.0, 0.0).into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
