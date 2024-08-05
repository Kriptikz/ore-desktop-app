use std::{
    fs, path::Path, sync::Arc, time::{Duration, Instant}
};

use async_compat::Compat;
use async_std::task::sleep;
use bevy::{prelude::*, tasks::{futures_lite::StreamExt, IoTaskPool}, utils::HashMap, winit::{UpdateMode, WinitSettings}};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions};
use copypasta::{ClipboardContext, ClipboardProvider};
use crossbeam_channel::{unbounded, Receiver, Sender};
use events::*;
use ore_api::{consts::TOKEN_DECIMALS, state::{Bus, Proof, Treasury}};
use ore_utils::{ORE_TOKEN_DECIMALS, AccountDeserialize};
use serde::{Deserialize, Serialize};
use solana_account_decoder::{parse_token::UiTokenAccount, UiAccountEncoding};
use solana_client::{nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient}, rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSendTransactionConfig}, rpc_filter::RpcFilterType, rpc_response::{Response, RpcKeyedAccount}};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel}, keccak::Hash as KeccakHash, program_pack::Pack, pubkey::Pubkey, signature::{Keypair, Signature}, signer::Signer, transaction::Transaction
};
use solana_transaction_status::UiTransactionEncoding;
use tasks::{
    handle_task_got_sig_checks, handle_task_process_tx_result, handle_task_send_tx_result, handle_task_tx_sig_check_results, task_generate_hash, task_register_wallet, task_update_app_wallet_sol_balance, TaskSendTx
};
use ui::{
    components::{AppScreenParent, BaseScreenNode, ButtonCaptureTextInput, DashboardScreenNode, MiningScreenNode, NavItem, NavItemArrow, NavItemIcon, NavItemText, NavItemWhiteSelectedBar, SpinnerIcon, TextInput, TextPasswordInput}, nav_item_systems::nav_item_interactions, screens::{screen_base::spawn_base_screen, screen_dashboard::{despawn_dashboard_screen, spawn_dashboard_screen}, screen_locked::{despawn_locked_screen, spawn_locked_screen}, screen_mining::{despawn_mining_screen, spawn_app_screen_mining}, screen_settings_config::{despawn_settings_config_screen, spawn_settings_config_screen}, screen_settings_general::{despawn_settings_general_screen, spawn_settings_general_screen}, screen_settings_wallet::{despawn_settings_wallet_screen, spawn_settings_wallet_screen}, screen_setup_wallet::{despawn_wallet_create_screen, spawn_wallet_setup_screen}}, ui_button_systems::{
        button_auto_scroll, button_capture_text, button_claim_ore_rewards, button_copy_text, button_generate_wallet, button_lock, button_open_web_tx_explorer, button_request_airdrop, button_save_config, button_save_wallet, button_stake_ore, button_start_stop_mining, button_unlock, tick_button_cooldowns
    }, ui_sync_systems::{
        fps_counter_showhide, fps_text_update_system, mouse_scroll, update_active_miners_ui, update_active_text_input_cursor_vis, update_app_wallet_ui, update_busses_ui, update_hash_rate_ui, update_miner_status_ui, update_proof_account_ui, update_text_input_ui, update_treasury_account_ui
    }
};


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
            rpc_url: "https://floral-dawn-pallet.solana-devnet.quiknode.pro/8b38be5427b44d3b42dc67c891dea71a56cd3a8c".to_string(),
            ws_url: "wss://floral-dawn-pallet.solana-devnet.quiknode.pro/8b38be5427b44d3b42dc67c891dea71a56cd3a8c".to_string(),
            is_devnet: true,
            threads: 1,
            ui_fetch_interval: 1000,
            tx_send_interval: 3000,
            tx_sigs_check_interval: 1000,
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum AppScreenState {
    WalletSetup,
    Unlock,
    Dashboard,
    Mining,
    SettingsConfig,
    SettingsWallet,
    SettingsGeneral,
}

