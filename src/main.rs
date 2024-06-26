use std::{
    borrow::BorrowMut, fs, path::Path, str::FromStr, sync::{mpsc, Arc, Mutex}, time::{Duration, Instant}
};

use async_compat::{Compat, CompatExt};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, tasks::{futures_lite::StreamExt, AsyncComputeTaskPool, IoTaskPool, Task}, window::RequestRedraw, winit::{UpdateMode, WinitSettings}};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions};
use copypasta::{ClipboardContext, ClipboardProvider};
use crossbeam_channel::{unbounded, Receiver};
use events::*;
use ore::{state::{Bus, Proof, Treasury}, utils::AccountDeserialize};
use ore_utils::proof_pubkey;
use serde::{Deserialize, Serialize};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient}, rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSendTransactionConfig}, rpc_filter::RpcFilterType, rpc_response::{Response, RpcKeyedAccount}};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel}, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signature}, signer::Signer, transaction::Transaction, keccak::Hash as KeccakHash
};
use solana_transaction_status::{TransactionConfirmationStatus, UiTransactionEncoding};
use tasks::{
    handle_task_got_sig_checks, handle_task_process_tx_result, handle_task_send_tx_result, handle_task_tx_sig_check_results, task_generate_hash, task_register_wallet, task_update_app_wallet_sol_balance, TaskCheckSigStatus, TaskSendTx
};
use ui::{
    components::{ButtonCaptureTextInput, SpinnerIcon, TextInput, TextPasswordInput, FpsRoot, FpsText},
    screens::{screen_despawners::{
        despawn_initial_setup_screen, despawn_locked_screen,
        despawn_mining_screen, despawn_wallet_setup_screen, 
    }, screen_initial_setup::spawn_initial_setup_screen, screen_locked::spawn_locked_screen, screen_mining::spawn_mining_screen, screen_setup_wallet::spawn_wallet_setup_screen},
    ui_button_systems::{
        button_auto_scroll, button_capture_text, button_claim_ore_rewards, button_copy_text, button_generate_wallet, button_lock, button_open_web_tx_explorer, button_request_airdrop, button_save_config, button_save_wallet, button_stake_ore, button_start_stop_mining, button_unlock, tick_button_cooldowns
    },
    ui_sync_systems::{
        fps_counter_showhide, fps_text_update_system, mouse_scroll, update_active_text_input_cursor_vis, update_app_wallet_ui, update_busses_ui, update_miner_status_ui, update_proof_account_ui, update_text_input_ui, update_treasury_account_ui
    },
};

// screens::{
//     spawn_initial_setup_screen, spawn_locked_screen, spawn_mining_screen,
// },

pub const FAST_DURATION: Duration = Duration::from_millis(30);
pub const REGULAR_DURATION: Duration = Duration::from_millis(100);
pub const SLOW_DURATION: Duration = Duration::from_millis(1000);

pub mod events;
pub mod ore_utils;
pub mod tasks;
pub mod ui;
pub mod utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub is_devnet: bool,
    pub threads: u64,
    pub ui_fetch_interval: u64,
    pub tx_send_interval: u64,
    pub tx_sigs_check_interval: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://floral-dawn-pallet.solana-devnet.quiknode.pro/8b38be5427b44d3b42dc67c891dea71a56cd3a8c/".to_string(),
            ws_url: "wss://floral-dawn-pallet.solana-devnet.quiknode.pro/8b38be5427b44d3b42dc67c891dea71a56cd3a8c/".to_string(),
            is_devnet: true,
            threads: 1,
            ui_fetch_interval: 1000,
            tx_send_interval: 3000,
            tx_sigs_check_interval: 1000,
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    ConfigSetup,
    WalletSetup,
    Locked,
    Mining,
}

