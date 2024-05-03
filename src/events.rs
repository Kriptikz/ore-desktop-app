use bevy::prelude::*;

use crate::ui::{components::MovingScrollPanel, layout::spawn_new_list_item};

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
        spawn_new_list_item(&mut commands, &asset_server, scroll_panel_entity);
    }
}