#[derive(PartialEq)]
pub enum NavItemScreen {
    Dashboard,
    Mining,
    SettingsConfig,
    SettingsWallet,
    SettingsGeneral,
}

fn main() {
    let mut starting_state = AppScreenState::SettingsConfig;
    let config_path = Path::new("config.toml");
    let config: AppConfig = if config_path.exists() {
        let config_string = fs::read_to_string(config_path).unwrap();
        let config = match toml::from_str(&config_string) {
            Ok(d) => {
                starting_state = AppScreenState::WalletSetup;
                Some(d)
            }
            Err(_) => None,
        };
        config.unwrap_or(AppConfig::default())
    } else {
        AppConfig::default()
    };

    if starting_state == AppScreenState::WalletSetup {
        let wallet_path = Path::new("save.data");
        if wallet_path.exists() {
            starting_state = AppScreenState::Unlock;
        }
    }

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
            config: config.clone(),
            active_input_node: None,
        })
        .insert_resource(MinerStatusResource {
            miner_threads: threads,
            ..Default::default()
        })
        .insert_resource(RpcConnection {
            rpc: None,
            fetch_ui_data_timer: Timer::new(
                Duration::from_millis(config.ui_fetch_interval),
                TimerMode::Once,
            ),
        })
        .insert_resource(AppWallet {
            wallet: None,
            sol_balance: 0.0,
            ore_balance: 0.0,
        })
        .insert_resource(BussesResource {
            busses: vec![],
            current_bus_id: 0,
        })
        .insert_resource(HashrateResource {
            hashrate: 0.0,
        })
        .insert_resource(MiningProofsResource {
            proofs: HashMap::new(),
            largest_difficulty_seen: 0,
            miners_last_epoch: 0,
            miners_this_epoch: 0,
        })
        .insert_resource(MiningDataChannelResource {
            receiver: None,
            sender: None,
        })
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
        .add_event::<EventProofAccountUpdated>()
        .add_event::<EventCancelMining>()
        .add_systems(Startup, setup_base_screen)
        .add_systems(Update, fps_text_update_system)
        .add_systems(Update, fps_counter_showhide)
        .add_systems(Update, text_input)
        .add_systems(Update, update_text_input_ui)
        .add_systems(Update, button_capture_text)
        .add_systems(Update, update_active_text_input_cursor_vis)
        .add_systems(Update, tick_button_cooldowns)
        .add_systems(Update, nav_item_interactions)
        .add_systems(Update, update_app_wallet_ui)
        .add_systems(Update, mouse_scroll)
        .add_systems(Update, 
            (
                (
                    button_start_stop_mining,
                    spin_spinner_icons,
                    update_busses_ui,
                    update_treasury_account_ui,
                ),
                (
                    handle_event_start_stop_mining_clicked,
                    handle_event_submit_hash_tx,
                    handle_event_tx_result,
                    handle_event_fetch_ui_data_from_rpc,
                    handle_event_register_wallet,
                    handle_event_mine_for_hash,
                    handle_event_check_sigs,
                    handle_event_proof_account_updated,
                    handle_event_cancel_mining,
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
                    tx_processor_result_checks,
                    tx_processors_send,
                    tx_processors_sigs_check,
                    read_accounts_update_channel,
                )
            ).run_if(run_if_has_some_wallet)
        )
        .add_systems(OnEnter(AppScreenState::SettingsConfig), setup_settings_config_screen)
        .add_systems(
            OnExit(AppScreenState::SettingsConfig),
            (
                despawn_settings_config_screen,
            )
        )
        .add_systems(OnEnter(AppScreenState::SettingsGeneral), setup_settings_general_screen)
        .add_systems(
            OnExit(AppScreenState::SettingsGeneral),
            (
                despawn_settings_general_screen,
            )
        )
        .add_systems(OnEnter(AppScreenState::SettingsWallet), setup_settings_wallet_screen)
        .add_systems(
            OnExit(AppScreenState::SettingsWallet),
            (
                despawn_settings_wallet_screen,
            )
        )
        .add_systems(OnEnter(AppScreenState::WalletSetup), setup_wallet_create_screen)
        .add_systems(
            OnExit(AppScreenState::WalletSetup),
            (
                despawn_wallet_create_screen,
            )
        )
        // .add_systems(OnExit(GameState::WalletSetup), despawn_wallet_setup_screen)
        .add_systems(OnEnter(AppScreenState::Dashboard), setup_dashboard_screen)
        .add_systems(OnExit(AppScreenState::Dashboard), hide_dashboard_screen)
        .add_systems(OnEnter(AppScreenState::Unlock), setup_locked_screen)
        .add_systems(OnExit(AppScreenState::Unlock), despawn_locked_screen)
        .add_systems(OnEnter(AppScreenState::Mining), setup_mining_screen)
        .add_systems(OnExit(AppScreenState::Mining), hide_mining_screen)
        .add_systems(
            Update,
            (
                button_save_config,
                handle_event_save_config,
            )
                .run_if(in_state(AppScreenState::SettingsConfig)),
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
                .run_if(in_state(AppScreenState::WalletSetup)),
        )
        .add_systems(
            Update,
            (button_unlock, handle_event_unlock, text_password_input)
                .run_if(in_state(AppScreenState::Unlock)),
        )
        .add_systems(
            Update,
            (update_active_miners_ui)
                .run_if(in_state(AppScreenState::Dashboard)),
        )
        .add_systems(
            Update,
            (
                // individual tuple max size is 12
                (
                    button_lock,
                    button_copy_text,
                    button_claim_ore_rewards,
                    button_stake_ore,
                    button_auto_scroll,
                    button_open_web_tx_explorer,
                    button_request_airdrop
                ),
                (
                    handle_event_claim_ore_rewards,
                    handle_event_stake_ore,
                    handle_event_lock,
                    handle_event_request_airdrop,
                ),
                (
                    update_proof_account_ui,
                    update_miner_status_ui,
                    update_hash_rate_ui,
                ),
            )
                .run_if(is_mining_screen_with_some_wallet),
        )
        .run();
}

