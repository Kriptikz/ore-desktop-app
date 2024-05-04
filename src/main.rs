use std::{
    fs::{self, File},
    path::Path,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    tasks::AsyncComputeTaskPool,
};
use bevy_inspector_egui::{
    inspector_options::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions,
};
use cocoon::Cocoon;
use copypasta::{ClipboardContext, ClipboardProvider};
use events::*;
use serde::Deserialize;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};
use solana_transaction_status::{TransactionConfirmationStatus, UiTransactionEncoding};
use tasks::*;
use ui::{components::{BaseScreenNode, LockedScreenNode}, screens::{despawn_locked_screen, spawn_mining_screen, despawn_mining_screen, spawn_locked_screen}, systems::*};

pub mod events;
pub mod tasks;
pub mod ui;

#[derive(Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub ore_mint: String,
    pub threads: u64,
    pub fetch_ui_data_from_rpc_interval_ms: u64,
    pub tx_check_status_and_resend_interval_ms: u64,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Locked,
    Mining
}

fn main() {
    // TODO: put rpc_url in save.data and let user input from UI.
    let config: Config;
    let config_path = Path::new("config.toml");
    if config_path.exists() {
        let config_string = fs::read_to_string(config_path).unwrap();
        config = match toml::from_str(&config_string) {
            Ok(d) => d,
            Err(_) => {
                panic!("Failed to read config string.");
            }
        };
    } else {
        panic!("Please create a config.toml with the rpc_url.");
    }

    let rpc_connection = Arc::new(RpcClient::new_with_commitment(
        config.rpc_url.clone(),
        CommitmentConfig::confirmed(),
    ));

    let wallet: Keypair;
    let wallet_path = Path::new("save.data");

    // TODO: get password from user with UI.
    let cocoon = Cocoon::new(b"secret password");

    if wallet_path.exists() {
        let mut file = File::open(wallet_path).unwrap();
        let encoded = cocoon.parse(&mut file).unwrap();
        wallet = Keypair::from_bytes(&encoded).unwrap();
    } else {
        let new_wallet = Keypair::new();
        let wallet_bytes = new_wallet.to_bytes();

        let mut file = File::create(wallet_path).unwrap();

        let _ = cocoon.dump(wallet_bytes.to_vec(), &mut file).unwrap();
        wallet = new_wallet;
    }

    let _ =
        Pubkey::from_str(&config.ore_mint).expect("Config ore_mint is not a valid pubkey");


    let tx_send_interval = config.tx_check_status_and_resend_interval_ms;
    let rpc_ui_data_fetch_interval = config.fetch_ui_data_from_rpc_interval_ms;
    let threads = config.threads;
    App::new()
        .insert_state(GameState::Locked)
        .add_plugins(DefaultPlugins)
        //.add_plugins(WorldInspectorPlugin::new())
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(OreAppState { config })
        .insert_resource(CurrentTx {
            tx_type: "".to_string(),
            tx_sig: None,
            tx_status: TxStatus {
                status: "".to_string(),
                error: "".to_string()
            },
            hash_time: None,
            elapsed_instant: Instant::now(),
            elapsed_seconds: 0,
            interval_timer: Timer::new(Duration::from_millis(tx_send_interval), TimerMode::Once),
        })
        .insert_resource(AppWallet {
            wallet,
            sol_balance: 0.0,
            ore_balance: 0.0,
        })
        .insert_resource(MinerStatusResource {
            miner_threads: threads,
            ..Default::default()
        })
        // .init_resource::<MinerStatusResource>()
        // .register_type::<MinerStatusResource>()
        .init_resource::<ProofAccountResource>()
        .register_type::<ProofAccountResource>()
        .init_resource::<TreasuryAccountResource>()
        .register_type::<TreasuryAccountResource>()
        .insert_resource(RpcConnection {
            rpc: rpc_connection,
            fetch_ui_data_timer: Timer::new(Duration::from_millis(rpc_ui_data_fetch_interval), TimerMode::Once),
        })
        .add_event::<EventStartStopMining>()
        .add_event::<EventSubmitHashTx>()
        .add_event::<EventTxResult>()
        .add_event::<EventFetchUiDataFromRpc>()
        .add_event::<EventMineForHash>()
        .add_event::<EventRegisterWallet>()
        .add_event::<EventProcessTx>()
        .add_event::<EventResetEpoch>()
        .add_event::<EventClaimOreRewards>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, fps_text_update_system)
        .add_systems(Update, fps_counter_showhide)
        .add_systems(OnEnter(GameState::Locked), setup_locked_screen)
        .add_systems(OnExit(GameState::Locked), despawn_locked_screen)
        .add_systems(OnEnter(GameState::Mining), setup_mining_screen)
        .add_systems(OnExit(GameState::Mining), despawn_mining_screen)
        .add_systems(Update,(
                button_unlock,
        ).run_if(in_state(GameState::Locked)))
        .add_systems(Update,(
            (
                button_lock,
                button_copy_text,
                button_start_stop_mining,
                button_reset_epoch,
                button_claim_ore_rewards,
            ),
            (
                handle_event_start_stop_mining_clicked,
                handle_event_submit_hash_tx,
                handle_event_tx_result,
                handle_event_fetch_ui_data_from_rpc,
                handle_event_register_wallet,
                handle_event_process_tx,
                handle_event_reset_epoch,
                handle_event_mine_for_hash,
                handle_event_claim_ore_rewards,
            ),
            (
                task_update_app_wallet_sol_balance,
                task_generate_hash,
                task_register_wallet,
                task_process_tx,
                task_process_current_tx,
                task_update_current_tx,
            ),
            (
                update_app_wallet_ui,
                update_proof_account_ui,
                update_treasury_account_ui,
                update_miner_status_ui,
                update_current_tx_ui,
            ),
            (
                mouse_scroll,
                process_current_transaction,
                trigger_rpc_calls_for_ui,
            ),
        ).run_if(in_state(GameState::Mining)))
        .run();
}

