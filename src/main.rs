use std::{
    fs::{self, File},
    path::Path,
    str::FromStr,
    sync::Arc,
};

use bevy::{
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};
use bevy_inspector_egui::{
    inspector_options::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions,
};
use cocoon::Cocoon;
use copypasta::{ClipboardContext, ClipboardProvider};
use ore::{
    state::{Proof, Treasury},
    utils::AccountDeserialize,
    MINT_ADDRESS, PROOF, TREASURY_ADDRESS,
};
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_associated_token_account::get_associated_token_address;

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
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(OreAppState { ore_mint })
        .insert_resource(AppWallet {
            wallet,
            sol_balance: 0.0,
            ore_balance: 0.0,
        })
        .init_resource::<ProofAccountResource>()
        .register_type::<ProofAccountResource>()
        .init_resource::<TreasuryAccountResource>()
        .register_type::<TreasuryAccountResource>()
        .insert_resource(RpcConnection {
            rpc: rpc_connection,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, button_update_sol_balance)
        .add_systems(Update, button_copy_text)
        .add_systems(Update, task_update_app_wallet_sol_balance)
        .add_systems(Update, update_app_wallet_ui)
        .add_systems(Update, update_proof_account_ui)
        .add_systems(Update, update_treasury_account_ui)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const FONT_SIZE: f32 = 16.0;

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

#[derive(Resource)]
pub struct RpcConnection {
    // Cannot use the nonblocking client and await with the bevy tasks because bevy doesn't actually use tokio for async tasks.
    rpc: Arc<RpcClient>,
}

// TODO: use real AppState for this
#[derive(Resource)]
pub struct OreAppState {
    ore_mint: Pubkey,
}

// Components
#[derive(Component)]
pub struct CopyableText {
    full_text: String,
}

#[derive(Component)]
pub struct TextWalletPubkey;

#[derive(Component)]
pub struct TextWalletSolBalance;

#[derive(Component)]
pub struct TextWalletOreBalance;

#[derive(Component)]
pub struct TextCurrentHash;

#[derive(Component)]
pub struct TextTotalHashes;

#[derive(Component)]
pub struct TextTotalRewards;

#[derive(Component)]
pub struct TextClaimableRewards;

#[derive(Component)]
pub struct TextTreasuryBalance;
#[derive(Component)]
pub struct TextTreasuryAdmin;

#[derive(Component)]
pub struct TextTreasuryDifficulty;

#[derive(Component)]
pub struct TextTreasuryLastResetAt;

#[derive(Component)]
pub struct TextTreasuryRewardRate;

#[derive(Component)]
pub struct TextTreasuryTotalClaimedRewards;

#[derive(Component)]
pub struct ButtonUpdateSolOreBalances;

#[derive(Component)]
pub struct ButtonCopyText;

// Task Components
// TODO: tasks should return results so errors can be dealt with by the task handler system
struct TaskUpdateAppWalletSolBalanceData {
    pub sol_balance: f64,
    pub ore_balance: f64,
    pub proof_account_data: ProofAccountResource,
    pub treasury_account_data: TreasuryAccountResource,
}
#[derive(Component)]
struct TaskUpdateAppWalletSolBalance {
    pub task: Task<TaskUpdateAppWalletSolBalanceData>,
}

fn task_update_app_wallet_sol_balance(
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

fn button_update_sol_balance(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonUpdateSolOreBalances>),
    >,
    app_wallet: Res<AppWallet>,
    ore_app_state: Res<OreAppState>,
    rpc_connection: ResMut<RpcConnection>,
) {
    for (entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                let pubkey = app_wallet.wallet.pubkey();

                let pool = AsyncComputeTaskPool::get();

                let connection = rpc_connection.rpc.clone();
                let ore_mint = ore_app_state.ore_mint.clone();
                let task = pool.spawn(async move {
                    let balance = connection.get_balance(&pubkey).unwrap();
                    let sol_balance = balance as f64 / LAMPORTS_PER_SOL as f64;
                    let token_account = get_associated_token_address(&pubkey, &ore_mint);

                    let ore_balance = connection
                        .get_token_account_balance(&token_account)
                        .unwrap()
                        .ui_amount
                        .unwrap();

                    let proof_account = get_proof(&connection, pubkey);
                    let proof_account_res_data;
                    if let Ok(proof_account) = proof_account {
                        proof_account_res_data = ProofAccountResource {
                            current_hash: proof_account.hash.to_string(),
                            total_hashes: proof_account.total_hashes,
                            total_rewards: proof_account.total_rewards,
                            claimable_rewards: proof_account.claimable_rewards,
                        };
                    } else {
                        proof_account_res_data = ProofAccountResource {
                            current_hash: "Not Found".to_string(),
                            total_hashes: 0,
                            total_rewards: 0,
                            claimable_rewards: 0,
                        };
                    }

                    let treasury_ore_balance = connection
                        .get_token_account_balance(&treasury_tokens_pubkey())
                        .unwrap()
                        .ui_amount
                        .unwrap();

                    let treasury_account = get_treasury(&connection);
                    let treasury_account_res_data;
                    if let Ok(treasury_account) = treasury_account {
                        let reward_rate = (treasury_account.reward_rate as f64)
                            / 10f64.powf(ore::TOKEN_DECIMALS as f64);
                        let total_claimed_rewards = (treasury_account.total_claimed_rewards as f64)
                            / 10f64.powf(ore::TOKEN_DECIMALS as f64);

                        treasury_account_res_data = TreasuryAccountResource {
                            balance: treasury_ore_balance.to_string(),
                            admin: treasury_account.admin.to_string(),
                            difficulty: treasury_account.difficulty.to_string(),
                            last_reset_at: treasury_account.last_reset_at,
                            reward_rate,
                            total_claimed_rewards,
                        };
                    } else {
                        treasury_account_res_data = TreasuryAccountResource {
                            balance: "Not Found".to_string(),
                            admin: "".to_string(),
                            difficulty: "".to_string(),
                            last_reset_at: 0,
                            reward_rate: 0.0,
                            total_claimed_rewards: 0.0,
                        };
                    }

                    TaskUpdateAppWalletSolBalanceData {
                        sol_balance,
                        ore_balance,
                        proof_account_data: proof_account_res_data,
                        treasury_account_data: treasury_account_res_data,
                    }
                });

                commands
                    .entity(entity)
                    .insert(TaskUpdateAppWalletSolBalance { task });
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn button_copy_text(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonCopyText>),
    >,
    text_query: Query<(&CopyableText, &Children)>,
) {
    for (entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                let mut text: Option<String> = None;
                for (copyable_text, children) in text_query.iter() {
                    for child in children.iter() {
                        if *child == entity {
                            text = Some(copyable_text.full_text.clone());
                        }
                    }
                }
                if let Some(text) = text {
                    let mut ctx = ClipboardContext::new().unwrap();
                    if let Err(_) = ctx.set_contents(text) {
                        info!("Failed to set clipboard content.");
                    } else {
                        info!("Succesfully copied to clipboard");
                    }
                } else {
                    info!("Failed to find copyable_text.");
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, app_wallet: Res<AppWallet>) {
    commands.spawn(Camera2dBundle::default());
    let full_addr = app_wallet.wallet.pubkey().to_string();
    let wallet_str;
    let len = full_addr.len();
    if len > 10 {
        let prefix = &full_addr[0..5];

        let suffix = &full_addr[len - 5..len];

        wallet_str = format!("{}...{}", prefix, suffix);
    } else {
        wallet_str = full_addr;
    }
    let sol_balance_str = app_wallet.sol_balance.to_string();
    let ore_balance_str = app_wallet.ore_balance.to_string();
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("Screen Node"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(50.0),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Top Half"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(50.0),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Top Half Left"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(350.0),
                                            height: Val::Px(150.0),
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("TreasuryAccountNode"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Treasury",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTitleTreasury"),
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Balance: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryBalance"),
                                        TextTreasuryBalance,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Admin: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryAdmin"),
                                        TextTreasuryAdmin,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Difficulty: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryDifficulty"),
                                        TextTreasuryDifficulty,
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Last Reset At: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryLastResetAt"),
                                        TextTreasuryLastResetAt,
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Reward Rate: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryRewardRate"),
                                        TextTreasuryRewardRate,
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Total Claimed Rewards: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryTotalClaimedRewards"),
                                        TextTreasuryTotalClaimedRewards,
                                    ));
                                });
                        });

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(50.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::End,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Top Half Right"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::End,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("AppWallet Node"),
                                ))
                                .with_children(|parent| {
                                    spawn_copyable_text(
                                        parent,
                                        &asset_server,
                                        app_wallet.wallet.pubkey().to_string(),
                                        wallet_str,
                                    );
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    flex_direction: FlexDirection::Column,
                                                    align_items: AlignItems::End,
                                                    padding: UiRect {
                                                        left: Val::Px(0.0),
                                                        right: Val::Px(20.0),
                                                        top: Val::Px(0.0),
                                                        bottom: Val::Px(0.0),
                                                    },
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("WalletBalance Nodes"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    &(sol_balance_str + " SOL"),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextWalletSolBalance,
                                                Name::new("TextWalletSolBalance"),
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    &(ore_balance_str + " ORE"),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextWalletOreBalance,
                                                Name::new("TextWalletOreBalance"),
                                            ));
                                        });
                                });

                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(200.0),
                                            height: Val::Px(110.0),
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect {
                                                top: Val::Px(100.0),
                                                left: Val::Px(0.0),
                                                right: Val::Px(160.0),
                                                bottom: Val::Px(0.0),
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("ProofAccountNode"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Proof Account",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTitleProofAccount"),
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "current hash: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextCurrentHash"),
                                        TextCurrentHash,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "total hashes: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTotalHashes"),
                                        TextTotalHashes,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "total rewards: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTotalRewards"),
                                        TextTotalRewards,
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            "claimable rewards: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextClaimableRewards"),
                                        TextClaimableRewards,
                                    ));
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(300.0),
                                            height: Val::Px(30.0),
                                            margin: UiRect {
                                                top: Val::Px(0.0),
                                                right: Val::Px(100.0),
                                                left: Val::Px(0.0),
                                                bottom: Val::Px(0.0),
                                            },
                                            border: UiRect::all(Val::Px(5.0)),
                                            // horizontally center child text
                                            justify_content: JustifyContent::Center,
                                            // vertically center child text
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        border_color: BorderColor(Color::BLACK),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    //ButtonUpdateSolOreBalances,
                                    Name::new("ButtonClaimOreRewards"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "CLAIM",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(300.0),
                                            height: Val::Px(30.0),
                                            margin: UiRect {
                                                top: Val::Px(0.0),
                                                right: Val::Px(100.0),
                                                left: Val::Px(0.0),
                                                bottom: Val::Px(0.0),
                                            },
                                            border: UiRect::all(Val::Px(5.0)),
                                            // horizontally center child text
                                            justify_content: JustifyContent::Center,
                                            // vertically center child text
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        border_color: BorderColor(Color::BLACK),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    ButtonUpdateSolOreBalances,
                                    Name::new("ButtonUpdateSolOreBalances"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Update Sol and Ore balances",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });
                        });

                    // ore logo (flex center)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexStart,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        z_index: ZIndex::Global(-1),
                                        style: Style {
                                            width: Val::Px(125.0),
                                            height: Val::Px(125.0),
                                            margin: UiRect::top(Val::VMin(5.)),
                                            ..default()
                                        },
                                        // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                                        background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    UiImage::new(asset_server.load("ore-icon.webp")),
                                ))
                                .with_children(|parent| {
                                    // alt text
                                    // This UI node takes up no space in the layout and the `Text` component is used by the accessibility module
                                    // and is not rendered.
                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                display: Display::None,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        },
                                        Text::from_section("Ore logo", TextStyle::default()),
                                    ));
                                });
                        });
                });
            parent.spawn((
                NodeBundle {
                    background_color: Color::GRAY.into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Bottom Half"),
            ));
        });
}

