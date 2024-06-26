use std::time::{Duration, Instant};

use bevy::prelude::*;

use crate::{
    EntityTaskFetchUiData, EntityTaskHandler,
    MinerStatusResource, OreAppState, RpcConnection, TxStatus,
};

use crate::ui::
    components::{
        BaseScreenNode,
        InitialSetupScreenNode,
        LockedScreenNode, WalletSetupScreenNode,
    };


pub fn despawn_initial_setup_screen(
    mut commands: Commands,
    query: Query<Entity, With<InitialSetupScreenNode>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_wallet_setup_screen(
    mut commands: Commands,
    query: Query<Entity, With<WalletSetupScreenNode>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_locked_screen(mut commands: Commands, query: Query<Entity, With<LockedScreenNode>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_mining_screen(
    mut commands: Commands,
    app_state: Res<OreAppState>,
    mut miner_status: ResMut<MinerStatusResource>,
    query: Query<Entity, With<BaseScreenNode>>,
    query_task_miner_entity: Query<Entity, With<EntityTaskHandler>>,
    query_task_fetch_ui_data_entity: Query<Entity, With<EntityTaskFetchUiData>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let config = &app_state.config;

    miner_status.miner_status = "STOPPED".to_string();

    let entity_task_miner = query_task_miner_entity
        .get_single()
        .expect("Should only have a single task miner entity");
    let entity_task_fetch_ui_data = query_task_fetch_ui_data_entity
        .get_single()
        .expect("Should only have a single fetch ui data entity");

    commands.remove_resource::<RpcConnection>();

    commands.entity(entity_task_miner).despawn_recursive();
    commands
        .entity(entity_task_fetch_ui_data)
        .despawn_recursive();
}