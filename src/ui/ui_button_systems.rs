use bevy::prelude::*;
use crate::*;
use super::{components::*, styles::*};

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
                border_color.0 = Color::RED;

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
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
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
                border_color.0 = Color::RED;
                
                ev_start_stop_mining.send(EventStartStopMining);
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
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
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
                border_color.0 = Color::RED;

                event_writer.send(EventClaimOreRewards);
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

pub fn button_lock(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonLock>),
    >,
     mut next_state: ResMut<NextState<GameState>>,
) {
    for (_entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                next_state.set(GameState::Locked);
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
     mut next_state: ResMut<NextState<GameState>>,
) {
    for (_entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                next_state.set(GameState::Mining);
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
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor, &Children),
        (Changed<Interaction>, With<ButtonCaptureTextInput>),
    >,
    mut ore_app_state: ResMut<OreAppState>,
    child_text_query: Query<Entity, With<TextInput>>
) {
    for (_entity, interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                for button_child in children.iter() {
                    for child_text in child_text_query.iter() {
                        if child_text == *button_child {
                            ore_app_state.active_input_node = Some(child_text);
                        }
                    }
                }

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