fn update_app_wallet_ui(
    app_wallet: Res<AppWallet>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextWalletSolBalance>>,
        Query<&mut Text, With<TextWalletOreBalance>>,
    )>,
) {
    let mut text_sol_balance_query = set.p0();
    let mut text_sol_balance = text_sol_balance_query.single_mut();
    text_sol_balance.sections[0].value = app_wallet.sol_balance.to_string() + " SOL";

    let mut text_ore_balance_query = set.p1();
    let mut text_ore_balance = text_ore_balance_query.single_mut();
    text_ore_balance.sections[0].value = app_wallet.ore_balance.to_string() + " ORE";
}

fn update_proof_account_ui(
    proof_account_res: Res<ProofAccountResource>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextCurrentHash>>,
        Query<&mut Text, With<TextTotalHashes>>,
        Query<&mut Text, With<TextTotalRewards>>,
        Query<&mut Text, With<TextClaimableRewards>>,
    )>,
) {
    let mut text_current_hash_query = set.p0();
    let mut text_current_hash = text_current_hash_query.single_mut();
    text_current_hash.sections[0].value =
        "Current Hash: ".to_string() + &proof_account_res.current_hash.clone();

    let mut text_total_hashes_query = set.p1();
    let mut text_total_hashes = text_total_hashes_query.single_mut();
    text_total_hashes.sections[0].value =
        "Total Hashes: ".to_string() + &proof_account_res.total_hashes.to_string();

    let mut text_total_rewards_query = set.p2();
    let mut text_total_rewards = text_total_rewards_query.single_mut();
    text_total_rewards.sections[0].value =
        "Total Rewards: ".to_string() + &proof_account_res.total_rewards.to_string();

    let mut text_claimable_rewards_query = set.p3();
    let mut text_claimable_rewards = text_claimable_rewards_query.single_mut();
    text_claimable_rewards.sections[0].value =
        "Claimable Rewards: ".to_string() + &proof_account_res.claimable_rewards.to_string();
}

