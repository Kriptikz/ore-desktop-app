use bevy::{a11y::{accesskit::{NodeBuilder, Role}, AccessibilityNode}, prelude::*};

use crate::ui::components::MovingScrollPanel;

// Events
#[derive(Event)]
pub struct EventStartStopMining;

#[derive(Event)]
pub struct EventSubmitHashTransaction;

pub struct TxResult {
    pub sig: String,
    pub tx_time: u64,
    pub hash_time: u64,
    // TODO: create a TxStatus struct that will be able to show different colors based on status enums
    pub status: String,
}

#[derive(Event)]
pub struct EventTxResult;

pub fn handle_event_start_stop_mining_clicked(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_start_stop_mining: EventReader<EventStartStopMining>,
    query: Query<Entity, With<MovingScrollPanel>>,
) {
    for _ev in ev_start_stop_mining.read() {
        info!("Start/Stop Mining Event Handler.");
        let scroll_panel_entity = query.get_single().unwrap();
        // TODO: use a function here for spawning.
        // Moving all of this layout code to layouts.rs and styles.rs
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
                        format!("NEW."),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.,
                            ..default()
                        },
                    ),
                    Label,
                ));

                parent.spawn((TextBundle::from_section(
                    format!("TxnS...s8cs   COPY"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        ..default()
                    },
                ),));

                parent.spawn((TextBundle::from_section(
                    format!("23s"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        ..default()
                    },
                ),));

                parent.spawn(TextBundle::from_section(
                    format!("40s"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        ..default()
                    },
                ));

                parent.spawn(TextBundle::from_section(
                    format!("SUCCESS"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        ..default()
                    },
                ));
            }).id();

        commands.entity(scroll_panel_entity).add_child(new_result_item);
    }
}
