use std::{
    fs::{self, File}, path::Path, str::FromStr, sync::Arc
};

use bevy::{
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cocoon::Cocoon;
use copypasta::{ClipboardContext, ClipboardProvider};
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{Keypair, Signer}
};
use spl_associated_token_account::get_associated_token_address;

#[derive(Deserialize)]
pub struct Config {
    pub rpc_url: String,
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

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(AppWallet {
            wallet,
            sol_balance: 0.0,
            ore_balance: 0.0,
        })
        .insert_resource(RpcConnection {
            rpc: rpc_connection,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, button_update_sol_balance)
        .add_systems(Update, button_copy_text)
        .add_systems(Update, task_update_app_wallet_sol_balance)
        .add_systems(Update, update_app_wallet_ui)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Resources
#[derive(Resource)]
pub struct AppWallet {
    wallet: Keypair,
    sol_balance: f64,
    ore_balance: f64,
}

#[derive(Resource)]
pub struct RpcConnection {
    // Cannot use the nonblocking client and await with the bevy tasks because bevy doesn't actually use tokio for async tasks.
    rpc: Arc<RpcClient>,
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
pub struct ButtonUpdateSolOreBalances;

#[derive(Component)]
pub struct ButtonCopyText;

// Task Components
// TODO: tasks should return results so errors can be dealt with by the task handler system
struct TaskUpdateAppWalletSolBalanceData {
    pub sol_balance: f64,
    pub ore_balance: f64,
}
#[derive(Component)]
struct TaskUpdateAppWalletSolBalance {
    pub task: Task<TaskUpdateAppWalletSolBalanceData>,
}

fn task_update_app_wallet_sol_balance(
    mut commands: Commands,
    mut app_wallet: ResMut<AppWallet>,
    mut query: Query<(Entity, &mut TaskUpdateAppWalletSolBalance)>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            app_wallet.sol_balance = result.sol_balance;
            app_wallet.ore_balance = result.ore_balance;
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
                let task = pool.spawn(async move {
                    let balance = connection.get_balance(&pubkey).unwrap();
                    let sol_balance = balance as f64 / LAMPORTS_PER_SOL as f64;
                    let ore_mint = Pubkey::from_str("oreoN2tQbHXVaZsr3pf66A48miqcBXCDJozganhEJgz").unwrap();
                    let token_account = get_associated_token_address(&pubkey, &ore_mint);
                    
                    let ore_balance = connection.get_token_account_balance(&token_account).unwrap().ui_amount.unwrap();

                    TaskUpdateAppWalletSolBalanceData {
                        sol_balance,
                        ore_balance
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
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
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
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("Screen Node"),
        ))
        .with_children(|parent| {
            spawn_copyable_text(parent, &asset_server, app_wallet.wallet.pubkey().to_string(), wallet_str);
            parent.spawn((
                TextBundle::from_section(
                    &(sol_balance_str + " SOL"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
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
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                TextWalletOreBalance,
                Name::new("TextWalletOreBalance"),
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(300.0),
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
                    ButtonUpdateSolOreBalances,
                    Name::new("ButtonUpdateSolOreBalances"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Update Sol and Ore balances",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn update_app_wallet_ui(
    app_wallet: Res<AppWallet>,
    mut sol_balance_text_query: Query<&mut Text, With<TextWalletSolBalance>>,
    mut ore_balance_text_query: Query<&mut Text, (With<TextWalletOreBalance>, Without<TextWalletSolBalance>)>,
) {
    let mut text_sol_balance = sol_balance_text_query.single_mut();
    let mut text_ore_balance = ore_balance_text_query.single_mut();
    text_sol_balance.sections[0].value = app_wallet.sol_balance.to_string() + " SOL";
    text_ore_balance.sections[0].value = app_wallet.ore_balance.to_string() + " ORE";
}

fn spawn_copyable_text(parent: &mut ChildBuilder, asset_server: &AssetServer, copy_text: String, display_text: String) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(30.0),
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
                        font_size: 40.0,
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
                            width: Val::Px(60.0),
                            height: Val::Px(60.0),
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
                        "Click to Copy...",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
