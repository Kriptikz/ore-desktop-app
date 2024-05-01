use std::{fs::{self, File}, path::Path, sync::Arc};

use bevy::{prelude::*, tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task}};
use cocoon::Cocoon;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::{Keypair, Signer}};

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
    
    let rpc_connection = Arc::new(RpcClient::new_with_commitment(config.rpc_url, CommitmentConfig::confirmed()));
    
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
        .insert_resource(AppWallet {
            wallet,
        })
        .insert_resource(RpcConnection {
            rpc: rpc_connection,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, button_log_balance_system)
        .add_systems(Update, task_system_log_sol_balance)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Resources
#[derive(Resource)]
pub struct AppWallet {
    wallet: Keypair,
}

#[derive(Resource)]
pub struct RpcConnection {
    // Cannot use the nonblocking client and await with the bevy tasks because bevy doesn't actually use tokio for async tasks.
    rpc: Arc<RpcClient>,
}

// Components
#[derive(Component)]
pub struct WalletPubkeyText;

#[derive(Component)]
pub struct ButtonLogBalance;

// Task Components
// TODO: tasks should return results so errors can be dealt with by the task handler system
#[derive(Component)]
struct TaskLogSolBalance {
    pub task: Task<f64>,
}

fn task_system_log_sol_balance(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TaskLogSolBalance)>,
) {
    for (entity, mut task) in &mut query.iter_mut() {
        if let Some(result) = block_on(future::poll_once(&mut task.task)) {
            info!("BALANCE: {}", result);
            commands.entity(entity).remove::<TaskLogSolBalance>();
        }
    }
}

fn button_log_balance_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (
            Changed<Interaction>,
            With<ButtonLogBalance>,
        ),
    >,
    app_wallet: Res<AppWallet>,
    rpc_connection: ResMut<RpcConnection>
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
                    balance as f64 / LAMPORTS_PER_SOL as f64
                });

                commands.entity(entity).insert(TaskLogSolBalance { task });

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
    let wallet_str = app_wallet.wallet.pubkey().to_string();
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    &wallet_str,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                WalletPubkeyText,
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
                    ButtonLogBalance,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Log balance",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
