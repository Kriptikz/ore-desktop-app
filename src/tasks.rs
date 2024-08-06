use std::time::{Duration, Instant};

use bevy::{
    prelude::*,
    tasks::{block_on, futures_lite::future, Task}, winit::{UpdateMode, WinitSettings},
};
use drillx::Solution;
use ore_api::state::Bus;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{commitment_config::CommitmentLevel, signature::Signature, transaction::Transaction};
use solana_transaction_status::{TransactionConfirmationStatus, TransactionStatus, UiTransactionEncoding};

use crate::{
    ui::{components::{SpinnerIcon, TextTxProcessorTxType, ToggleAutoMineParent, TxPopUpArea}, styles::{hex_black, CURRENT_TX_STATUS_BACKGROUND, FONT_REGULAR, FONT_SIZE_MEDIUM, SPINNER_ICON, TX_POP_UP_BACKGROUND}}, utils::get_unix_timestamp, AppConfig, AppWallet, BussesResource, EventFetchUiDataFromRpc, EventProcessTx, EventSubmitHashTx, EventTxResult, HashStatus, MinerStatusResource, OreAppState, ProofAccountResource, TreasuryAccountResource, TxProcessor, TxStatus, TxType, FAST_DURATION, REGULAR_DURATION
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
    pub task: Task<Result<(Solution, u32, u64, u64), String>>,
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

pub struct SigCheckResults {
    pub ents: Vec<Entity>,
    pub sigs: Vec<Signature>,
    pub sig_statuses: Vec<Option<TransactionStatus>>,
}

#[derive(Component)]
pub struct TaskSigChecks {
    pub task: Task<Result<SigCheckResults, String>>,
}

#[derive(Component)]
pub struct TaskConfirmTx {
    pub task: Task<Result<Signature, String>>,
}

#[derive(Component)]
pub struct TaskRegisterWallet {
    pub task: Task<Option<Transaction>>,
}

pub struct TaskProcessTxData {
    // TODO: change this to enum
    pub tx_type: String,
    pub signature: Option<Signature>,
    pub signed_tx: Option<Transaction>,
    pub hash_time: Option<(u64, u32)> // hash_time, difficulty
}

#[derive(Component)]
pub struct TaskProcessTx {
    pub task: Task<Result<TaskProcessTxData, (TaskProcessTxData, String)>>,
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
    mut query_toggle_mine: Query<&mut Visibility, With<ToggleAutoMineParent>>,
    mut event_fetch_ui_data: EventWriter<EventFetchUiDataFromRpc>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            let mut fetch_failed = false;
            match result {
                Ok(result) => {
                    // if result.proof_account_data.challenge == "Not Found" {
                    //     // TODO: Spawn Open Button
                    // } else {
                           // if let Ok(mut vis) = query_toggle_mine.get_single_mut() {
                           //     *vis = Visibility::Visible;
                           // }
                    // }
                    if let Ok(mut vis) = query_toggle_mine.get_single_mut() {
                        *vis = Visibility::Visible;
                    }
                    app_wallet.sol_balance = result.sol_balance;
                    app_wallet.ore_balance = result.ore_balance;
                    busses_res.busses = result.busses;
                    *proof_account_res = result.proof_account_data;
                    *treasury_account_res = result.treasury_account_data;
                },
                Err(e) => {
                    error!("Tasks UpdateResources error: {}", e);
                    fetch_failed = true;
                }
            }

            commands
                .entity(entity)
                .remove::<TaskUpdateAppWalletSolBalance>();

            if fetch_failed {
                event_fetch_ui_data.send(EventFetchUiDataFromRpc);
            }
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
                    if status == "MINING" {
                        ev_submit_hash_tx.send(EventSubmitHashTx(result));
                    } else {
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
                error!("Failed to confirm register wallet tx...");
            }

            commands.entity(entity).remove::<TaskRegisterWallet>();
        }
    }
}