fn main() {
    let mut starting_state = GameState::ConfigSetup;
    let config_path = Path::new("config.toml");
    let config: Option<AppConfig> = if config_path.exists() {
        let config_string = fs::read_to_string(config_path).unwrap();
        let config = match toml::from_str(&config_string) {
            Ok(d) => {
                starting_state = GameState::WalletSetup;
                Some(d)
            }
            Err(_) => None,
        };
        config
    } else {
        None
    };

    if starting_state == GameState::WalletSetup {
        let wallet_path = Path::new("save.data");
        if wallet_path.exists() {
            starting_state = GameState::Locked;
        }
    }

    let config = config.unwrap_or(AppConfig::default());

    // let tx_send_interval = config.tx_send_interval;
    let threads = config.threads;
    App::new()
        .insert_state(starting_state)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Unofficial Ore App".to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resizable: false,
                        focused: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
        )
        // .add_plugins(WorldInspectorPlugin::new())
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::ReactiveLowPower { wait: REGULAR_DURATION },
            unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower { wait: REGULAR_DURATION },
        })
        .insert_resource(OreAppState {
            config,
            active_input_node: None,
        })
        .insert_resource(MinerStatusResource {
            miner_threads: threads,
            ..Default::default()
        })
        .insert_resource(BussesResource {
            busses: vec![],
            current_bus_id: 0,
        })
        .init_resource::<ProofAccountResource>()
        .register_type::<ProofAccountResource>()
        .init_resource::<TreasuryAccountResource>()
        .register_type::<TreasuryAccountResource>()
        .add_event::<EventStartStopMining>()
        .add_event::<EventStopMining>()
        .add_event::<EventSubmitHashTx>()
        .add_event::<EventTxResult>()
        .add_event::<EventFetchUiDataFromRpc>()
        .add_event::<EventMineForHash>()
        .add_event::<EventRegisterWallet>()
        .add_event::<EventProcessTx>()
        .add_event::<EventClaimOreRewards>()
        .add_event::<EventStakeOre>()
        .add_event::<EventUnlock>()
        .add_event::<EventLock>()
        .add_event::<EventSaveConfig>()
        .add_event::<EventGenerateWallet>()
        .add_event::<EventSaveWallet>()
        .add_event::<EventLoadKeypairFile>()
        .add_event::<EventRequestAirdrop>()
        .add_event::<EventCheckSigs>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, fps_text_update_system)
        .add_systems(Update, fps_counter_showhide)
        .add_systems(Update, text_input)
        .add_systems(Update, update_text_input_ui)
        .add_systems(Update, button_capture_text)
        .add_systems(Update, update_active_text_input_cursor_vis)
        .add_systems(Update, tick_button_cooldowns)
        .add_systems(OnEnter(GameState::ConfigSetup), setup_initial_setup_screen)
        .add_systems(
            OnExit(GameState::ConfigSetup),
            (
                despawn_initial_setup_screen,
            )
        )
        .add_systems(OnExit(GameState::ConfigSetup), despawn_locked_screen)
        .add_systems(OnEnter(GameState::WalletSetup), setup_wallet_setup_screen)
        .add_systems(OnExit(GameState::WalletSetup), despawn_wallet_setup_screen)
        .add_systems(OnEnter(GameState::Locked), setup_locked_screen)
        .add_systems(OnExit(GameState::Locked), despawn_locked_screen)
        .add_systems(OnEnter(GameState::Mining), setup_mining_screen)
        .add_systems(OnExit(GameState::Mining), despawn_mining_screen)
        .add_systems(
            Update,
            (
                button_save_config,
                handle_event_save_config,
            )
                .run_if(in_state(GameState::ConfigSetup)),
        )
        .add_systems(
            Update,
            (
                (
                    button_generate_wallet,
                    button_save_wallet,
                ),
                (
                    handle_event_generate_wallet,
                    handle_event_save_wallet,
                    handle_event_load_keypair_file,
                ),
                (
                    text_password_input,
                    file_drop,
                ),
            )
                .run_if(in_state(GameState::WalletSetup)),
        )
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
                    button_claim_ore_rewards,
                    button_stake_ore,
                    button_auto_scroll,
                    button_open_web_tx_explorer,
                    button_request_airdrop
                ),
                (
                    handle_event_start_stop_mining_clicked,
                    handle_event_submit_hash_tx,
                    handle_event_tx_result,
                    handle_event_fetch_ui_data_from_rpc,
                    handle_event_register_wallet,
                    handle_event_mine_for_hash,
                    handle_event_claim_ore_rewards,
                    handle_event_stake_ore,
                    handle_event_lock,
                    handle_event_request_airdrop,
                    handle_event_check_sigs,
                ),
                (
                    task_update_app_wallet_sol_balance,
                    task_generate_hash,
                    task_register_wallet,
                    handle_task_process_tx_result,
                    handle_task_send_tx_result,
                    handle_task_tx_sig_check_results,
                    handle_task_got_sig_checks,
                ),
                (
                    update_app_wallet_ui,
                    update_busses_ui,
                    update_proof_account_ui,
                    update_treasury_account_ui,
                    update_miner_status_ui,
                ),
                (
                    mouse_scroll,
                    tx_processor_result_checks,
                    tx_processors_send,
                    tx_processors_sigs_check,
                    mining_screen_hotkeys,
                    spin_spinner_icons,
                    read_accounts_update_channel,
                ),
            )
                .run_if(in_state(GameState::Mining)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // setup_fps_counter(commands);
}