fn update_treasury_account_ui(
    treasury_account_res: Res<TreasuryAccountResource>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextTreasuryBalance>>,
        Query<&mut Text, With<TextTreasuryAdmin>>,
        Query<&mut Text, With<TextTreasuryDifficulty>>,
        Query<&mut Text, With<TextTreasuryLastResetAt>>,
        Query<&mut Text, With<TextTreasuryRewardRate>>,
        Query<&mut Text, With<TextTreasuryTotalClaimedRewards>>,
    )>,
) {
    let mut text_query_0 = set.p0();
    let mut text_0 = text_query_0.single_mut();
    text_0.sections[0].value = "Balance: ".to_string() + &treasury_account_res.balance.clone();

    let mut text_query_1 = set.p1();
    let mut text_1 = text_query_1.single_mut();
    text_1.sections[0].value = "Admin: ".to_string() + &treasury_account_res.admin.clone();

    let mut text_query_2 = set.p2();
    let mut text_2 = text_query_2.single_mut();
    text_2.sections[0].value =
        "Difficulty: ".to_string() + &treasury_account_res.difficulty.clone();

    let mut text_query_3 = set.p3();
    let mut text_3 = text_query_3.single_mut();
    text_3.sections[0].value =
        "Last Reset At: ".to_string() + &treasury_account_res.last_reset_at.to_string();

    let mut text_query_4 = set.p4();
    let mut text_4 = text_query_4.single_mut();
    text_4.sections[0].value =
        "Reward Rate: ".to_string() + &treasury_account_res.reward_rate.to_string();

    let mut text_query_5 = set.p5();
    let mut text_5 = text_query_5.single_mut();
    text_5.sections[0].value = "Total Claimed Rewards: ".to_string()
        + &treasury_account_res.total_claimed_rewards.to_string();
}