// Startup system
fn setup_camera(mut commands: Commands) {
    // TODO: does camera need to be respawned?
    commands.spawn(Camera2dBundle::default());
    //setup_fps_counter(commands);
}

fn setup_mining_screen(mut commands: Commands, asset_server: Res<AssetServer>, app_wallet: Res<AppWallet>) {
    commands.spawn(EntityTaskHandler);
    commands.spawn(EntityTaskFetchUiData);
    spawn_mining_screen(commands.reborrow(), asset_server, app_wallet);
}

fn setup_locked_screen(mut commands: Commands, asset_server: Res<AssetServer>, app_wallet: Res<AppWallet>) {
    spawn_locked_screen(commands.reborrow(), asset_server, app_wallet);
}


// Components
#[derive(Component)]
pub struct EntityTaskHandler;

#[derive(Component)]
pub struct EntityTaskFetchUiData;

// Resources
#[derive(Resource)]
pub struct AppWallet {
    wallet: Keypair,
    sol_balance: f64,
    ore_balance: f64,
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct ProofAccountResource {
    current_hash: String,
    total_hashes: u64,
    total_rewards: u64,
    claimable_rewards: u64,
}

impl Default for ProofAccountResource {
    fn default() -> Self {
        Self {
            current_hash: "loading...".to_string(),
            total_hashes: Default::default(),
            total_rewards: Default::default(),
            claimable_rewards: Default::default(),
        }
    }
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct TreasuryAccountResource {
    balance: String,
    admin: String,
    difficulty: String,
    last_reset_at: i64,
    need_epoch_reset: bool,
    reward_rate: f64,
    total_claimed_rewards: f64,
}

impl Default for TreasuryAccountResource {
    fn default() -> Self {
        Self {
            balance: "loading...".to_string(),
            admin: "loading...".to_string(),
            difficulty: "loading...".to_string(),
            last_reset_at: 0,
            need_epoch_reset: false,
            reward_rate: 0.0,
            total_claimed_rewards: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct MinerStatusResource {
    miner_status: String,
    miner_threads: u64,
    current_timestamp: u64,
    sys_refresh_timer: Timer,
    sys_info: sysinfo::System,
}

impl Default for MinerStatusResource {
    fn default() -> Self {
        let mut sys_info = sysinfo::System::new_all();
        sys_info.refresh_all();

        Self {
            miner_status: "STOPPED".to_string(),
            miner_threads: 1,
            current_timestamp: get_unix_timestamp(),
            sys_refresh_timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
            sys_info,
        }
    }
}

#[derive(Resource)]
pub struct RpcConnection {
    // Cannot use the nonblocking client and await with the bevy tasks because bevy doesn't actually use tokio for async tasks.
    rpc: Arc<RpcClient>,
    pub fetch_ui_data_timer: Timer,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TxStatus {
    pub status: String,
    pub error: String,
}

#[derive(Resource, Debug)]
pub struct CurrentTx {
    pub tx_type: String,
    pub tx_sig: Option<(Transaction, Signature)>,
    pub tx_status: TxStatus,
    pub hash_time: Option<u64>,
    pub elapsed_instant: Instant,
    pub elapsed_seconds: u64,
    pub interval_timer: Timer,
}

// TODO: use real AppState for this
#[derive(Resource)]
pub struct OreAppState {
    config: Config,
}

pub fn process_current_transaction(
    mut commands: Commands,
    mut current_transaction: ResMut<CurrentTx>,
    time: Res<Time>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
    rpc_connection: Res<RpcConnection>,
) {
    if let Some((tx, sig)) = current_transaction.tx_sig.clone() {
        if current_transaction.tx_status.status != "SUCCESS" &&
            current_transaction.tx_status.status != "FAILED" &&
            current_transaction.tx_status.status != "INTERRUPTED" 
        {
            current_transaction.interval_timer.tick(time.delta());
            if current_transaction.interval_timer.just_finished() {
                let task_handler_entity = query_task_handler.get_single().unwrap();
                let pool = AsyncComputeTaskPool::get();
                let client = rpc_connection.rpc.clone();
                let task = pool.spawn(async move {
                    // start a timer
                    info!("SendAndConfirmTransaction....");

                    let send_cfg = RpcSendTransactionConfig {
                        skip_preflight: true,
                        preflight_commitment: Some(CommitmentLevel::Confirmed),
                        encoding: Some(UiTransactionEncoding::Base64),
                        max_retries: Some(0),
                        min_context_slot: None,
                    };

                    let mut status = "SENDING".to_string();
                    let mut error = "".to_string();
                    let sig = client.send_transaction_with_config(&tx, send_cfg);
                    if let Ok(sig) = sig {
                        let sigs = [sig];
                        match client.get_signature_statuses(&sigs) {
                            Ok(signature_statuses) => {
                                for signature_status in signature_statuses.value {
                                    if let Some(signature_status) = signature_status.as_ref() {
                                        if signature_status.confirmation_status.is_some() {
                                            let current_commitment = signature_status
                                                .confirmation_status
                                                .as_ref()
                                                .unwrap();
                                            match current_commitment {
                                                TransactionConfirmationStatus::Processed => {
                                                    info!("Transaction landed!");
                                                    info!("STATUS: {:?}", signature_status);
                                                    match &signature_status.status {
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
                                                    info!("STATUS: {:?}", signature_status);
                                                    match &signature_status.status {
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
                                        }
                                    }
                                }
                            }

                            // Handle confirmation errors
                            Err(err) => {
                                info!("Confirmation Error: {:?}", err.kind().to_string());
                            }
                        }
                        let tx_status = TxStatus {
                            status,
                            error,
                        };
                        return (Some(sig), tx_status);
                    }
                    let tx_status = TxStatus {
                        status,
                        error,
                    };
                    (None, tx_status)
                });
                commands
                    .entity(task_handler_entity)
                    .insert(TaskProcessCurrentTx { task });
            }
        }
    }
}

pub fn trigger_rpc_calls_for_ui(
    time: Res<Time>,
    mut rpc_connection: ResMut<RpcConnection>,
    mut event_fetch_ui_rpc_data: EventWriter<EventFetchUiDataFromRpc>
) {
    rpc_connection.fetch_ui_data_timer.tick(time.delta());
    if rpc_connection.fetch_ui_data_timer.just_finished() {
        event_fetch_ui_rpc_data.send(EventFetchUiDataFromRpc);
        rpc_connection.fetch_ui_data_timer.reset();
    }
}

pub fn shorten_string(text: String, max_len: usize) -> String {
    let len = text.len();
    if len > max_len {
        let prefix = &text[0..5];

        let suffix = &text[len - 5..len];

        format!("{}...{}", prefix, suffix)
    } else {
        text
    }
}

pub fn get_unix_timestamp() -> u64 {
    let time = SystemTime::now();
    time.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}
