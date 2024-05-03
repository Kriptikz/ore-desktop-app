use std::{
    fs::{self, File},
    path::Path,
    str::FromStr,
    sync::Arc,
    time::Duration,
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
use spl_associated_token_account::get_associated_token_address;
use tasks::*;
use ui::{layout::spawn_ui, systems::*};

pub mod events;
pub mod tasks;
pub mod ui;

#[derive(Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub ore_mint: String,
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
        config.rpc_url,
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

    let ore_mint =
        Pubkey::from_str(&config.ore_mint).expect("Config ore_mint is not a valid pubkey");

    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(OreAppState { ore_mint })
        .insert_resource(CurrentTx {
            tx_sig: None,
            status: "".to_string(),
            elapsed: 0,
            interval_timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
        })
        .insert_resource(AppWallet {
            wallet,
            sol_balance: 0.0,
            ore_balance: 0.0,
        })
        .init_resource::<MinerStatusResource>()
        .register_type::<MinerStatusResource>()
        .init_resource::<ProofAccountResource>()
        .register_type::<ProofAccountResource>()
        .init_resource::<TreasuryAccountResource>()
        .register_type::<TreasuryAccountResource>()
        .insert_resource(RpcConnection {
            rpc: rpc_connection,
        })
        .add_event::<EventStartStopMining>()
        .add_event::<EventSubmitHashTx>()
        .add_event::<EventTxResult>()
        .add_event::<EventFetchUiDataFromRpc>()
        .add_event::<EventMineForHash>()
        .add_event::<EventRegisterWallet>()
        .add_event::<EventProcessTx>()
        .add_event::<EventResetTreasury>()
        .add_systems(Startup, setup)
        .add_systems(Update, fps_text_update_system)
        .add_systems(Update, fps_counter_showhide)
        .add_systems(Update, button_update_sol_balance)
        .add_systems(Update, button_copy_text)
        .add_systems(Update, button_start_stop_mining)
        .add_systems(Update, button_reset_treasury)
        .add_systems(Update, handle_event_start_stop_mining_clicked)
        .add_systems(Update, handle_event_submit_hash_tx)
        .add_systems(Update, handle_event_tx_result)
        .add_systems(Update, handle_event_fetch_ui_data_from_rpc)
        .add_systems(Update, handle_event_register_wallet)
        .add_systems(Update, handle_event_process_tx)
        .add_systems(Update, handle_event_reset_treasury)
        .add_systems(Update, handle_event_mine_for_hash)
        .add_systems(Update, task_update_app_wallet_sol_balance)
        .add_systems(Update, task_generate_hash)
        .add_systems(Update, task_send_and_confirm_tx)
        .add_systems(Update, task_register_wallet)
        .add_systems(Update, task_process_tx)
        .add_systems(Update, task_process_current_tx)
        .add_systems(Update, task_update_current_tx)
        .add_systems(Update, mouse_scroll)
        .add_systems(Update, update_app_wallet_ui)
        .add_systems(Update, update_proof_account_ui)
        .add_systems(Update, update_treasury_account_ui)
        .add_systems(Update, update_miner_status_ui)
        .add_systems(Update, update_current_tx_ui)
        .add_systems(Update, process_current_transaction)
        .run();
}

// Startup system
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, app_wallet: Res<AppWallet>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(EntityTaskHandler);
    commands.spawn(EntityTaskFetchUiData);
    spawn_ui(commands.reborrow(), asset_server, app_wallet);
    setup_fps_counter(commands);
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
            reward_rate: 0.0,
            total_claimed_rewards: 0.0,
        }
    }
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MinerStatusResource {
    miner_status: String,
    cpu_usage: u64,
    ram_usage: f64,
}

impl Default for MinerStatusResource {
    fn default() -> Self {
        Self {
            miner_status: "STOPPED".to_string(),
            cpu_usage: Default::default(),
            ram_usage: Default::default(),
        }
    }
}

#[derive(Resource)]
pub struct RpcConnection {
    // Cannot use the nonblocking client and await with the bevy tasks because bevy doesn't actually use tokio for async tasks.
    rpc: Arc<RpcClient>,
}

#[derive(Resource)]
pub struct CurrentTx {
    pub tx_sig: Option<(Transaction, Signature)>,
    pub status: String,
    pub elapsed: u64,
    pub interval_timer: Timer,
}

// TODO: use real AppState for this
#[derive(Resource)]
pub struct OreAppState {
    ore_mint: Pubkey,
}

pub fn process_current_transaction(
    mut commands: Commands,
    mut current_transaction: ResMut<CurrentTx>,
    time: Res<Time>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
    rpc_connection: Res<RpcConnection>,
) {
    if let Some((tx, sig)) = current_transaction.tx_sig.clone() {
        if current_transaction.status != "SUCCESS" && current_transaction.status != "FAILED" {
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
                                                TransactionConfirmationStatus::Processed => {}
                                                TransactionConfirmationStatus::Confirmed
                                                | TransactionConfirmationStatus::Finalized => {
                                                    info!("Transaction landed!");
                                                    info!("STATUS: {:?}", signature_status);
                                                    match &signature_status.status {
                                                        Ok(_) => {
                                                            status = "SUCCESS".to_string();
                                                        }
                                                        Err(_) => {
                                                            status = "FAILED".to_string();
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
                                println!("{:?}", err.kind().to_string());
                            }
                        }
                        return (Some(sig), status);
                    }
                    (None, status)
                });
                commands
                    .entity(task_handler_entity)
                    .insert(TaskProcessCurrentTx { task });
            }
            // task send tx
            // task check tx status
        }
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
