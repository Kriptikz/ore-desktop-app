use bevy::{
    prelude::*, tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task}
};

use crate::{AppWallet, EventSubmitHashTx, EventTxResult, ProofAccountResource, TreasuryAccountResource};

// Task Components
// TODO: tasks should return results so errors can be dealt with by the task handler system
pub struct TaskUpdateAppWalletSolBalanceData {
    pub sol_balance: f64,
    pub ore_balance: f64,
    pub proof_account_data: ProofAccountResource,
    pub treasury_account_data: TreasuryAccountResource,
}
#[derive(Component)]
pub struct TaskUpdateAppWalletSolBalance {
    pub task: Task<TaskUpdateAppWalletSolBalanceData>,
}

#[derive(Component)]
pub struct TaskGenerateHash {
    pub task: Task<String>,
}

#[derive(Component)]
pub struct TaskSendAndConfirmTx {
    pub task: Task<(String, String)>,
}

#[derive(Component)]
pub struct TaskSendTx {
    pub task: Task<String>,
}

#[derive(Component)]
pub struct TaskConfirmTx {
    pub task: Task<String>,
}

pub fn task_update_app_wallet_sol_balance(
    mut commands: Commands,
    mut app_wallet: ResMut<AppWallet>,
    mut proof_account_res: ResMut<ProofAccountResource>,
    mut treasury_account_res: ResMut<TreasuryAccountResource>,
    mut query: Query<(Entity, &mut TaskUpdateAppWalletSolBalance)>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            app_wallet.sol_balance = result.sol_balance;
            app_wallet.ore_balance = result.ore_balance;
            *proof_account_res = result.proof_account_data;
            *treasury_account_res = result.treasury_account_data;
            commands
                .entity(entity)
                .remove::<TaskUpdateAppWalletSolBalance>();
        }
    }
}

pub fn task_generate_hash(
    mut commands: Commands,
    mut ev_submit_hash_tx: EventWriter<EventSubmitHashTx>,
    mut query: Query<(Entity, &mut TaskGenerateHash)>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            // Build unsigned tx
            ev_submit_hash_tx.send(EventSubmitHashTx(result));
            commands
                .entity(entity)
                .remove::<TaskGenerateHash>();
        }
    }
}

pub fn task_send_and_confirm_tx(
    mut commands: Commands,
    mut ev_tx_result: EventWriter<EventTxResult>,
    mut query: Query<(Entity, &mut TaskSendAndConfirmTx)>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some((_tx_sig, _tx_status)) = block_on(future::poll_once(&mut task.task)) {
            ev_tx_result.send(EventTxResult);

            commands
                .entity(entity)
                .remove::<TaskSendAndConfirmTx>();
        }
    }
}