fn spawn_copyable_text(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    copy_text: String,
    display_text: String,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        right: Val::Px(10.0),
                        bottom: Val::Px(0.0),
                    },
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            },
            CopyableText {
                full_text: copy_text.clone(),
            },
            Name::new("CopyableText"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    &display_text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                TextWalletPubkey,
                Name::new("WalletPubkeyText"),
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(30.0),
                            height: Val::Px(30.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonCopyText,
                    Name::new("ButtonCopyText"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Copy",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FONT_SIZE,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn get_treasury(client: &RpcClient) -> Result<Treasury, ()> {
    let data = client.get_account_data(&TREASURY_ADDRESS);
    if let Ok(data) = data {
        Ok(*Treasury::try_from_bytes(&data).expect("Failed to parse treasury account"))
    } else {
        Err(())
    }
}

pub fn get_proof(client: &RpcClient, authority: Pubkey) -> Result<Proof, String> {
    let proof_address = proof_pubkey(authority);
    let data = client.get_account_data(&proof_address);
    match data {
        Ok(data) => return Ok(*Proof::try_from_bytes(&data).unwrap()),
        Err(_) => return Err("Failed to get miner account".to_string()),
    }
}

pub fn proof_pubkey(authority: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[PROOF, authority.as_ref()], &ore::ID).0
}

pub fn treasury_tokens_pubkey() -> Pubkey {
    get_associated_token_address(&TREASURY_ADDRESS, &MINT_ADDRESS)
}