fn setup_initial_setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config_path = Path::new("config.toml");
    let config: Option<AppConfig> = if config_path.exists() {
        let config_string = fs::read_to_string(config_path).unwrap();
        let config = match toml::from_str(&config_string) {
            Ok(d) => {
                Some(d)
            }
            Err(_) => None,
        };
        config
    } else {
        None
    };

    let config = config.unwrap_or(AppConfig::default());

    spawn_initial_setup_screen(commands.reborrow(), asset_server, config);
}

fn setup_wallet_setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_wallet_setup_screen(commands.reborrow(), asset_server);
}

fn setup_mining_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_wallet: Res<AppWallet>,
    app_state: Res<OreAppState>,
    mut event_writer: EventWriter<EventFetchUiDataFromRpc>,
) {
    commands.spawn((EntityTaskHandler, Name::new("EntityTaskHandler")));
    commands.spawn((EntityTaskFetchUiData, Name::new("EntityFetchUiData")));
    let config = &app_state.config;

    let rpc_connection = Arc::new(RpcClient::new_with_commitment(
        config.rpc_url.clone(),
        CommitmentConfig::confirmed(),
    ));

    commands.insert_resource(RpcConnection {
        rpc: rpc_connection,
        fetch_ui_data_timer: Timer::new(
            Duration::from_millis(config.ui_fetch_interval),
            TimerMode::Once,
        ),
    });
    spawn_mining_screen(commands.reborrow(), asset_server, app_wallet.wallet.pubkey().to_string(), app_wallet.sol_balance, app_wallet.ore_balance, app_state.config.clone());

    // let (mut account_subscription_client, account_subscription_receiver)

    let task_pool = IoTaskPool::get();

    let (sender, receiver) = unbounded::<AccountUpdatesData>();

    let account_update_channel = AccountUpdatesChannel {
        channel: receiver.clone(),
    };


    // TODO: use an entity here, we need to unsubscribe and cleanup this task when switching screens.
    commands.insert_resource(account_update_channel);

    let wallet_pubkey = app_wallet.wallet.pubkey().clone();

    let ws_url = config.ws_url.clone();

    task_pool.spawn(Compat::new(async move {
        let sender = sender.clone();
        let ps_client = PubsubClient::new(&ws_url).await;
        if let Ok(ps_client) = ps_client {
            let ps_client = Arc::new(ps_client);

            let sender_c = sender.clone();
            let ps_client_c = ps_client.clone();
            task_pool.spawn(async move {
                let ps_client = ps_client_c;
                let sender = sender_c;
                let account_pubkey = proof_pubkey(wallet_pubkey);
                let mut pubsub =
                    ps_client.account_subscribe(
                        &account_pubkey,
                    Some(
                        RpcAccountInfoConfig {
                                encoding: Some(UiAccountEncoding::Base64),
                                data_slice: None,
                                commitment: Some(CommitmentConfig::confirmed()),
                                min_context_slot: None,
                        }
                    )).await;

                    loop {
                        if let Ok((account_sub_client, _account_sub_receiver)) = pubsub.as_mut() {
                            if let Some(response) = account_sub_client.next().await {
                                let data = response.value.data.decode();
                                if let Some(data_bytes) = data {
                                    let proof = Proof::try_from_bytes(&data_bytes);
                                    if let Ok(proof) = proof {
                                        let _ = sender.send(AccountUpdatesData::ProofData(*proof));
                                    }
                                }
                            }
                        }
                    }
                }).detach();

            let sender_c = sender.clone();
            let ps_client_c = ps_client.clone();
            task_pool.spawn(async move {
                let ps_client = ps_client_c;
                let sender = sender_c;
                let account_pubkey = ore::CONFIG_ADDRESS;
                let mut pubsub =
                    ps_client.account_subscribe(
                        &account_pubkey,
                    Some(
                        RpcAccountInfoConfig {
                                encoding: Some(UiAccountEncoding::Base64),
                                data_slice: None,
                                commitment: Some(CommitmentConfig::confirmed()),
                                min_context_slot: None,
                        }
                    )).await;

                    loop {
                        if let Ok((account_sub_client, _account_sub_receiver)) = pubsub.as_mut() {
                            if let Some(response) = account_sub_client.next().await {
                                let data = response.value.data.decode();
                                if let Some(data_bytes) = data {
                                    let ore_config = ore::state::Config::try_from_bytes(&data_bytes);
                                    if let Ok(ore_config) = ore_config {
                                        let _ = sender.send(AccountUpdatesData::TreasuryConfigData(*ore_config));
                                        continue;
                                    }
                                }
                            }
                        }
                    }

                }).detach();

            let sender_c = sender.clone();
            let ps_client_c = ps_client.clone();
            task_pool.spawn(async move {
                let ps_client = ps_client_c;
                let sender = sender_c;
                let account_pubkey = ore::ID;
                let mut pubsub =
                    ps_client.program_subscribe(
                        &account_pubkey,
                        Some(RpcProgramAccountsConfig {
                            filters: Some(vec![RpcFilterType::DataSize(32)]),
                            account_config: RpcAccountInfoConfig {
                                encoding: Some(UiAccountEncoding::Base64),
                                data_slice: None,
                                commitment: Some(CommitmentConfig::confirmed()),
                                min_context_slot: None,
                            },
                            with_context: None,
                        })
                    ).await;

                    loop {
                        if let Ok((account_sub_client, _account_sub_receiver)) = pubsub.as_mut() {
                            if let Some(response) = account_sub_client.next().await {

                                let data = response.value.account.data.decode();
                                if let Some(data_bytes) = data {
                                    let bus = Bus::try_from_bytes(&data_bytes);
                                    if let Ok(bus) = bus {
                                        let _ = sender.send(AccountUpdatesData::BusData(*bus));
                                    }
                                }
                            }
                        }
                    }
                }).detach();
        }
    })).detach();


    event_writer.send(EventFetchUiDataFromRpc);
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

#[derive(Clone, PartialEq, Eq)]
pub enum TxType {
    Mine,
    Register,
    ResetEpoch,
    CreateAta,
    Stake,
    Claim,
    Airdrop
}

impl ToString for TxType {
    fn to_string(&self) -> String {
        match self {
            TxType::Mine => {
                "Mine".to_string()
            },
            TxType::Register => {
                "Register".to_string()
            },
            TxType::ResetEpoch => {
                "Reset".to_string()
            },
            TxType::CreateAta =>  {
                "Create Ata".to_string()
            },
            TxType::Stake =>  {
                "Stake".to_string()
            },
            TxType::Claim => {
                "Claim".to_string()
            },
            TxType::Airdrop => {
                "Airdrop".to_string()
            },
        }
    }
}

#[derive(Copy, Clone)]
pub struct HashStatus {
    pub hash_time: u64,
    pub hash_difficulty: u32,
}

#[derive(Component)]
pub struct TxProcessor {
    tx_type: TxType,
    status: String,
    error: String,
    sol_balance: f64,
    staked_balance: Option<u64>,
    challenge: String,
    signed_tx: Option<Transaction>,
    signature: Option<Signature>,
    hash_status: Option<HashStatus>,
    created_at: Instant,
    send_and_confirm_interval: Timer,
}

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
    challenge: String,
    stake: u64,
    last_hash_at: i64,
    total_hashes: u64,
}

