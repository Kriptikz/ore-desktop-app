use std::{
    fs::{self, File},
    path::Path,
    str::FromStr,
    sync::Arc,
};

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    tasks::{block_on, futures_lite::future, Task},
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
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_associated_token_account::get_associated_token_address;
use ui::{layout::spawn_ui, systems::*};

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
        .insert_resource(OreAppState { ore_mint })
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
        .add_systems(Startup, setup)
        .add_systems(Update, button_update_sol_balance)
        .add_systems(Update, button_copy_text)
        .add_systems(Update, button_start_stop_mining)
        .add_systems(Update, task_update_app_wallet_sol_balance)
        .add_systems(Update, update_app_wallet_ui)
        .add_systems(Update, update_proof_account_ui)
        .add_systems(Update, update_treasury_account_ui)
        .add_systems(Update, update_miner_status_ui)
        .add_systems(Update, mouse_scroll)
        .run();
}

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

// TODO: use real AppState for this
#[derive(Resource)]
pub struct OreAppState {
    ore_mint: Pubkey,
}


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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, app_wallet: Res<AppWallet>) {
    commands.spawn(Camera2dBundle::default());
    spawn_ui(commands, asset_server, app_wallet);
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
