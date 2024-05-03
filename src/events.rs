use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::{
    ui::{components::MovingScrollPanel, layout::spawn_new_list_item}, EntityTaskHandler, TaskConfirmTx, TaskGenerateHash, TaskSendAndConfirmTx, TaskSendTx
};

// Events
#[derive(Event)]
pub struct EventStartStopMining;

#[derive(Event)]
pub struct EventSubmitHashTx(pub String);

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
    mut ev_start_stop_mining: EventReader<EventStartStopMining>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in ev_start_stop_mining.read() {
        info!("Start/Stop Mining Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();
        let task = pool.spawn(async move {
            "NEWGENERATEDHASH".to_string()
        });

        commands
            .entity(task_handler_entity)
            .insert(TaskGenerateHash { task });

    }
}

pub fn handle_event_submit_hash_tx(
    mut commands: Commands,
    mut ev_submit_hash_tx: EventReader<EventSubmitHashTx>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in ev_submit_hash_tx.read() {
        info!("Submit Hash Tx Event Handler.");
        // TODO: sign the tx
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();

        // TODO: spawn the tx sender task
        let task = pool.spawn(async move {
            // start a timer
            // sign the transaction
            // send the transaction
            let mut i = 0;
            // loop
            loop {
                // based on timer, resend signed tx
                // based on timer, check tx status
                // if blockhash expired, return with FAILED - Blockhash Expired
                i += 1;
                if i > 100 {
                    return ("SIGNATURE".to_string(), "SUCCESS".to_string());
                }
            }
        });

        commands
            .entity(task_handler_entity)
            .insert( TaskSendAndConfirmTx { task });
    }
}

pub fn handle_event_tx_result(
    mut commands: Commands,
    mut ev_tx_result: EventReader<EventTxResult>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MovingScrollPanel>>,
) {
    for _ev in ev_tx_result.read() {
        info!("Tx Result Event Handler.");
        let scroll_panel_entity = query.get_single().unwrap();
        spawn_new_list_item(&mut commands, &asset_server, scroll_panel_entity);
    }
}