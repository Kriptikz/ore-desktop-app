use bevy::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};

use crate::{
    Config, EventClaimOreRewards, EventLock, EventResetEpoch, EventSaveConfig, EventStartStopMining, EventUnlock, OreAppState
};

use super::{
    components::{
        ButtonCaptureTextInput, ButtonClaimOreRewards, ButtonCopyText, ButtonLock, ButtonResetEpoch, ButtonSaveConfig, ButtonStartStopMining, ButtonUnlock, CopyableText, TextConfigInputRpcFetchAccountsInterval, TextConfigInputRpcSendTxInterval, TextConfigInputRpcUrl, TextConfigInputThreads, TextCursor, TextInput
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
                    let mut ctx = ClipboardContext::new().unwrap();
                    if let Err(_) = ctx.set_contents(text) {
                        info!("Failed to set clipboard content.");
                    } else {
                        info!("Succesfully copied to clipboard");
                    }
                } else {
                    info!("Failed to find copyable_text.");
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
        (Changed<Interaction>, With<ButtonStartStopMining>),
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

pub fn button_reset_epoch(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonResetEpoch>),
    >,
    mut event_writer: EventWriter<EventResetEpoch>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                event_writer.send(EventResetEpoch);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
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
        (Entity, &Interaction, &mut UiImage, &Children),
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
    for (_entity, interaction, mut ui_image, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("pressed");
                if !ui_image.flip_y {
                    ui_image.flip_y = true;
                }

                let text_rpc_url = set.p0().get_single().unwrap().text.clone();
                let text_threads = set.p1().get_single().unwrap().text.clone();
                let text_rpc_fetch_interval = set.p2().get_single().unwrap().text.clone();
                let text_rpc_send_interval = set.p3().get_single().unwrap().text.clone();

                event_writer.send(EventSaveConfig(Config {
                    rpc_url: text_rpc_url.clone(),
                    threads: text_threads.parse::<u64>().unwrap(),
                    fetch_ui_data_from_rpc_interval_ms: text_rpc_fetch_interval.parse::<u64>().unwrap(),
                    tx_check_status_and_resend_interval_ms: text_rpc_send_interval.parse::<u64>().unwrap(),
                }));
            }
            Interaction::Hovered => {
                info!("hovered");
                if ui_image.flip_y {
                    ui_image.flip_y = false;
                }
            }
            Interaction::None => {
                info!("none");
                if ui_image.flip_y {
                    ui_image.flip_y = false;
                }
            }
        }
    }
}
