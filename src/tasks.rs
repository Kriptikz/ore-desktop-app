use std::time::Instant;

use bevy::{
    prelude::*,
    tasks::{block_on, futures_lite::future, Task},
};
use ore::state::Bus;
use solana_sdk::{signature::Signature, transaction::Transaction};

use crate::{
    AppWallet, BussesResource, CurrentTx, EventProcessTx, EventSubmitHashTx, EventTxResult, MinerStatusResource, ProofAccountResource, TreasuryAccountResource, TxStatus
};

// Task Components
// TODO: tasks should return results so errors can be dealt with by the task handler system
pub struct TaskUpdateAppWalletSolBalanceData {
    pub sol_balance: f64,
    pub ore_balance: f64,
    pub proof_account_data: ProofAccountResource,
    pub treasury_account_data: TreasuryAccountResource,
    pub busses: Vec<Bus>
}
#[derive(Component)]
pub struct TaskUpdateAppWalletSolBalance {
    pub task: Task<Result<TaskUpdateAppWalletSolBalanceData, String>>,
}

#[derive(Component)]
pub struct TaskGenerateHash {
    pub task: Task<Result<(solana_program::keccak::Hash, u64, u32, u64), String>>,
}

#[derive(Component)]
pub struct TaskSendAndConfirmTx {
    pub task: Task<Result<(String, String), String>>,
}

#[derive(Component)]
pub struct TaskSendTx {
    pub task: Task<Result<Transaction, String>>,
}

#[derive(Component)]
pub struct TaskConfirmTx {
    pub task: Task<Result<Signature, String>>,
}

#[derive(Component)]
pub struct TaskRegisterWallet {
    pub task: Task<Option<Transaction>>,
}

#[derive(Component)]
pub struct TaskProcessTx {
    pub task: Task<Option<(String, Transaction, Option<(u64, u32)>)>>,
}

#[derive(Component)]
pub struct TaskUpdateCurrentTx {
    pub task: Task<Option<(String, Transaction, Signature, Option<(u64, u32)>)>>,
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
    mut busses_res: ResMut<BussesResource>,
    mut query: Query<(Entity, &mut TaskUpdateAppWalletSolBalance)>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            match result {
                Ok(result) => {
                    app_wallet.sol_balance = result.sol_balance;
                    app_wallet.ore_balance = result.ore_balance;
                    busses_res.busses = result.busses;
                    *proof_account_res = result.proof_account_data;
                    *treasury_account_res = result.treasury_account_data;
                },
                Err(e) => {
                    error!("Tasks UpdateResources error: {}", e);
                }
            }

            commands
                .entity(entity)
                .remove::<TaskUpdateAppWalletSolBalance>();
        }
    }
}

pub fn task_generate_hash(
    mut commands: Commands,
    mut ev_submit_hash_tx: EventWriter<EventSubmitHashTx>,
    miner_status: Res<MinerStatusResource>,
    mut query: Query<(Entity, &mut TaskGenerateHash)>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        let status = &miner_status.miner_status;

        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            match result {
                Ok(result) => {
                    info!("TaskGenerateHash Got Result.");
                    if status == "MINING" {
                        info!("Miner status is mining, submitting hash.");
                        ev_submit_hash_tx.send(EventSubmitHashTx(result));
                    } else {
                        info!("Miner status is not MINING, discarding hash.");
                    }
                },
                Err(e) => {
                    error!("Tasks GenerateHash error: {}", e);
                }
            }

            commands.entity(entity).remove::<TaskGenerateHash>();
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
                    hash_status: None,
                });
            } else {
                info!("Failed to confirm register wallet tx...");
            }

            commands.entity(entity).remove::<TaskRegisterWallet>();
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
            if let Some((tx_type, tx, hash_status)) = result {
                ev_process_tx.send(EventProcessTx {
                    tx_type,
                    tx,
                    hash_status,
                });
            } else {
                info!("Failed to process tx...");
            }

            commands.entity(entity).remove::<TaskProcessTx>();
        }
    }
}

pub fn task_update_current_tx(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TaskUpdateCurrentTx)>,
    mut current_tx: ResMut<CurrentTx>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            if let Some((tx_type, tx, sig, hash_time)) = result {
                current_tx.tx_type = tx_type;
                current_tx.hash_status = hash_time;
                current_tx.tx_sig = Some((tx, sig));
                let new_tx_status = TxStatus {
                    status: "SENDING".to_string(),
                    error: "".to_string(),
                };
                current_tx.tx_status = new_tx_status;
            } else {
                current_tx.tx_sig = None;
                let new_tx_status = TxStatus {
                    status: "FAILED".to_string(),
                    error: "".to_string(),
                };
                current_tx.tx_status = new_tx_status;
            }
            current_tx.elapsed_instant = Instant::now();
            current_tx.elapsed_seconds = 0;
            current_tx.interval_timer.reset();
            commands.entity(entity).remove::<TaskUpdateCurrentTx>();
        }
    }
}

pub fn task_process_current_tx(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TaskProcessCurrentTx)>,
    mut current_tx: ResMut<CurrentTx>,
    mut ev_tx_result: EventWriter<EventTxResult>,
    miner_status: Res<MinerStatusResource>,
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
                    hash_status: current_tx.hash_status,
                    tx_time: current_tx.elapsed_seconds,
                    tx_status: tx_status.clone(),
                });
            }
            current_tx.tx_status = tx_status;
            current_tx.interval_timer.reset();

            commands.entity(entity).remove::<TaskProcessCurrentTx>();
        }
    }
}
