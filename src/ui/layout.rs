use crate::{get_unix_timestamp, shorten_string, spawn_copyable_text, AppWallet};
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};
use chrono::NaiveDateTime;
use solana_sdk::signature::Signer;

use super::{components::*, styles::*};


pub struct UiListItem {
    pub id: String,
    pub sig: String,
    pub tx_time: String,
    pub hash_time: String,
    pub status: String,
}

pub fn spawn_new_list_item(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scroll_panel_entity: Entity,
    item_data: UiListItem,
) {
    let sig = shorten_string(item_data.sig.clone(), 10);
    let new_result_item = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
            Name::new("TxResult Item"),
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    item_data.id,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        ..default()
                    },
                ),
                Label,
            ));
            spawn_copyable_text(parent, asset_server, item_data.sig.clone(), sig);

            // parent.spawn((TextBundle::from_section(
            //     sig,
            //     TextStyle {
            //         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //         font_size: 20.,
            //         ..default()
            //     },
            // ),));

            parent.spawn((TextBundle::from_section(
                item_data.tx_time,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.,
                    ..default()
                },
            ),));

            parent.spawn(TextBundle::from_section(
                item_data.hash_time,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.,
                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section(
                item_data.status,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.,
                    ..default()
                },
            ));
        })
        .id();

    commands
        .entity(scroll_panel_entity)
        .add_child(new_result_item);
}
