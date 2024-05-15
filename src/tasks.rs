use std::time::{Duration, Instant};

use bevy::{
    prelude::*,
    tasks::{block_on, futures_lite::future, Task},
};
use ore::state::Bus;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{commitment_config::CommitmentLevel, signature::Signature, transaction::Transaction};
use solana_transaction_status::{TransactionConfirmationStatus, TransactionStatus, UiTransactionEncoding};

use crate::{
    ui::{components::{TextTxProcessorTxType, TxPopUpArea}, styles::{hex_black, CURRENT_TX_STATUS_BACKGROUND, FONT_ROBOTO, FONT_SIZE_TITLE}}, AppWallet, BussesResource, EventProcessTx, EventSubmitHashTx, EventTxResult, HashStatus, MinerStatusResource, ProofAccountResource, TreasuryAccountResource, TxProcessor, TxStatus, TxType
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
    pub task: Task<Result<Signature, String>>,
}

#[derive(Component)]
pub struct TaskCheckSigStatus {
    pub task: Task<Result<Option<TransactionStatus>, String>>,
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
pub struct TaskSendAndConfirmCheck {
    pub task: Task<(Entity, TxStatus)>,
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

pub fn handle_task_process_tx_result(
    mut commands: Commands,
    mut query_task_handler: Query<(Entity, &mut TaskProcessTx)>,
    mut query_pop_up: Query<Entity, With<TxPopUpArea>>,
    asset_server: Res<AssetServer>
    // mut query: Query<(Entity, &mut TaskProcessTx)>,
) {
    for (entity, mut task) in &mut query_task_handler.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            info!("Handle task process tx...");
            if let Some((tx_type, tx, hash_status)) = result {
                // spawn transaction entity
                // ev_process_tx.send(EventProcessTx {
                //     tx_type,
                //     tx,
                //     hash_status,
                // });
                let tx_type = match tx_type.as_str() {
                    "Mine" => {
                        TxType::Mine
                    },
                    "Reset" => {
                        TxType::ResetEpoch
                    },
                    "Claim" => {
                        TxType::Claim
                    },
                    "Stake" => {
                        TxType::Stake
                    },
                    "CreateAta" => {
                        TxType::CreateAta
                    },
                    _ => {
                        error!("Invalid tx_type, stop using strings....");
                        continue;
                    }
                };

                let hash_status = if let Some(hash_status) = hash_status {
                    Some(HashStatus {
                        hash_time: hash_status.0,
                        hash_difficulty: hash_status.1,
                    })
                } else {
                    None
                };

                let timer = Timer::new(Duration::from_millis(1000), TimerMode::Once);

                let pop_up_area = query_pop_up.single_mut();

                let new_tx = commands.spawn((
                    NodeBundle {
                        background_color: Color::WHITE.into(),
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(80.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(20.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::new(asset_server.load(CURRENT_TX_STATUS_BACKGROUND)),
                    TxProcessor {
                        tx_type: tx_type.clone(),
                        status: "SENDING".to_string(),
                        error: "".to_string(),
                        signature: None,
                        signed_tx: tx,
                        hash_status,
                        send_and_confirm_interval: timer,
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            tx_type.to_string(),
                            TextStyle {
                                font: asset_server.load(FONT_ROBOTO),
                                font_size: FONT_SIZE_TITLE,
                                color: Color::hex("#FFFFFF").unwrap(),
                            },
                        ),
                        Name::new("TextTxProcessorTxType"),
                        TextTxProcessorTxType,
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "SENDING".to_string(),
                            TextStyle {
                                font: asset_server.load(FONT_ROBOTO),
                                font_size: FONT_SIZE_TITLE,
                                color: Color::hex("#FFFFFF").unwrap(),
                            },
                        ),
                        Name::new("TextTxProcessorTxType"),
                        TextTxProcessorTxType,
                    ));
                }).id();

                commands.entity(pop_up_area).add_child(new_tx);

            } else {
                info!("Failed to process tx...");
            }

            commands.entity(entity).remove::<TaskProcessTx>();
        }
    }
}

pub fn handle_task_send_tx_result(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TaskSendTx, &mut TxProcessor)>,
) {
    for (entity, mut task, mut tx_processor) in &mut query.iter_mut() {
        if let Some(send_tx_result) = block_on(future::poll_once(&mut task.task)) {
            if let Ok(sig) = send_tx_result {
                info!("Tx sent, sig updated");
                tx_processor.signature = Some(sig);
            }
            commands.entity(entity).remove::<TaskSendTx>();
        }
    }
}

pub fn handle_task_tx_sig_check_results(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TaskCheckSigStatus, &mut TxProcessor)>,
) {
    for (entity, mut task, mut tx_processor) in &mut query.iter_mut() {
        if let Some(signature_status) = block_on(future::poll_once(&mut task.task)) {
            info!("Tx status check result");
            match signature_status {
                Ok(sig_status) => {
                    if let Some(sig_status) = sig_status {
                        if let Some(confirmation_status) = &sig_status.confirmation_status {
                            let current_commitment = confirmation_status;
                            let mut status;
                            let mut error = "".to_string();;
                            match current_commitment {
                                TransactionConfirmationStatus::Processed => {
                                    info!("Transaction landed!");
                                    info!("STATUS: {:?}", sig_status);
                                    match &sig_status.status {
                                        Ok(_) => {
                                            status = "PROCESSED".to_string();
                                        }
                                        Err(e) => {
                                            status = "FAILED".to_string();
                                            error = e.to_string();
                                        }
                                    }
                                }
                                TransactionConfirmationStatus::Confirmed
                                | TransactionConfirmationStatus::Finalized => {
                                    info!("Transaction landed!");
                                    info!("STATUS: {:?}", sig_status);
                                    match &sig_status.status {
                                        Ok(_) => {
                                            status = "SUCCESS".to_string();
                                        }
                                        Err(e) => {
                                            status = "FAILED".to_string();
                                            error = e.to_string();
                                        }
                                    }
                                }
                            }
                            tx_processor.status = status;
                            tx_processor.error = error;
                        }
                    }
                },
                Err(e) => {
                    error!("Error checking tx status: {}", e);
                }
            }
            commands.entity(entity).remove::<TaskCheckSigStatus>();
        }
    }
}