use std::{
    fs::{self},
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use events::*;
use serde::Deserialize;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    signature::{Keypair, Signature},
    transaction::Transaction,
};
use solana_transaction_status::{TransactionConfirmationStatus, UiTransactionEncoding};
use tasks::{
    task_generate_hash, task_process_current_tx, task_process_tx, task_register_wallet,
    task_update_app_wallet_sol_balance, task_update_current_tx, TaskProcessCurrentTx,
};
use ui::{
    components::{TextInput, TextPasswordInput},
    screens::{
        despawn_initial_setup_screen, despawn_locked_screen, despawn_mining_screen,
        spawn_initial_setup_screen, spawn_locked_screen, spawn_mining_screen,
    },
    ui_button_systems::{
        button_capture_text, button_claim_ore_rewards, button_copy_text, button_lock,
        button_reset_epoch, button_start_stop_mining, button_unlock,
    },
    ui_sync_systems::{
        fps_counter_showhide, fps_text_update_system, mouse_scroll, update_app_wallet_ui,
        update_current_tx_ui, update_miner_status_ui, update_proof_account_ui,
        update_text_input_ui, update_treasury_account_ui,
    },
};

pub mod events;
pub mod ore_utils;
pub mod tasks;
pub mod ui;
pub mod utils;

#[derive(Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub threads: u64,
    pub fetch_ui_data_from_rpc_interval_ms: u64,
    pub tx_check_status_and_resend_interval_ms: u64,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    InitialSetup,
    Locked,
    Mining,
}

fn main() {
    // TODO: put rpc_url in save.data and let user input from UI.
    let mut starting_state = GameState::InitialSetup;
    let config_path = Path::new("config.toml");
    let config: Option<Config> = if config_path.exists() {
        let config_string = fs::read_to_string(config_path).unwrap();
        let config = match toml::from_str(&config_string) {
            Ok(d) => {
                starting_state = GameState::Locked;
                Some(d)
            }
            Err(_) => None,
        };
        config
    } else {
        None
    };

    let config = config.unwrap_or(Config {
        rpc_url: "".to_string(),
        threads: 1,
        fetch_ui_data_from_rpc_interval_ms: 3000,
        tx_check_status_and_resend_interval_ms: 10000,
    });

    let tx_send_interval = config.tx_check_status_and_resend_interval_ms;
    let threads = config.threads;
    App::new()
        .insert_state(starting_state)
        .add_plugins(DefaultPlugins)
        //.add_plugins(WorldInspectorPlugin::new())
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(OreAppState {
            config,
            active_input_node: None,
        })
        .insert_resource(CurrentTx {
            tx_type: "".to_string(),
            tx_sig: None,
            tx_status: TxStatus {
                status: "".to_string(),
                error: "".to_string(),
            },
            hash_time: None,
            elapsed_instant: Instant::now(),
            elapsed_seconds: 0,
            interval_timer: Timer::new(Duration::from_millis(tx_send_interval), TimerMode::Once),
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
        .add_event::<EventStartStopMining>()
        .add_event::<EventSubmitHashTx>()
        .add_event::<EventTxResult>()
        .add_event::<EventFetchUiDataFromRpc>()
        .add_event::<EventMineForHash>()
        .add_event::<EventRegisterWallet>()
        .add_event::<EventProcessTx>()
        .add_event::<EventResetEpoch>()
        .add_event::<EventClaimOreRewards>()
        .add_event::<EventUnlock>()
        .add_event::<EventLock>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, fps_text_update_system)
        .add_systems(Update, fps_counter_showhide)
        .add_systems(Update, text_input)
        .add_systems(Update, update_text_input_ui)
        .add_systems(Update, button_capture_text)
        .add_systems(OnEnter(GameState::InitialSetup), setup_initial_setup_screen)
        .add_systems(
            OnExit(GameState::InitialSetup),
            despawn_initial_setup_screen,
        )
        .add_systems(OnExit(GameState::InitialSetup), despawn_locked_screen)
        .add_systems(OnEnter(GameState::Locked), setup_locked_screen)
        .add_systems(OnExit(GameState::Locked), despawn_locked_screen)
        .add_systems(OnEnter(GameState::Mining), setup_mining_screen)
        .add_systems(OnExit(GameState::Mining), despawn_mining_screen)
        .add_systems(
            Update,
            (button_unlock, handle_event_unlock, text_password_input)
                .run_if(in_state(GameState::Locked)),
        )
        .add_systems(
            Update,
            (
                // individual tuple max size is 12
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
                    handle_event_lock,
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
            )
                .run_if(in_state(GameState::Mining)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    //setup_fps_counter(commands);
}

fn setup_initial_setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_initial_setup_screen(commands.reborrow(), asset_server);
}

fn setup_mining_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_wallet: Res<AppWallet>,
    app_state: Res<OreAppState>,
) {
    commands.spawn(EntityTaskHandler);
    commands.spawn(EntityTaskFetchUiData);
    let config = &app_state.config;

    let rpc_connection = Arc::new(RpcClient::new_with_commitment(
        config.rpc_url.clone(),
        CommitmentConfig::confirmed(),
    ));
    commands.insert_resource(RpcConnection {
        rpc: rpc_connection,
        fetch_ui_data_timer: Timer::new(
            Duration::from_millis(config.fetch_ui_data_from_rpc_interval_ms),
            TimerMode::Once,
        ),
    });
    spawn_mining_screen(commands.reborrow(), asset_server, app_wallet);
}

fn setup_locked_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<OreAppState>,
) {
    let pass_text_entity = spawn_locked_screen(commands.reborrow(), asset_server);
    app_state.active_input_node = pass_text_entity;
}