fn setup_base_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_state: Res<OreAppState>,
    mut event_writer: EventWriter<EventFetchUiDataFromRpc>,
) {
    // Spawn Camera
    commands.spawn(Camera2dBundle::default());

    // Spawn Task Entities
    commands.spawn((EntityTaskHandler, Name::new("EntityTaskHandler")));
    commands.spawn((EntityTaskFetchUiData, Name::new("EntityFetchUiData")));

    // Setup the base screen
    spawn_base_screen(commands.reborrow(), asset_server, "Locked".to_string(), 0.0, 0.0, app_state.config.clone());
}

fn setup_mining_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_state: Res<OreAppState>,
    app_wallet: Res<AppWallet>,
    mut rpc_connection: ResMut<RpcConnection>,
    query: Query<Entity, With<AppScreenParent>>,
    mut query_mining_screen: Query<(Entity, &mut Visibility), (With<MiningScreenNode>, Without<AppScreenParent>)>,
    mut event_writer: EventWriter<EventFetchUiDataFromRpc>,
    mut next_state: ResMut<NextState<AppScreenState>>,
    mut set: ParamSet<(
        Query<(&mut Visibility, &NavItemWhiteSelectedBar), Without<MiningScreenNode>>,
        Query<(&mut BackgroundColor, &NavItemIcon)>,
        Query<(&mut Text, &NavItemText)>,
        Query<(&mut BackgroundColor, &NavItemArrow)>,
    )>,
) {
    let base_screen_entity_id = query.get_single().unwrap();

    let config = &app_state.config;

    if let Ok((_mining_screen_ent, mut visibility)) = query_mining_screen.get_single_mut() {
        *visibility = Visibility::Visible;
    } else {

        if let Some(wallet) = &app_wallet.wallet {
            let mut parent = commands.get_entity(base_screen_entity_id).unwrap();
            parent.with_children(|parent| {
                spawn_app_screen_mining(parent, &asset_server);
            });
            if rpc_connection.rpc.is_none() {
                let new_rpc_connection = Arc::new(RpcClient::new_with_commitment(
                    config.rpc_url.clone(),
                    CommitmentConfig::confirmed(),
                ));
                rpc_connection.rpc = Some(new_rpc_connection);
                let task_pool = IoTaskPool::get();

                let (sender, receiver) = unbounded::<AccountUpdatesData>();

                let account_update_channel = AccountUpdatesChannel {
                    channel: receiver.clone(),
                };


                // TODO: use an entity here, we need to unsubscribe and cleanup this task when switching screens.
                commands.insert_resource(account_update_channel);

                let wallet_pubkey = wallet.pubkey().clone();

                info!("Wallet Pubkey: {}", wallet_pubkey);

                let ws_url = config.ws_url.clone();

                task_pool.spawn(Compat::new(async move {
                    let sender = sender.clone();

                    let mut ps_client = PubsubClient::new(&ws_url).await;
                    let mut attempts = 0;
                    
                    while ps_client.is_err() && attempts < 3 {
                        error!("Failed to connect to websocket, retrying...");
                        ps_client = PubsubClient::new(&ws_url).await;
                        sleep(Duration::from_millis(1000)).await;
                        attempts += 1;
                    }

                    if let Ok(ps_client) = ps_client {
                        let ps_client = Arc::new(ps_client);
                        let sender_c = sender.clone();
                        let ps_client_c = ps_client.clone();
                        task_pool.spawn(async move {
                            let ps_client = ps_client_c;
                            let sender = sender_c;
                            let account_pubkey = ore_api::ID;
                            let pubsub =
                                ps_client.program_subscribe(
                                    &account_pubkey,
                                    Some(RpcProgramAccountsConfig {
                                        //filters: Some(vec![RpcFilterType::DataSize(32)]),
                                        filters: None,
                                        account_config: RpcAccountInfoConfig {
                                            encoding: Some(UiAccountEncoding::Base64),
                                            data_slice: None,
                                            commitment: Some(CommitmentConfig::confirmed()),
                                            min_context_slot: None,
                                        },
                                        with_context: None,
                                    })
                                ).await;

                                if let Ok((mut account_sub_notifications, _account_unsub)) = pubsub {
                                    loop {
                                        if let Some(response) = account_sub_notifications.next().await {

                                            let data = response.value.account.data.decode();
                                            if let Some(data_bytes) = data {
                                                if let Ok(bus) = Bus::try_from_bytes(&data_bytes) {
                                                    let _ = sender.send(AccountUpdatesData::BusData(*bus));
                                                }
                                                if let Ok(ore_config) = ore_api::state::Config::try_from_bytes(&data_bytes) {
                                                    let _ = sender.send(AccountUpdatesData::TreasuryConfigData(*ore_config));
                                                }
                                                if let Ok(proof) = Proof::try_from_bytes(&data_bytes) {
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
                            let account_pubkey = ore_api::consts::TREASURY_TOKENS_ADDRESS;
                            let pubsub = ps_client.account_subscribe(
                                &account_pubkey,
                            Some(
                                    RpcAccountInfoConfig {
                                            encoding: Some(UiAccountEncoding::Base64),
                                            data_slice: None,
                                            commitment: Some(CommitmentConfig::confirmed()),
                                            min_context_slot: None,
                                    }
                                )).await;

                                if let Ok((mut account_sub_notifications, _account_unsub)) = pubsub {
                                    loop {
                                        if let Some(response) = account_sub_notifications.next().await {

                                            let data = response.value.data.decode();
                                            if let Some(data_bytes) = data {
                                                if let Ok(token_account) = spl_token::state::Account::unpack(&data_bytes) {
                                                    let _ = sender.send(AccountUpdatesData::TreasuryBalanceData(token_account.amount));
                                                }
                                            }
                                        }
                                    }
                                }
                            }).detach();
                    } else {
                        error!("Failed to connect to pubsub client via wss");
                    }
                })).detach();


                event_writer.send(EventFetchUiDataFromRpc);
            }
        } else {

            let wallet_path = Path::new("save.data");
            if wallet_path.exists() {
                next_state.set(AppScreenState::Unlock);
            } else {
                next_state.set(AppScreenState::WalletSetup);
            }
        }
    }

    // Update Nav Items Highlights
    for (mut visibility, nav_item_screen) in set.p0().iter_mut() {
        if nav_item_screen.0 == NavItemScreen::Mining {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    for (mut background_color, nav_item_screen) in set.p1().iter_mut() {
        if nav_item_screen.0 == NavItemScreen::Mining {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }
    for (mut text, nav_item_screen) in set.p2().iter_mut() {
        if nav_item_screen.0 == NavItemScreen::Mining {
            text.sections[0].style.color = Color::WHITE;
        } else {
            text.sections[0].style.color = Color::GRAY;
        }
    }
    for (mut background_color, nav_item_screen) in set.p3().iter_mut() {
        if nav_item_screen.0 == NavItemScreen::Mining {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }

}

fn hide_mining_screen(
    mut query: Query<(Entity, &mut Visibility), With<MiningScreenNode>>,
) {
    if let Ok((_screen_node, mut visibility)) = query.get_single_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn setup_dashboard_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<AppScreenParent>>,
    mut query_app_screen: Query<(Entity, &mut Visibility), With<DashboardScreenNode>>,
    mut set: ParamSet<(
        Query<(&mut Visibility, &NavItemWhiteSelectedBar), Without<DashboardScreenNode>>,
        Query<(&mut BackgroundColor, &NavItemIcon)>,
        Query<(&mut Text, &NavItemText)>,
        Query<(&mut BackgroundColor, &NavItemArrow)>,
    )>,
) {
    let base_screen_entity_id = query.get_single().unwrap();

    if let Ok((_mining_screen_ent, mut visibility)) = query_app_screen.get_single_mut() {
        *visibility = Visibility::Visible;
    } else {
        let mut parent = commands.get_entity(base_screen_entity_id).unwrap();

        parent.with_children(|parent| {
            spawn_dashboard_screen(parent, &asset_server);
        });
    }

    // Update Nav Items Highlights
    for (mut visibility, nav_item_screen) in set.p0().iter_mut() {
        if nav_item_screen.0 == NavItemScreen::Dashboard {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    for (mut background_color, nav_item_screen) in set.p1().iter_mut() {
        if nav_item_screen.0 == NavItemScreen::Dashboard {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }
    for (mut text, nav_item_screen) in set.p2().iter_mut() {
        if nav_item_screen.0 == NavItemScreen::Dashboard {
            text.sections[0].style.color = Color::WHITE;
        } else {
            text.sections[0].style.color = Color::GRAY;
        }
    }
    for (mut background_color, nav_item_screen) in set.p3().iter_mut() {
        if nav_item_screen.0 == NavItemScreen::Dashboard {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }
}

fn hide_dashboard_screen(
    mut query: Query<(Entity, &mut Visibility), With<DashboardScreenNode>>,
) {
    if let Ok((_screen_node, mut visibility)) = query.get_single_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn setup_settings_config_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_state: Res<OreAppState>,
    query: Query<Entity, With<AppScreenParent>>,
    mut set: ParamSet<(
        Query<(&mut Visibility, &NavItemWhiteSelectedBar)>,
        Query<(&mut BackgroundColor, &NavItemIcon)>,
        Query<(&mut Text, &NavItemText)>,
        Query<(&mut BackgroundColor, &NavItemArrow)>,
    )>,
) {
    let base_screen_entity_id = query.get_single().unwrap();

    let mut parent = commands.get_entity(base_screen_entity_id).unwrap();

    parent.with_children(|parent| {
        spawn_settings_config_screen(parent, asset_server, app_state.config.clone());
    });

    // Update Nav Items Highlights
    let this_nav_screen = NavItemScreen::SettingsConfig;
    for (mut visibility, nav_item_screen) in set.p0().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    for (mut background_color, nav_item_screen) in set.p1().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }
    for (mut text, nav_item_screen) in set.p2().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            text.sections[0].style.color = Color::WHITE;
        } else {
            text.sections[0].style.color = Color::GRAY;
        }
    }
    for (mut background_color, nav_item_screen) in set.p3().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }

}

fn setup_settings_general_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<AppScreenParent>>,
    mut set: ParamSet<(
        Query<(&mut Visibility, &NavItemWhiteSelectedBar)>,
        Query<(&mut BackgroundColor, &NavItemIcon)>,
        Query<(&mut Text, &NavItemText)>,
        Query<(&mut BackgroundColor, &NavItemArrow)>,
    )>,
) {
    let base_screen_entity_id = query.get_single().unwrap();

    let mut parent = commands.get_entity(base_screen_entity_id).unwrap();

    parent.with_children(|parent| {
        spawn_settings_general_screen(parent, asset_server);
    });

    let this_nav_screen = NavItemScreen::SettingsGeneral;
    for (mut visibility, nav_item_screen) in set.p0().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    for (mut background_color, nav_item_screen) in set.p1().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }
    for (mut text, nav_item_screen) in set.p2().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            text.sections[0].style.color = Color::WHITE;
        } else {
            text.sections[0].style.color = Color::GRAY;
        }
    }
    for (mut background_color, nav_item_screen) in set.p3().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }

}

fn setup_wallet_create_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<AppScreenParent>>,
) {
    let base_screen_entity_id = query.get_single().unwrap();

    let mut parent = commands.get_entity(base_screen_entity_id).unwrap();

    parent.with_children(|parent| {
        spawn_wallet_setup_screen(parent, asset_server);
    });
}

fn setup_settings_wallet_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<AppScreenParent>>,
    mut set: ParamSet<(
        Query<(&mut Visibility, &NavItemWhiteSelectedBar)>,
        Query<(&mut BackgroundColor, &NavItemIcon)>,
        Query<(&mut Text, &NavItemText)>,
        Query<(&mut BackgroundColor, &NavItemArrow)>,
    )>,
) {
    let base_screen_entity_id = query.get_single().unwrap();

    let mut parent = commands.get_entity(base_screen_entity_id).unwrap();

    parent.with_children(|parent| {
        spawn_settings_wallet_screen(parent, asset_server);
    });

    let this_nav_screen = NavItemScreen::SettingsWallet;
    for (mut visibility, nav_item_screen) in set.p0().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    for (mut background_color, nav_item_screen) in set.p1().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }
    for (mut text, nav_item_screen) in set.p2().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            text.sections[0].style.color = Color::WHITE;
        } else {
            text.sections[0].style.color = Color::GRAY;
        }
    }
    for (mut background_color, nav_item_screen) in set.p3().iter_mut() {
        if nav_item_screen.0 == this_nav_screen {
            *background_color = Color::WHITE.into();
        } else {
            *background_color = Color::GRAY.into();
        }
    }

}