pub fn handle_task_process_tx_result(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_wallet: Res<AppWallet>,
    proof_account: Res<ProofAccountResource>,
    ore_app_state: Res<OreAppState>,
    mut winit_settings: ResMut<WinitSettings>,
    mut query_task_handler: Query<(Entity, &mut TaskProcessTx)>,
    mut event_writer: EventWriter<EventTxResult>,
    mut query_pop_up: Query<Entity, With<TxPopUpArea>>,
    // mut query: Query<(Entity, &mut TaskProcessTx)>,
) {
    for (entity, mut task) in &mut query_task_handler.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            match result {
                Ok(task_process_tx_data) => {
                    // spawn transaction entity
                    // ev_process_tx.send(EventProcessTx {
                    //     tx_type,
                    //     tx,
                    //     hash_status,
                    // });
                    let tx_type = task_process_tx_data.tx_type.clone();
                    let tx = task_process_tx_data.signed_tx;
                    let hash_status = task_process_tx_data.hash_time;
                    let tx_type = match tx_type.as_str() {
                        "Mine" => {
                            TxType::Mine
                        },
                        "Register" => {
                            TxType::Register
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
                        "Airdrop" => {
                            let tx_result = EventTxResult {
                                tx_type: task_process_tx_data.tx_type,
                                sig: task_process_tx_data.signature.unwrap().to_string(),
                                tx_time: 0,
                                hash_status: None,
                                tx_status: TxStatus {
                                    status: "SUCCESS".to_string(),
                                    error: "".to_string(),
                                }

                            };
                            event_writer.send(tx_result);
                            commands.entity(entity).remove::<TaskProcessTx>();
                            continue;
                        },
                        _ => {
                            error!("Invalid tx_type, stop using strings....");
                            commands.entity(entity).remove::<TaskProcessTx>();
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

                    let tx_send_interval = ore_app_state.config.tx_send_interval;
                    let timer = Timer::new(Duration::from_millis(tx_send_interval), TimerMode::Once);

                    let pop_up_area = query_pop_up.single_mut();

                    let sol_balance = app_wallet.sol_balance;
                    let staked_balance = if tx_type == TxType::Mine {
                        let current_ts = get_unix_timestamp();
                        let time_since_last_hash = current_ts - proof_account.last_hash_at as u64;
                        if time_since_last_hash >= 62 || time_since_last_hash <= 53 {
                            None
                        } else {
                            Some(proof_account.stake)
                        }
                    } else {
                        None
                    };

                    let new_tx = commands.spawn((
                        NodeBundle {
                            background_color: Color::WHITE.into(),
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(40.0),
                                flex_direction: FlexDirection::Row,
                                // row_gap: Val::Px(20.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceAround,
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(asset_server.load(TX_POP_UP_BACKGROUND)),
                        TxProcessor {
                            tx_type: tx_type.clone(),
                            status: "SENDING".to_string(),
                            error: "".to_string(),
                            sol_balance,
                            staked_balance,
                            signature: None,
                            signed_tx: tx,
                            hash_status,
                            created_at: Instant::now(),
                            challenge: proof_account.challenge.clone(),
                            send_and_confirm_interval: timer,
                        },
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                tx_type.to_string(),
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
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
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: Color::ORANGE.into(),
                                },
                            ),
                            Name::new("TextTxProcessorTxType"),
                            TextTxProcessorTxType,
                        ));
                        parent.spawn((
                            NodeBundle {
                                background_color: Color::WHITE.into(),
                                style: Style {
                                    width: Val::Px(24.0),
                                    height: Val::Px(24.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            Name::new("SpinnerIcon"),
                            UiImage::new(asset_server.load(SPINNER_ICON)),
                            SpinnerIcon,
                        ));
                    }).id();

                    commands.entity(pop_up_area).add_child(new_tx);

                    winit_settings.focused_mode = UpdateMode::ReactiveLowPower { wait: FAST_DURATION };
                    winit_settings.unfocused_mode = UpdateMode::ReactiveLowPower { wait: FAST_DURATION };
                },
                Err((task_process_tx_data, error_str)) => {
                    let sig = if let Some(sig) = &task_process_tx_data.signature {
                        sig.to_string()
                    } else {
                        "".to_string()
                    };
                    let tx_result = EventTxResult {
                        tx_type: task_process_tx_data.tx_type,
                        sig,
                        tx_time: 0,
                        hash_status: None,
                        tx_status: TxStatus {
                            status: "FAILED".to_string(),
                            error: error_str.clone(),
                        }

                    };
                    event_writer.send(tx_result);
                }
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
            // the txn's are sent on an interval, only successfull sends will 
            // return the sig.
            // Txn's will expire after about 80's automatically
            if let Ok(sig) = send_tx_result {
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
            match signature_status {
                Ok(sig_status) => {
                    if let Some(sig_status) = sig_status {
                        if let Some(confirmation_status) = &sig_status.confirmation_status {
                            let current_commitment = confirmation_status;
                            let mut status;
                            let mut error = "".to_string();
                            match current_commitment {
                                TransactionConfirmationStatus::Processed => {
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

pub fn handle_task_got_sig_checks(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TaskSigChecks)>,
    mut query_tx_processors: Query<&mut TxProcessor>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(signature_status) = block_on(future::poll_once(&mut task.task)) {
            match signature_status {
                Ok(sig_check_results) => {
                    let ents = sig_check_results.ents;
                    let sigs = sig_check_results.sigs;
                    let sig_statuses = sig_check_results.sig_statuses;

                    if ents.len() == sigs.len() && sigs.len() == sig_statuses.len() {
                        for i in 0..ents.len() {
                            let ent = ents[i];
                            let sig_status = &sig_statuses[i];

                            if let Some(sig_status) = sig_status {
                                if let Some(confirmation_status) = &sig_status.confirmation_status {
                                    let current_commitment = confirmation_status;
                                    let status;
                                    let mut error = "".to_string();
                                    match current_commitment {
                                        TransactionConfirmationStatus::Processed => {
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

                                    // let tx_processor = query_tx_processors.get_mut(ent);
                                    if let Ok(mut tx_processor) = query_tx_processors.get_mut(ent) {
                                        tx_processor.status = status;
                                        tx_processor.error = error;
                                    }
                                }
                            }
                        }
                    } else {
                        error!("Error: sigs check ents, sigs, sig_statuses lengths missmatch.");
                    }
                },
                Err(e) => {
                    error!("Error checking tx status: {}", e);
                }
            }
            commands.entity(entity).remove::<TaskSigChecks>();
        }
    }
}