// Components
#[derive(Component)]
pub struct EntityTaskHandler;

#[derive(Component)]
pub struct EntityTaskFetchUiData;

// Resources
#[derive(Resource)]
pub struct AppWallet {
    wallet: Arc<Keypair>,
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

#[derive(Resource)]
pub struct OreAppState {
    config: Config,
    active_input_node: Option<Entity>,
}

pub fn process_current_transaction(
    mut commands: Commands,
    mut current_transaction: ResMut<CurrentTx>,
    time: Res<Time>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
    rpc_connection: Res<RpcConnection>,
) {
    if let Some((tx, _sig)) = current_transaction.tx_sig.clone() {
        if current_transaction.tx_status.status != "SUCCESS"
            && current_transaction.tx_status.status != "FAILED"
            && current_transaction.tx_status.status != "INTERRUPTED"
        {
            current_transaction.interval_timer.tick(time.delta());
            if current_transaction.interval_timer.just_finished() {
                let task_handler_entity = query_task_handler.get_single().unwrap();
                let pool = AsyncComputeTaskPool::get();
                let client = rpc_connection.rpc.clone();
                let task = pool.spawn(async move {
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
                        let tx_status = TxStatus { status, error };
                        return (Some(sig), tx_status);
                    }
                    let tx_status = TxStatus { status, error };
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
    mut event_fetch_ui_rpc_data: EventWriter<EventFetchUiDataFromRpc>,
) {
    rpc_connection.fetch_ui_data_timer.tick(time.delta());
    if rpc_connection.fetch_ui_data_timer.just_finished() {
        event_fetch_ui_rpc_data.send(EventFetchUiDataFromRpc);
        rpc_connection.fetch_ui_data_timer.reset();
    }
}

pub struct BackspaceTimer {
    pub timer: Timer,
}

impl Default for BackspaceTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

pub fn text_password_input(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<ButtonInput<KeyCode>>,
    app_state: Res<OreAppState>,
    mut backspace_timer: Local<BackspaceTimer>,
    time: Res<Time>,
    mut active_text_query: Query<(Entity, &mut TextInput), With<TextPasswordInput>>,
    mut event_writer: EventWriter<EventUnlock>,
) {
    if let Some(app_state_active_text_entity) = app_state.active_input_node {
        for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
            if active_text_entity == app_state_active_text_entity {
                if kbd.just_pressed(KeyCode::Enter) {
                    event_writer.send(EventUnlock);
                }
                if kbd.just_pressed(KeyCode::Home) {
                    text_input.hidden = !text_input.hidden;
                }
                if kbd.just_pressed(KeyCode::Backspace) {
                    text_input.text.pop();
                    // reset, to ensure multiple presses aren't going to result in multiple backspaces
                    backspace_timer.timer.reset();
                } else if kbd.pressed(KeyCode::Backspace) {
                    backspace_timer.timer.tick(time.delta());
                    if backspace_timer.timer.just_finished() {
                        text_input.text.pop();
                        backspace_timer.timer.reset();
                    }
                }
                for ev in evr_char.read() {
                    let mut cs = ev.char.chars();

                    let c = cs.next();
                    if let Some(char) = c {
                        if !char.is_control() {
                            text_input.text.push_str(ev.char.as_str());
                        }
                    }
                }
            }
        }
    }
}

pub fn text_input(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<ButtonInput<KeyCode>>,
    app_state: Res<OreAppState>,
    mut backspace_timer: Local<BackspaceTimer>,
    time: Res<Time>,
    mut active_text_query: Query<
        (Entity, &mut Text),
        (With<TextInput>, Without<TextPasswordInput>),
    >,
) {
    if let Some(app_state_active_text_entity) = app_state.active_input_node {
        for (active_text_entity, mut active_text_text) in active_text_query.iter_mut() {
            if active_text_entity == app_state_active_text_entity {
                if kbd.just_pressed(KeyCode::Enter) {
                    // println!("Text input: {}", &*string);
                    // string.clear();
                }
                if kbd.just_pressed(KeyCode::Backspace) {
                    active_text_text.sections[0].value.pop();
                    // reset, to ensure multiple presses aren't going to result in multiple backspaces
                    backspace_timer.timer.reset();
                } else if kbd.pressed(KeyCode::Backspace) {
                    backspace_timer.timer.tick(time.delta());
                    if backspace_timer.timer.just_finished() {
                        active_text_text.sections[0].value.pop();
                        backspace_timer.timer.reset();
                    }
                }
                for ev in evr_char.read() {
                    let mut cs = ev.char.chars();

                    let c = cs.next();
                    if let Some(char) = c {
                        if !char.is_control() {
                            active_text_text.sections[0]
                                .value
                                .push_str(ev.char.as_str());
                        }
                    }
                }
            }
        }
    }
}