impl Default for ProofAccountResource {
    fn default() -> Self {
        Self {
            challenge: "loading...".to_string(),
            stake: Default::default(),
            last_hash_at: Default::default(),
            total_hashes: Default::default(),
        }
    }
}

#[derive(Resource)]
pub struct BussesResource {
    busses: Vec<ore::state::Bus>,
    current_bus_id: usize,
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct TreasuryAccountResource {
    balance: String,
    admin: String,
    last_reset_at: i64,
    need_epoch_reset: bool,
    base_reward_rate: f64,
}

impl Default for TreasuryAccountResource {
    fn default() -> Self {
        Self {
            balance: "loading...".to_string(),
            admin: "loading...".to_string(),
            last_reset_at: 0,
            need_epoch_reset: false,
            base_reward_rate: 0.0,
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

#[derive(Debug)]
pub enum AccountUpdatesData {
    ProofData(Proof),
    BusData(Bus),
    TreasuryConfigData(ore::state::Config)
}

#[derive(Resource)]
pub struct AccountUpdatesChannel {
    pub channel: Receiver<AccountUpdatesData>
}

#[derive(Clone, PartialEq, Debug)]
pub struct TxStatus {
    pub status: String,
    pub error: String,
}

#[derive(Resource)]
pub struct OreAppState {
    config: AppConfig,
    active_input_node: Option<Entity>,
}

pub struct LocalResetCooldown {
    reset_timer: Timer
}

impl Default for LocalResetCooldown {
    fn default() -> Self {
        Self { reset_timer: Timer::new(Duration::from_secs(5), TimerMode::Once) }
    }
}

pub fn mining_screen_hotkeys(
    key_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if key_input.just_pressed(KeyCode::KeyC) {
        next_state.set(GameState::ConfigSetup);
    }
}

pub fn trigger_rpc_calls_for_ui(
    time: Res<Time>,
    mut rpc_connection: ResMut<RpcConnection>,
    mut event_fetch_ui_rpc_data: EventWriter<EventFetchUiDataFromRpc>,
) {
    // rpc_connection.fetch_ui_data_timer.tick(time.delta());
    // if rpc_connection.fetch_ui_data_timer.just_finished() {
        event_fetch_ui_rpc_data.send(EventFetchUiDataFromRpc);
        rpc_connection.fetch_ui_data_timer.reset();
    // }
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
    captured_text_query: Query<(Entity, &Children), With<ButtonCaptureTextInput>>,
    mut active_text_query: Query<(Entity, &mut TextInput), With<TextPasswordInput>>,
    mut event_writer: EventWriter<EventUnlock>,
) {
    if let Some(app_state_active_text_entity) = app_state.active_input_node {
        if kbd.just_pressed(KeyCode::Enter) {
            for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
                if captured_text_entity == app_state_active_text_entity {
                    for child in captured_text_children {
                        for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
                            if active_text_entity == *child {
                                event_writer.send(EventUnlock);
                            }
                        }
                    }
                }
            }
        }
        if kbd.just_pressed(KeyCode::Home) {
            for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
                if captured_text_entity == app_state_active_text_entity {
                    for child in captured_text_children {
                        for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
                            if active_text_entity == *child {
                                text_input.hidden = !text_input.hidden;
                            }
                        }
                    }
                }
            }
        }
        if kbd.just_pressed(KeyCode::Backspace) {
            for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
                if captured_text_entity == app_state_active_text_entity {
                    for child in captured_text_children {
                        for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
                            if active_text_entity == *child {
                                text_input.text.pop();
                                // reset, to ensure multiple presses aren't going to result in multiple backspaces
                                backspace_timer.timer.reset();
                            }
                        }
                    }
                }
            }
        } else if kbd.pressed(KeyCode::Backspace) {
            for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
                if captured_text_entity == app_state_active_text_entity {
                    for child in captured_text_children {
                        for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
                            if active_text_entity == *child {
                                backspace_timer.timer.tick(time.delta());
                                if backspace_timer.timer.just_finished() {
                                    text_input.text.pop();
                                    backspace_timer.timer.reset();
                                }
                            }
                        }
                    }
                }
            }
        }
        for ev in evr_char.read() {
            let mut cs = ev.char.chars();

            let c = cs.next();
            if let Some(char) = c {
                if !char.is_control() {
                    for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
                        if captured_text_entity == app_state_active_text_entity {
                            for child in captured_text_children {
                                for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
                                    if active_text_entity == *child {
                                        text_input.text.push_str(ev.char.as_str());
                                    }
                                }
                            }
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
    mouse_input: Res<ButtonInput<MouseButton>>,
    app_state: Res<OreAppState>,
    mut backspace_timer: Local<BackspaceTimer>,
    time: Res<Time>,
    captured_text_query: Query<(Entity, &Children), With<ButtonCaptureTextInput>>,
    mut active_text_query: Query<
        (Entity, &mut TextInput),
        Without<TextPasswordInput>,
    >,
) {
    if let Some(app_state_active_text_entity) = app_state.active_input_node {
        if kbd.just_pressed(KeyCode::Enter) {
            // TODO: give TextInput some event for enter key
        }
        if mouse_input.just_pressed(MouseButton::Right) {
            if let Ok(mut ctx) = ClipboardContext::new() {
                if let Ok(text) = ctx.get_contents() {
                    for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
                        if captured_text_entity == app_state_active_text_entity {
                            for child in captured_text_children {
                                for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
                                    if active_text_entity == *child {
                                        text_input.text = text.clone();
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error!("Failed to paste clipboard contents.");
                }
            } else {
                error!("Failed to create clipboard context.");
            }

        }
        if kbd.just_pressed(KeyCode::Backspace) {
            for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
                if captured_text_entity == app_state_active_text_entity {
                    for child in captured_text_children {
                        for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
                            if active_text_entity == *child {
                                text_input.text.pop();
                                // reset, to ensure multiple presses aren't going to result in multiple backspaces
                                backspace_timer.timer.reset();
                            }
                        }
                    }
                }
            }
        } else if kbd.pressed(KeyCode::Backspace) {
            for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
                if captured_text_entity == app_state_active_text_entity {
                    for child in captured_text_children {
                        for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
                            if active_text_entity == *child {
                                backspace_timer.timer.tick(time.delta());
                                if backspace_timer.timer.just_finished() {
                                    text_input.text.pop();
                                    backspace_timer.timer.reset();
                                }
                            }
                        }
                    }
                }
            }
        }
        for ev in evr_char.read() {
            let mut cs = ev.char.chars();
            let c = cs.next();
            if let Some(char) = c {
                if !char.is_control() {
                    for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
                        if captured_text_entity == app_state_active_text_entity {
                            for child in captured_text_children {
                                for (active_text_entity, mut text_input) in active_text_query.iter_mut() {
                                    if active_text_entity == *child {
                                        if text_input.numbers_only {
                                            if char.is_numeric() {
                                                text_input.text.push_str(ev.char.as_str());
                                            }
                                        } else {
                                            text_input.text.push_str(ev.char.as_str());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn file_drop(
    mut dnd_evr: EventReader<FileDragAndDrop>,
    mut event_writer: EventWriter<EventLoadKeypairFile>
) {
    for ev in dnd_evr.read() {
        println!("{:?}", ev);
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = ev {
            println!("Dropped file with path: {:?}", path_buf);

            event_writer.send(EventLoadKeypairFile(path_buf.to_path_buf()));
        }
    }
}

pub fn tx_processors_send(
    mut commands: Commands,
    mut query_tx: Query<(Entity, &mut TxProcessor)>,
    rpc_connection: Res<RpcConnection>,
    time: Res<Time>
) {
    for (entity, mut tx_processor) in query_tx.iter_mut() {
        if tx_processor.status.as_str() != "SUCCESS" && tx_processor.status.as_str() != "FAILED" {
            let mut just_finished = false;
            {
                let timer = &mut tx_processor.send_and_confirm_interval;
                timer.tick(time.delta());
                if timer.just_finished() {
                    just_finished = true;
                    timer.reset();
                }
            }

            if just_finished {
                if let Some(signed_tx) = &tx_processor.signed_tx {
                    let task_pool = IoTaskPool::get();
                    let client = rpc_connection.rpc.clone();
                    let tx = signed_tx.clone();
                    let task = task_pool.spawn(Compat::new(async move {
                        let send_cfg = RpcSendTransactionConfig {
                            skip_preflight: true,
                            preflight_commitment: Some(CommitmentLevel::Confirmed),
                            encoding: Some(UiTransactionEncoding::Base64),
                            max_retries: Some(0),
                            min_context_slot: None,
                        };

                        let sig = client.send_transaction_with_config(&tx, send_cfg).await;
                        if let Ok(sig) = sig {
                            return Ok(sig);
                        } else {
                            error!("Failed to send initial transaction...");
                            return Err("Failed to send tx".to_string());
                        }
                    }));

                    commands
                        .entity(entity)
                        .insert(TaskSendTx { task });
                }
            }
        }
    }
}

pub struct SigChecksTimer {
    timer: Timer,
}

impl Default for SigChecksTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(1000), TimerMode::Once)
        }
    }
}

pub fn tx_processors_sigs_check(
    mut event_writer: EventWriter<EventCheckSigs>,
    mut sig_checks_timer: Local<SigChecksTimer>,
    time: Res<Time>
) {
    sig_checks_timer.timer.tick(time.delta());
    if sig_checks_timer.timer.just_finished() {
        event_writer.send(EventCheckSigs);
        sig_checks_timer.timer.reset();
    }
}

pub fn tx_processor_result_checks(
    mut commands: Commands,
    mut event_writer: EventWriter<EventTxResult>,
    proof_res: Res<ProofAccountResource>,
    query_tx: Query<(Entity, &TxProcessor)>,
) {
    for (entity, tx_processor) in query_tx.iter() {
        let status = tx_processor.status.clone();
        if status == "SUCCESS" || status == "FAILED" {
            let sig = if let Some(s) = tx_processor.signature {
                s.to_string()
            } else {
                "FAILED".to_string()
            };

            match tx_processor.tx_type {
                TxType::Mine =>  {
                    if status == "SUCCESS" {
                        let previous_staked_balance = tx_processor.staked_balance;
                        if let Some(previous_staked_balance) = previous_staked_balance {
                            let current_staked_balance = proof_res.stake;
                            if  tx_processor.challenge.as_str() != proof_res.challenge {
                                // let sol_diff = current_sol_balance - previous_sol_balance;
                                let staked_diff = current_staked_balance - previous_staked_balance;
                                let ore_conversion = staked_diff as f64 / 10f64.powf(ore::TOKEN_DECIMALS as f64);
                                let status = format!("{} +{} ORE.", status, ore_conversion.to_string());
                                
                                event_writer.send(EventTxResult {
                                    tx_type: tx_processor.tx_type.to_string(),
                                    sig,
                                    hash_status: tx_processor.hash_status,
                                    tx_time: tx_processor.created_at.elapsed().as_secs(),
                                    tx_status:  TxStatus {
                                        status,
                                        error: tx_processor.error.clone()
                                    }
                                });

                                commands.entity(entity).despawn_recursive();

                            }
                        } else {
                            event_writer.send(EventTxResult {
                                tx_type: tx_processor.tx_type.to_string(),
                                sig,
                                hash_status: tx_processor.hash_status,
                                tx_time: tx_processor.created_at.elapsed().as_secs(),
                                tx_status:  TxStatus {
                                    status,
                                    error: tx_processor.error.clone()
                                }
                            });

                            commands.entity(entity).despawn_recursive();
                        }
                    } else if status == "FAILED" {
                            event_writer.send(EventTxResult {
                                tx_type: tx_processor.tx_type.to_string(),
                                sig,
                                hash_status: tx_processor.hash_status,
                                tx_time: tx_processor.created_at.elapsed().as_secs(),
                                tx_status:  TxStatus {
                                    status,
                                    error: tx_processor.error.clone()
                                }
                            });

                            commands.entity(entity).despawn_recursive();
                    }
                }
                TxType::Airdrop => {
                    event_writer.send(EventTxResult {
                        tx_type: tx_processor.tx_type.to_string(),
                        sig: tx_processor.signature.unwrap().to_string(),
                        hash_status: tx_processor.hash_status,
                        tx_time: tx_processor.created_at.elapsed().as_secs(),
                        tx_status:  TxStatus {
                            status,
                            error: tx_processor.error.clone()
                        }
                    });

                    commands.entity(entity).despawn_recursive();
                },
                TxType::Register |
                TxType::ResetEpoch |
                TxType::Stake |
                TxType::Claim |
                TxType::CreateAta =>  {
                    event_writer.send(EventTxResult {
                        tx_type: tx_processor.tx_type.to_string(),
                        sig,
                        hash_status: tx_processor.hash_status,
                        tx_time: tx_processor.created_at.elapsed().as_secs(),
                        tx_status:  TxStatus {
                            status,
                            error: tx_processor.error.clone()
                        }
                    });

                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

pub fn spin_spinner_icons(
    mut query: Query<(&mut Transform, &Visibility), With<SpinnerIcon>>,
    mut winit_setting: ResMut<WinitSettings>,
    time: Res<Time>,
) {
    let mut is_visible = false;
    for (mut transform, visibility) in query.iter_mut() {
        if visibility == Visibility::Visible  || visibility == Visibility::Inherited {

            is_visible = true;
            let rotation_rate = 6.0;

            let scaled_rotation = rotation_rate * time.delta().as_secs_f32();
            transform.rotate_z(scaled_rotation);
        }
    }

    let current_focused_mode = winit_setting.focused_mode;
    if is_visible {
        match &current_focused_mode {
            UpdateMode::Continuous => {},
            UpdateMode::Reactive { wait } => {
                if *wait != FAST_DURATION {
                    winit_setting.focused_mode = UpdateMode::Reactive {
                        wait: FAST_DURATION 
                    };
                    winit_setting.unfocused_mode = UpdateMode::Reactive {
                        wait: FAST_DURATION
                    };
                }
            },
            UpdateMode::ReactiveLowPower { wait } => {
                if *wait != FAST_DURATION {
                    winit_setting.focused_mode = UpdateMode::ReactiveLowPower { wait: FAST_DURATION };
                    winit_setting.unfocused_mode = UpdateMode::ReactiveLowPower { wait: FAST_DURATION};
                }
            }
        }
    } else {
        match &current_focused_mode {
            UpdateMode::Continuous => {},
            UpdateMode::Reactive { wait } => {
                if *wait != REGULAR_DURATION {
                    winit_setting.focused_mode = UpdateMode::Reactive { wait: REGULAR_DURATION };
                    winit_setting.unfocused_mode = UpdateMode::Reactive { wait: REGULAR_DURATION };
                }
            },
            UpdateMode::ReactiveLowPower { wait } => {
                if *wait != REGULAR_DURATION {
                    winit_setting.focused_mode = UpdateMode::ReactiveLowPower { wait: REGULAR_DURATION };
                    winit_setting.unfocused_mode = UpdateMode::ReactiveLowPower { wait: REGULAR_DURATION };
                }
            }
        }
    }
}

pub fn read_accounts_update_channel(
    account_update_channel: ResMut<AccountUpdatesChannel>,
    mut proof_account: ResMut<ProofAccountResource>,
    mut treasury_account: ResMut<TreasuryAccountResource>,
    mut busses_res: ResMut<BussesResource>,
) {
    let receiver = account_update_channel.channel.clone();

    while let Ok(data) = receiver.try_recv() {
        match data {
            AccountUpdatesData::BusData(new_bus_data) => {
                for bus in &mut busses_res.busses {
                    if bus.id == new_bus_data.id {
                        *bus = new_bus_data;
                    }
                }
            },
            AccountUpdatesData::ProofData(new_proof_data) => {
                let new_proof = ProofAccountResource {
                    challenge: KeccakHash::new_from_array(new_proof_data.challenge).to_string(),
                    stake: new_proof_data.balance,
                    last_hash_at: new_proof_data.last_hash_at,
                    total_hashes: new_proof_data.total_hashes,
                };

                *proof_account = new_proof;
            },
            AccountUpdatesData::TreasuryConfigData(new_treasury_data) => {
                treasury_account.last_reset_at = new_treasury_data.last_reset_at;
                let base_reward_rate =
                    (new_treasury_data.base_reward_rate as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
                treasury_account.base_reward_rate = base_reward_rate;

            },
        }
    }
}

