use bevy::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};

use crate::{
    Config, EventClaimOreRewards, EventGenerateWallet, EventLock, EventSaveConfig, EventSaveWallet, EventStakeOre, EventStartStopMining, EventUnlock, OreAppState
};

use super::{
    components::{
        AutoScrollCheckIcon, ButtonAutoScroll, ButtonCaptureTextInput, ButtonClaimOreRewards, ButtonCopyText, ButtonGenerateWallet, ButtonLock, ButtonOpenWebTxExplorer, ButtonSaveConfig, ButtonSaveGeneratedWallet, ButtonStakeOre, ButtonUnlock, CopyableText, TextConfigInputRpcFetchAccountsInterval, TextConfigInputRpcSendTxInterval, TextConfigInputRpcUrl, TextConfigInputThreads, TextInput, ToggleAutoMine
    },
    styles::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
};

pub fn button_copy_text(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonCopyText>),
    >,
    text_query: Query<(&CopyableText, &Children)>,
) {
    for (entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();

                let mut text: Option<String> = None;
                for (copyable_text, children) in text_query.iter() {
                    for child in children.iter() {
                        if *child == entity {
                            text = Some(copyable_text.full_text.clone());
                        }
                    }
                }
                if let Some(text) = text {
                    let mut ctx = ClipboardContext::new();
                    if let Ok(mut ctx) = ctx {
                        if let Err(_) = ctx.set_contents(text) {
                            error!("Failed to set clipboard content.");
                        } 
                    } else {
                        error!("Failed to get clipboard context.");
                    }
                } else {
                    error!("Failed to find copyable_text.");
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}

pub fn button_open_web_tx_explorer(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonOpenWebTxExplorer>),
    >,
    text_query: Query<(&CopyableText, &Children)>,
) {
    for (entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();

                let mut text: Option<String> = None;
                for (copyable_text, children) in text_query.iter() {
                    for child in children.iter() {
                        if *child == entity {
                            text = Some(copyable_text.full_text.clone());
                        }
                    }
                }
                if let Some(text) = text {
                    let url = format!("https://solscan.io/tx/{}?cluster=devnet", text);
                    if let Err(_) = open::that(url) {
                        error!("Failed to open web tx explorer with default web browser.");
                    }
                } else {
                    error!("Failed to find copyable_text.");
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}

pub fn button_start_stop_mining(
    mut ev_start_stop_mining: EventWriter<EventStartStopMining>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ToggleAutoMine>),
    >,
) {
    for (_entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // border_color.0 = Color::RED;

                ev_start_stop_mining.send(EventStartStopMining);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                // border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::WHITE.into();
                // *color = NORMAL_BUTTON.into();
                // border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn button_generate_wallet(
    mut event_writer: EventWriter<EventGenerateWallet>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonGenerateWallet>),
    >,
) {
    for (_entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // border_color.0 = Color::RED;

                event_writer.send(EventGenerateWallet);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                // border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::WHITE.into();
                // *color = NORMAL_BUTTON.into();
                // border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn button_claim_ore_rewards(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonClaimOreRewards>),
    >,
    mut event_writer: EventWriter<EventClaimOreRewards>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // border_color.0 = Color::RED;

                event_writer.send(EventClaimOreRewards);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                // border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::WHITE.into();
                // *color = NORMAL_BUTTON.into();
                // border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn button_stake_ore(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonStakeOre>),
    >,
    mut event_writer: EventWriter<EventStakeOre>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // border_color.0 = Color::RED;

                event_writer.send(EventStakeOre);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                // border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::WHITE.into();
                // *color = NORMAL_BUTTON.into();
                // border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn button_lock(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonLock>),
    >,
    mut event_writer: EventWriter<EventLock>,
) {
    for (_entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                event_writer.send(EventLock);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn button_unlock(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonUnlock>),
    >,
    mut event_writer: EventWriter<EventUnlock>,
) {
    for (_entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                event_writer.send(EventUnlock);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn button_capture_text(
    interaction_query: Query<
        (
            Entity,
            &Interaction,
        ),
        (Changed<Interaction>, With<ButtonCaptureTextInput>),
    >,
    mut ore_app_state: ResMut<OreAppState>,
) {
    for (entity, interaction) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                ore_app_state.active_input_node = Some(entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn button_save_config(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiImage, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonSaveConfig>),
    >,
    mut event_writer: EventWriter<EventSaveConfig>,
    mut set: ParamSet<(
        Query<&TextInput, With<TextConfigInputRpcUrl>>,
        Query<&TextInput, With<TextConfigInputThreads>>,
        Query<&TextInput, With<TextConfigInputRpcFetchAccountsInterval>>,
        Query<&TextInput, With<TextConfigInputRpcSendTxInterval>>,
    )>,
) {
    for (_entity, interaction, mut ui_image, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // if !ui_image.flip_y {
                //     ui_image.flip_y = true;
                // }

                let text_rpc_url = if let Ok(single) = set.p0().get_single() {
                    single.text.clone()
                } else {
                    error!("Failed to get text_rpc_url.");
                    break;
                };
                let threads = if let Ok(single) = set.p1().get_single() {
                    let threads = single.text.clone().parse::<u64>();
                    if let Ok(threads) = threads {
                        threads
                    } else {
                        error!("Failed to parse text_threads.");
                        break;
                    }
                } else {
                    error!("Failed to get text_threads.");
                    break;
                };
                let text_rpc_fetch_interval = if let Ok(single) = set.p2().get_single() {
                    let parsed = single.text.clone().parse::<u64>();
                    if let Ok(parsed) = parsed {
                        parsed
                    } else {
                        error!("Failed to parse text_rpc_fetch_interval.");
                        break;
                    }
                } else {
                    error!("Failed to get text_rpc_fetch_interval.");
                    break;
                };
                let text_rpc_send_interval = if let Ok(single) = set.p3().get_single() {
                    let parsed = single.text.clone().parse::<u64>();
                    if let Ok(parsed) = parsed {
                        parsed
                    } else {
                        error!("Failed to parse text_rpc_send_interval.");
                        break;
                    }
                } else {
                    error!("Failed to get text_rpc_send_interval.");
                    break;
                };

                event_writer.send(EventSaveConfig(Config {
                    rpc_url: text_rpc_url.clone(),
                    threads,
                    fetch_ui_data_from_rpc_interval_ms: text_rpc_fetch_interval,
                    tx_check_status_and_resend_interval_ms: text_rpc_send_interval,
                }));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                // if ui_image.flip_y {
                //     ui_image.flip_y = false;
                // }
            }
            Interaction::None => {
                *color = Color::WHITE.into();
                // if ui_image.flip_y {
                //     ui_image.flip_y = false;
                // }
            }
        }
    }
}

pub fn button_save_wallet(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiImage, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonSaveGeneratedWallet>),
    >,
    mut event_writer: EventWriter<EventSaveWallet>,
) {
    for (_entity, interaction, mut ui_image, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // if !ui_image.flip_y {
                //     ui_image.flip_y = true;
                // }

                event_writer.send(EventSaveWallet);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                // if ui_image.flip_y {
                //     ui_image.flip_y = false;
                // }
            }
            Interaction::None => {
                *color = Color::WHITE.into();
                // if ui_image.flip_y {
                //     ui_image.flip_y = false;
                // }
            }
        }
    }
}

pub fn button_auto_scroll(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiImage, &mut BackgroundColor, &mut ButtonAutoScroll),
        Changed<Interaction>,
    >,
    mut query_check_icon: Query<&mut BackgroundColor, (With<AutoScrollCheckIcon>, Without<ButtonAutoScroll>)>,
    // mut event_writer: EventWriter<EventToggleAutoScroll>,
) {
    for (_entity, interaction, mut ui_image, mut color, mut button_auto_scroll) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // if !ui_image.flip_y {
                //     ui_image.flip_y = true;
                // }
                button_auto_scroll.0 = !button_auto_scroll.0;

                let mut check_icon = query_check_icon.single_mut();
                let checked = button_auto_scroll.0;


                if checked {
                    *check_icon = Color::WHITE.into();
                }

                if !checked {
                    *check_icon = Color::DARK_GRAY.into();
                }
                // event_writer.send(EventSaveWallet);
                
            }
            Interaction::Hovered => {
                *color = Color::WHITE.into();
                // if ui_image.flip_y {
                //     ui_image.flip_y = false;
                // }
            }
            Interaction::None => {
                *color = HOVERED_BUTTON.into();
                // if ui_image.flip_y {
                //     ui_image.flip_y = false;
                // }
            }
        }
    }
}