fn setup_locked_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_state: Res<OreAppState>,
    mut event_writer: EventWriter<EventFetchUiDataFromRpc>,
    query: Query<Entity, With<AppScreenParent>>,
) {

    let base_screen_entity_id = query.get_single().unwrap();

    let mut parent = commands.get_entity(base_screen_entity_id).unwrap();

    parent.with_children(|parent| {
        spawn_locked_screen(parent, asset_server);
    });
}

fn is_mining_screen_with_some_wallet(
    app_wallet: Res<AppWallet>,
    app_screen_state: Res<State<AppScreenState>>,
) -> bool {
    *app_screen_state == AppScreenState::Mining && app_wallet.wallet.is_some()
}

fn run_if_has_some_wallet(
    app_wallet: Res<AppWallet>,
) -> bool {
    app_wallet.wallet.is_some()
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
    wallet: Option<Arc<Keypair>>,
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
pub struct HashrateResource {
    hashrate: f64,
}

#[derive(Resource)]
pub struct BussesResource {
    busses: Vec<ore_api::state::Bus>,
    current_bus_id: usize,
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct TreasuryAccountResource {
    balance: String,
    last_reset_at: i64,
    need_epoch_reset: bool,
    base_reward_rate: f64,
    min_difficulty: u64,
}

impl Default for TreasuryAccountResource {
    fn default() -> Self {
        Self {
            balance: "loading...".to_string(),
            last_reset_at: 0,
            need_epoch_reset: false,
            base_reward_rate: 0.0,
            min_difficulty: 0,
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
    rpc: Option<Arc<RpcClient>>,
    pub fetch_ui_data_timer: Timer,
}

#[derive(Resource)]
pub struct MiningProofsResource {
    proofs: HashMap<Pubkey, Proof>,
    largest_difficulty_seen: u32,
    miners_this_epoch: u32,
    miners_last_epoch: u32,
}


#[derive(Debug)]
pub enum MiningDataChannelMessage {
    Stop,
}

#[derive(Resource)]
pub struct MiningDataChannelResource {
    pub receiver: Option<Receiver<MiningDataChannelMessage>>,
    pub sender: Option<Sender<MiningDataChannelMessage>>
}

#[derive(Debug)]
pub enum AccountUpdatesData {
    ProofData(Proof),
    BusData(Bus),
    TreasuryConfigData(ore_api::state::Config),
    TreasuryBalanceData(u64)
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

// pub fn mining_screen_hotkeys(
//     key_input: Res<ButtonInput<KeyCode>>,
//     mut next_state: ResMut<NextState<GameState>>,
// ) {
//     if key_input.just_pressed(KeyCode::KeyC) {
//         next_state.set(GameState::ConfigSetup);
//     }
// }

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
        let client = if let Some(rpc) = &rpc_connection.rpc {
            rpc.clone()
        } else {
            error!("cannot process tx, rpc_connection.rpc is None");
            continue;
        };
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
                                let ore_conversion = staked_diff as f64 / 10f64.powf(ORE_TOKEN_DECIMALS as f64);
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
    mut mining_proofs_res: ResMut<MiningProofsResource>,
    app_wallet: Res<AppWallet>,
    mut event_proof_account_updated: EventWriter<EventProofAccountUpdated>
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
            AccountUpdatesData::ProofData(proof) => {
                if let Some(wallet) = &app_wallet.wallet {
                    if proof.authority == wallet.pubkey() {
                        let new_proof = ProofAccountResource {
                            challenge: KeccakHash::new_from_array(proof.challenge).to_string(),
                            stake: proof.balance,
                            last_hash_at: proof.last_hash_at,
                            total_hashes: proof.total_hashes,
                        };

                        *proof_account = new_proof;
                    }
                }

                event_proof_account_updated.send(EventProofAccountUpdated(proof));

            },
            AccountUpdatesData::TreasuryConfigData(new_treasury_data) => {
                if treasury_account.last_reset_at != new_treasury_data.last_reset_at {
                    // last_reset_at updated
                    mining_proofs_res.miners_last_epoch = mining_proofs_res.miners_this_epoch;
                    mining_proofs_res.miners_this_epoch = 0;

                    info!("miners last epoch: {}", mining_proofs_res.miners_last_epoch);
                }
                treasury_account.last_reset_at = new_treasury_data.last_reset_at;
                let base_reward_rate =
                    (new_treasury_data.base_reward_rate as f64) / 10f64.powf(ORE_TOKEN_DECIMALS as f64);
                treasury_account.base_reward_rate = base_reward_rate;

            },
            AccountUpdatesData::TreasuryBalanceData(new_balance) => {
                let new_balance = (new_balance as f64) / 10f64.powf(ORE_TOKEN_DECIMALS as f64);
                treasury_account.balance = new_balance.to_string();
            }
        }
    }
}

