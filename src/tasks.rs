use std::time::Instant;

use bevy::{
    prelude::*, tasks::{block_on, futures_lite::future, Task}
};
use solana_sdk::{signature::Signature, transaction::Transaction};

use crate::{AppWallet, CurrentTx, EventProcessTx, EventSubmitHashTx, EventTxResult, ProofAccountResource, TreasuryAccountResource, TxStatus};

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
    pub task: Task<(solana_program::keccak::Hash, u64, u64)>,
}

#[derive(Component)]
pub struct TaskSendAndConfirmTx {
    pub task: Task<(String, String)>,
}

#[derive(Component)]
pub struct TaskSendTx {
    pub task: Task<Transaction>,
}

#[derive(Component)]
pub struct TaskConfirmTx {
    pub task: Task<Signature>,
}

#[derive(Component)]
pub struct TaskRegisterWallet {
    pub task: Task<Option<Transaction>>,
}

#[derive(Component)]
pub struct TaskProcessTx {
    pub task: Task<Option<(String, Transaction, Option<u64>)>>,
}

#[derive(Component)]
pub struct TaskUpdateCurrentTx {
    pub task: Task<Option<(String, Transaction, Signature, Option<u64>)>>,
}

#[derive(Component)]
pub struct TaskProcessCurrentTx {
    pub task: Task<(Option<Signature>, TxStatus)>,
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
            info!("TaskUpdateResources Got Result.");
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
            ev_submit_hash_tx.send(EventSubmitHashTx(result));
            commands
                .entity(entity)
                .remove::<TaskGenerateHash>();
        }
    }
}

pub fn task_register_wallet(
    mut commands: Commands,
    mut ev_process_tx: EventWriter<EventProcessTx>,
    mut query: Query<(Entity, &mut TaskRegisterWallet)>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(tx) = block_on(future::poll_once(&mut task.task)) {
            if let Some(tx) = tx {
                ev_process_tx.send(EventProcessTx {
                    tx_type: "Register".to_string(),
                    tx,
                    hash_time: None,
                });
            } else {
                info!("Failed to confirm register wallet tx...");
            }

            commands
                .entity(entity)
                .remove::<TaskRegisterWallet>();
        }
    }

}

pub fn task_process_tx(
    mut commands: Commands,
    mut ev_process_tx: EventWriter<EventProcessTx>,
    mut query: Query<(Entity, &mut TaskProcessTx)>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            if let Some((tx_type, tx, hash_time)) = result {
                ev_process_tx.send(EventProcessTx {
                    tx_type,
                    tx,
                    hash_time,
                });
            } else {
                info!("Failed to confirm register wallet tx...");
            }

            commands
                .entity(entity)
                .remove::<TaskProcessTx>();
        }
    }

}

pub fn task_update_current_tx(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TaskUpdateCurrentTx)>,
    mut current_tx: ResMut<CurrentTx>
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            if let Some((tx_type, tx, sig, hash_time)) = result {
                current_tx.tx_type = tx_type;
                current_tx.hash_time = hash_time;
                current_tx.tx_sig = Some((tx, sig));
                let new_tx_status = TxStatus {
                    status: "SENDING".to_string(),
                    error: "".to_string()
                };
                current_tx.tx_status = new_tx_status;
            } else {
                current_tx.tx_sig = None;
                let new_tx_status = TxStatus {
                    status: "FAILED".to_string(),
                    error: "".to_string()
                };
                current_tx.tx_status = new_tx_status;
            }
            current_tx.elapsed_instant = Instant::now();
            current_tx.elapsed_seconds = 0;
            current_tx.interval_timer.reset();
            commands
                .entity(entity)
                .remove::<TaskUpdateCurrentTx>();
        }
    }

}

pub fn task_process_current_tx(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TaskProcessCurrentTx)>,
    mut current_tx: ResMut<CurrentTx>,
    mut ev_tx_result: EventWriter<EventTxResult>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some((sig, tx_status)) = block_on(future::poll_once(&mut task.task)) {

            let status = tx_status.status.clone();
            if status == "SUCCESS" || status == "FAILED" {
                let sig = if let Some(s) = sig {
                    s.to_string()
                } else {
                    "FAILED".to_string()
                };
                ev_tx_result.send(EventTxResult {
                    tx_type: current_tx.tx_type.clone(),
                    sig,
                    hash_time: current_tx.hash_time,
                    tx_time: current_tx.elapsed_seconds,
                    tx_status: tx_status.clone()
                });
            }
            current_tx.tx_status = tx_status;
            current_tx.interval_timer.reset();

            commands
                .entity(entity)
                .remove::<TaskProcessCurrentTx>();
        }
    }

}