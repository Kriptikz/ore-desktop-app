use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, IoTaskPool},
};
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use chrono::DateTime;
use cocoon::Cocoon;
use drillx::{Solution};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    ore_utils::{
        find_hash_par, get_claim_ix, get_clock_account, get_cutoff, get_mine_ix, get_ore_epoch_duration, get_ore_mint, get_proof, get_proof_and_treasury_with_busses, get_register_ix, get_reset_ix, get_stake_ix, get_treasury, proof_pubkey, treasury_tokens_pubkey
    }, tasks::{
        TaskGenerateHash, TaskProcessTx, TaskRegisterWallet, TaskUpdateAppWalletSolBalance,
        TaskUpdateAppWalletSolBalanceData
    }, ui::{
        components::{ButtonAutoScroll, MovingScrollPanel, ScrollingList, TextGeneratedKeypair, TextInput, TextMnemonicLine1, TextMnemonicLine2, TextMnemonicLine3, TextPasswordInput, ToggleAutoMine},
        spawn_utils::{spawn_new_list_item, UiListItem}, styles::{TOGGLE_OFF, TOGGLE_ON},
    }, utils::{find_best_bus, get_unix_timestamp}, AppWallet, BussesResource, Config, EntityTaskFetchUiData, EntityTaskHandler, GameState, HashStatus, MinerStatusResource, OreAppState, ProofAccountResource, RpcConnection, TreasuryAccountResource, TxStatus
};

use std::{
    fs::File, io::{stdout, Write}, path::{Path, PathBuf}, str::FromStr, sync::{atomic::AtomicBool, Arc, Mutex}, time::Instant
};

use solana_sdk::{
    bs58, commitment_config::CommitmentLevel, compute_budget::ComputeBudgetInstruction, derivation_path::DerivationPath, keccak::{hashv, Hash as KeccakHash}, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, signer::SeedDerivable, transaction::Transaction
};

// Events
#[derive(Event)]
pub struct EventStartStopMining;

#[derive(Event)]
pub struct EventGenerateWallet;

#[derive(Event)]
pub struct EventLoadKeypairFile(pub PathBuf);

#[derive(Event)]
pub struct EventSaveWallet;

#[derive(Event)]
pub struct EventMineForHash;

#[derive(Event)]
pub struct EventStopMining;

#[derive(Event)]
pub struct EventSubmitHashTx(pub (Solution, u32, u64));

pub struct TxResult {
    pub sig: String,
    pub tx_time: u64,
    pub hash_time: u64,
    // TODO: create a TxStatus struct that will be able to show different colors based on status enums
    pub status: String,
}

#[derive(Event)]
pub struct EventTxResult {
    pub tx_type: String,
    pub sig: String,
    pub tx_time: u64,
    pub hash_status: Option<HashStatus>,
    pub tx_status: TxStatus,
}

#[derive(Event)]
pub struct EventFetchUiDataFromRpc;

#[derive(Event)]
pub struct EventRegisterWallet;

#[derive(Event)]
pub struct EventClaimOreRewards;

#[derive(Event)]
pub struct EventStakeOre;

#[derive(Event)]
pub struct EventProcessTx {
    pub tx_type: String,
    pub tx: Transaction,
    pub hash_status: Option<(u64, u32)>,
}

#[derive(Event)]
pub struct EventLock;

#[derive(Event)]
pub struct EventUnlock;

#[derive(Event)]
pub struct EventSaveConfig(pub Config);

pub fn handle_event_start_stop_mining_clicked(
    mut ev_start_stop_mining: EventReader<EventStartStopMining>,
    mut event_writer: EventWriter<EventMineForHash>,
    mut event_writer_register: EventWriter<EventRegisterWallet>,
    app_wallet: Res<AppWallet>,
    mut miner_status: ResMut<MinerStatusResource>,
    rpc_connection: Res<RpcConnection>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut UiImage, &mut ToggleAutoMine)>,
) {
    for _ev in ev_start_stop_mining.read() {
        match miner_status.miner_status.as_str() {
            "MINING" |
            "PROCESSING" => {
                // stop mining
                // event_writer_stop.send(EventStopMining);
                miner_status.miner_status = "STOPPED".to_string();
                let (mut btn, mut toggle) = query.single_mut();
                toggle.0 = false;
                *btn = UiImage::new(asset_server.load(TOGGLE_OFF));
            
            },
            "STOPPED" => {
                // start mining
                let client = rpc_connection.rpc.clone();
                let proof_address = proof_pubkey(app_wallet.wallet.pubkey());
                if client.get_account(&proof_address).is_ok() {
                    event_writer.send(EventMineForHash);
                    let (mut btn, mut toggle) = query.single_mut();
                    toggle.0 = true;
                    *btn = UiImage::new(asset_server.load(TOGGLE_ON));
                } else {
                    event_writer_register.send(EventRegisterWallet);
                }
            },
            _ => {
                error!("Invalid Miner Status in handle_event_start_stop_mining_clicked");
            }

        }
    }
}

pub fn handle_event_mine_for_hash(
    mut commands: Commands,
    mut event_reader: EventReader<EventMineForHash>,
    app_wallet: Res<AppWallet>,
    rpc_connection: ResMut<RpcConnection>,
    mut miner_status: ResMut<MinerStatusResource>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in event_reader.read() {
        if let Ok(task_handler_entity) = query_task_handler.get_single() {
            let pool = AsyncComputeTaskPool::get();
            let wallet = app_wallet.wallet.clone();
            let client = rpc_connection.rpc.clone();
            let threads = miner_status.miner_threads;
            let task = pool.spawn(async move {
                // TODO: use proof resource cached proof. May need LatestHash Resource to ensure a new proof if loaded before mining.
                //  get proof account data
                let proof = if let Ok(result) = get_proof(&client, wallet.pubkey()) {
                    result
                } else {
                    return Err("Failed to get proof account. Please Retry.".to_string());
                };

                // TODO: use treasury resource cached difficulty
                let treasury =
                    get_treasury(&client);
                let treasury = if let Ok(result) = get_treasury(&client) {
                    result
                } else {
                    return Err("Failed to get treasury account. Please Retry.".to_string());
                };

                // ensure proof account is hash is not the same as the last generated one.
                // which results in 0x3 - Hash already submitted. Stale RPC Data...

                let current_ts = get_unix_timestamp();

                let cutoff = proof
                                    .last_hash_at
                                    .saturating_add(60)
                                    .saturating_sub(2 as i64)
                                    .saturating_sub(current_ts as i64)
                                    .max(0) as u64;

                let hash_time = Instant::now();
                let (solution, best_difficulty, best_hash) = find_hash_par(
                    proof,
                    cutoff,
                    threads,
                );

                Ok((solution, best_difficulty, hash_time.elapsed().as_secs()))
            });
            miner_status.miner_status = "MINING".to_string();

            commands
                .entity(task_handler_entity)
                .insert(TaskGenerateHash { task });
        }
    }
}

pub struct CurrentBus {
    bus: usize
}

impl Default for CurrentBus {
    fn default() -> Self {
        Self { bus: 0 }
    }
}

pub fn handle_event_submit_hash_tx(
    mut commands: Commands,
    mut ev_submit_hash_tx: EventReader<EventSubmitHashTx>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
    app_wallet: Res<AppWallet>,
    treasury: Res<TreasuryAccountResource>,
    mut miner_status: ResMut<MinerStatusResource>,
    rpc_connection: Res<RpcConnection>,
    mut busses_res: ResMut<BussesResource>,
) {
    for ev in ev_submit_hash_tx.read() {
        if let Ok(task_handler_entity) = query_task_handler.get_single() {
            let pool = IoTaskPool::get();
            let wallet = app_wallet.wallet.clone();
            let client = rpc_connection.rpc.clone();

            let bus = find_best_bus(&busses_res.busses);

            busses_res.current_bus_id = bus;

            let solution;
            let difficulty;
            let hash_time;

            {
                let (s, d, ht) = &ev.0;
                solution = Solution::new(s.d, s.n);

                difficulty = *d;
                hash_time = *ht;
            }

            let last_reset_at = treasury.last_reset_at;

            let current_ts = get_unix_timestamp() as i64;

            let time_until_reset = (last_reset_at + 60) - current_ts;

            let task = pool.spawn(async move {
                let signer = wallet;

                let mut ixs = vec![];
                // TODO: set cu's
                let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(500000);

                ixs.push(cu_limit_ix);


                if time_until_reset <= 5 {
                    let reset_ix = get_reset_ix(signer.pubkey());
                    ixs.push(reset_ix);
                }

                
                // let cu_price_ix =
                //     ComputeBudgetInstruction::set_compute_unit_price(self.priority_fee);
                let ix_mine = get_mine_ix(signer.pubkey(), solution, bus);
                ixs.push(ix_mine);
                let latest_blockhash = client
                    .get_latest_blockhash_with_commitment(client.commitment());

                if let Ok((hash, _slot)) = latest_blockhash {
                    let mut tx = Transaction::new_with_payer(&ixs, Some(&signer.pubkey()));

                    tx.sign(&[&signer], hash);

                    return Some(("Mine".to_string(), tx, Some((hash_time, difficulty))));
                } else {
                    error!("Failed to get latest blockhash. handle_event_submit_hash_tx");
                    return None;
                    // error
                }
            });

            miner_status.miner_status = "PROCESSING".to_string();
            commands
                .entity(task_handler_entity)
                .insert(TaskProcessTx { task });
        } else {
            error!("Failed to get task entity. handle_event_submit_hash_tx");
        }
    }
}

pub fn handle_event_tx_result(
    mut commands: Commands,
    mut ev_tx_result: EventReader<EventTxResult>,
    mut event_writer: EventWriter<EventMineForHash>,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut ScrollingList, &mut Style, &Parent, &Node), With<MovingScrollPanel>>,
    query_node: Query<&Node>,
    query_auto_scroll: Query<&ButtonAutoScroll>,
    query_toggle: Query<&ToggleAutoMine>,
) {
    for ev in ev_tx_result.read() {
        let (hash_time, difficulty) = if let Some(ht) = &ev.hash_status {
            (ht.hash_time.to_string(), ht.hash_difficulty.to_string())
        } else {
            ("N/A".to_string(), "".to_string())
        };
        let (scroll_panel_entity, mut scrolling_list, mut style, parent, list_node) = query.get_single_mut().expect("There should only be 1 scroll panel entity.");
        let status = format!(
            "{}  {}",
            ev.tx_status.status.clone(),
            ev.tx_status.error.clone()
        );

        let ts = get_unix_timestamp();
        let date_time = if let Some(dt) = DateTime::from_timestamp(ts as i64, 0) {
            dt.to_string()
        } else {
            "Err".to_string()
        };

        let hash_time = format!("{} - {}", hash_time, difficulty);
        let item_data = UiListItem {
            id: ev.tx_type.clone(),
            landed_at: date_time.clone(),
            sig: ev.sig.clone(),
            tx_time: ev.tx_time.to_string(),
            hash_time,
            status,
        };
        spawn_new_list_item(&mut commands, &asset_server, scroll_panel_entity, item_data);

        let auto_scroll = query_auto_scroll.single();

        if auto_scroll.0 {
            let items_height = list_node.size().y + 20.0;
            if let Ok(query_node_parent) = query_node.get(parent.get()) {
                let container_height = query_node_parent.size().y;

                if items_height > container_height {
                    let max_scroll = items_height - container_height;

                    scrolling_list.position = -max_scroll;
                    style.top = Val::Px(scrolling_list.position);
                }
            }
        }

        let toggle = query_toggle.single();
        if toggle.0 {
            if ev.tx_type == "Mine" {
                event_writer.send(EventMineForHash);
            }
        }
    }
}

pub fn handle_event_fetch_ui_data_from_rpc(
    mut commands: Commands,
    app_wallet: Res<AppWallet>,
    rpc_connection: ResMut<RpcConnection>,
    mut event_reader: EventReader<EventFetchUiDataFromRpc>,
    query_task_handler: Query<Entity, With<EntityTaskFetchUiData>>,
) {
    for _ev in event_reader.read() {
        if let Ok(task_handler_entity) = query_task_handler.get_single() {
            let pubkey = app_wallet.wallet.pubkey();

            let pool = IoTaskPool::get();

            let connection = rpc_connection.rpc.clone();
            let ore_mint = get_ore_mint();
            let task = pool.spawn(async move {
                let balance = if let Ok(result) = connection.get_balance(&pubkey) {
                    result
                } else {
                    return Err("Failed to get balance. Please retry.".to_string());
                };
                let sol_balance = balance as f64 / LAMPORTS_PER_SOL as f64;
                let token_account = get_associated_token_address(&pubkey, &ore_mint);

                let ore_balance =
                    if let Ok(response) = connection.get_token_account_balance(&token_account) {
                        if let Some(amount) = response.ui_amount {
                            amount
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    };

                // TODO: condense as many solana accounts into one rpc get_multiple_accounts call as possible
                let (proof_account, treasury_account, treasury_config, busses) = get_proof_and_treasury_with_busses(&connection, pubkey);

                let proof_account_res_data;
                if let Ok(proof_account) = proof_account {
                    proof_account_res_data = ProofAccountResource {
                        challenge: KeccakHash::new_from_array(proof_account.challenge).to_string(),
                        stake: proof_account.balance,
                        last_hash_at: proof_account.last_hash_at,
                        total_hashes: proof_account.total_hashes,
                        last_claim_at: proof_account.last_claim_at,
                    };
                } else {
                    proof_account_res_data = ProofAccountResource {
                        challenge: "Not Found".to_string(),
                        stake: 0,
                        last_hash_at: 0,
                        total_hashes: 0,
                        last_claim_at: 0,
                    };
                }

                let treasury_ore_balance = if let Ok(token_balance) = connection.get_token_account_balance(&treasury_tokens_pubkey()) {
                    if let Some(ui_amount) = token_balance.ui_amount {
                        ui_amount
                    } else {
                        return Err("Failed to get ui_amount from token_account. Fetch Ui Data.".to_string());
                    }

                } else {
                    return Err("Failed to get token account balance. Fetch Ui Data.".to_string());
                };

                let treasury_account_res_data;
                if let Ok(treasury_account) = treasury_config {
                    let base_reward_rate =
                        (treasury_account.base_reward_rate as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);

                    let clock = if let Ok(clock) =  get_clock_account(&connection) {
                        clock
                    } else {
                        return Err("Failed to get clock account. fetch ui data.".to_string());
                    };
                    let threshold = treasury_account
                        .last_reset_at
                        .saturating_add(get_ore_epoch_duration());

                    let need_epoch_reset = if clock.unix_timestamp.ge(&threshold) {
                        true
                    } else {
                        false
                    };

                    treasury_account_res_data = TreasuryAccountResource {
                        balance: treasury_ore_balance.to_string(),
                        admin: treasury_account.admin.to_string(),
                        last_reset_at: treasury_account.last_reset_at,
                        need_epoch_reset,
                        base_reward_rate,
                    };
                } else {
                    treasury_account_res_data = TreasuryAccountResource {
                        balance: "Not Found".to_string(),
                        admin: "".to_string(),
                        last_reset_at: 0,
                        need_epoch_reset: false,
                        base_reward_rate: 0.0,
                    };
                }
                let mut busses_res_data = vec![];
                if let Ok(busses) = busses {
                    for bus in busses {
                        if let Ok(bus) = bus {
                            busses_res_data.push(bus);
                        } else {
                            error!("Got error result for bus.");
                        }
                    }
                }

                Ok(TaskUpdateAppWalletSolBalanceData {
                    sol_balance,
                    ore_balance,
                    proof_account_data: proof_account_res_data,
                    treasury_account_data: treasury_account_res_data,
                    busses: busses_res_data,
                })
            });

            commands
                .entity(task_handler_entity)
                .insert(TaskUpdateAppWalletSolBalance { task });
        } else {
            error!("Failed to get task_handler_entity. handle_event_fetch_ui_data_from_rpc");
        }
    }
}

pub fn handle_event_register_wallet(
    mut commands: Commands,
    mut event_reader: EventReader<EventRegisterWallet>,
    app_wallet: Res<AppWallet>,
    rpc_connection: ResMut<RpcConnection>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in event_reader.read() {
        if let Ok(task_handler_entity) = query_task_handler.get_single() {
            let pool = IoTaskPool::get();
            let wallet = app_wallet.wallet.clone();
            let client = rpc_connection.rpc.clone();
            let task = pool.spawn(async move {
                let proof = get_proof(&client, wallet.pubkey());

                if let Ok(_) = proof {
                    return None;
                } else {
                    let signer = wallet;

                    let balance = if let Ok(balance) = client.get_balance(&signer.pubkey()) {
                        balance
                    } else {
                        return None;
                    };

                    if balance <= 0 {
                        error!("Insufficient Sol Balance!");
                        return None;
                    }

                    let ix = get_register_ix(signer.pubkey());
                    let latest_blockhash = client
                        .get_latest_blockhash_with_commitment(client.commitment());

                    if let Ok((hash, _slot)) = latest_blockhash {
                        let mut tx = Transaction::new_with_payer(&[ix], Some(&signer.pubkey()));

                        tx.sign(&[&signer], hash);

                        return Some(("Register".to_string(), tx, None));
                    } else {
                        error!("Failed to get latest blockhash. handle_event_submit_hash_tx");
                        return None;
                    }
                }
            });

            commands
                .entity(task_handler_entity)
                .insert(TaskProcessTx { task });
        } else {
            error!("Failed to get task_entity_handler. handle_event_register_wallet");
        }
    }
}

pub fn handle_event_claim_ore_rewards(
    mut commands: Commands,
    mut event_reader: EventReader<EventClaimOreRewards>,
    app_wallet: Res<AppWallet>,
    rpc_connection: ResMut<RpcConnection>,
    proof_account: Res<ProofAccountResource>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in event_reader.read() {
        if let Ok(task_handler_entity) = query_task_handler.get_single() {
            let pool = IoTaskPool::get();
            let wallet = app_wallet.wallet.clone();
            let client = rpc_connection.rpc.clone();
            let claim_amount = proof_account.stake;
            let task = pool.spawn(async move {
                let token_account_pubkey = spl_associated_token_account::get_associated_token_address(
                    &wallet.pubkey(),
                    &get_ore_mint(),
                );

                if let Ok(Some(_ata)) = client.get_token_account(&token_account_pubkey) {
                    let ix = get_claim_ix(wallet.pubkey(), token_account_pubkey, claim_amount);
                    let latest_blockhash = client
                        .get_latest_blockhash_with_commitment(client.commitment());

                    if let Ok((hash, _slot)) = latest_blockhash {
                        let mut tx = Transaction::new_with_payer(&[ix], Some(&wallet.pubkey()));

                        tx.sign(&[&wallet], hash);

                        return Some(("Claim".to_string(), tx, None));
                    } else {
                        error!("Failed to get latest blockhash. handle_event_claim_ore_rewards");
                        return None;
                        // error
                    }
                } else {
                    let ix = spl_associated_token_account::instruction::create_associated_token_account(
                        &wallet.pubkey(),
                        &wallet.pubkey(),
                        &get_ore_mint(),
                        &spl_token::id(),
                    );

                    let latest_blockhash = client
                        .get_latest_blockhash_with_commitment(client.commitment());

                    if let Ok((hash, _slot)) = latest_blockhash {
                        let mut tx = Transaction::new_with_payer(&[ix], Some(&wallet.pubkey()));

                        tx.sign(&[&wallet], hash);

                        return Some(("CreateAta".to_string(), tx, None));
                    } else {
                        error!("Failed to get latest blockhash. handle_event_claim_ore_rewards");
                        return None;
                        // error
                    }
                }
            });

            commands
                .entity(task_handler_entity)
                .insert(TaskProcessTx { task });
        } else {
            error!("Failed to get task_handler_entity. handle_event_claim_ore_rewards.");
        }
    }
}

pub fn handle_event_stake_ore(
    mut commands: Commands,
    mut event_reader: EventReader<EventStakeOre>,
    app_wallet: Res<AppWallet>,
    rpc_connection: ResMut<RpcConnection>,
    proof_account: Res<ProofAccountResource>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in event_reader.read() {
        if let Ok(task_handler_entity) = query_task_handler.get_single() {
            let pool = IoTaskPool::get();
            let wallet = app_wallet.wallet.clone();
            let client = rpc_connection.rpc.clone();
            let task = pool.spawn(async move {
                let token_account_pubkey = spl_associated_token_account::get_associated_token_address(
                    &wallet.pubkey(),
                    &get_ore_mint(),
                );

                if let Ok(Some(ata)) = client.get_token_account(&token_account_pubkey) {
                    if let Ok(stake_amount) = ata.token_amount.amount.parse::<u64>() {
                        let ix = get_stake_ix(wallet.pubkey(), token_account_pubkey, stake_amount);
                        let latest_blockhash = client
                            .get_latest_blockhash_with_commitment(client.commitment());

                        if let Ok((hash, _slot)) = latest_blockhash {
                            let mut tx = Transaction::new_with_payer(&[ix], Some(&wallet.pubkey()));

                            tx.sign(&[&wallet], hash);

                            return Some(("Stake".to_string(), tx, None));
                        } else {
                            error!("Failed to stake. handle_event_stake_ore.");
                            return None;
                            // error
                        }

                    } else {
                        error!("Failed to parse token amount for staking.");
                        return None;
                    }
                } else {
                    let ix = spl_associated_token_account::instruction::create_associated_token_account(
                        &wallet.pubkey(),
                        &wallet.pubkey(),
                        &get_ore_mint(),
                        &spl_token::id(),
                    );

                    let latest_blockhash = client
                        .get_latest_blockhash_with_commitment(client.commitment());

                    if let Ok((hash, _slot)) = latest_blockhash {
                        let mut tx = Transaction::new_with_payer(&[ix], Some(&wallet.pubkey()));

                        tx.sign(&[&wallet], hash);

                        return Some(("Create ATA".to_string(), tx, None));
                    } else {
                        error!("Failed to get latest blockhash. handle_event_claim_ore_rewards");
                        return None;
                        // error
                    }
                }
            });

            commands
                .entity(task_handler_entity)
                .insert(TaskProcessTx { task });
        } else {
            error!("Failed to get task_handler_entity. handle_event_stake_ore.");
        }
    }
}

pub fn handle_event_lock(
    mut commands: Commands,
    mut event_reader: EventReader<EventLock>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ev in event_reader.read() {
        commands.remove_resource::<AppWallet>();
        next_state.set(GameState::Locked);
    }
}

pub fn handle_event_unlock(
    mut commands: Commands,
    mut event_reader: EventReader<EventUnlock>,
    query: Query<&TextInput, With<TextPasswordInput>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ev in event_reader.read() {
        let text = query.get_single();
        if let Ok(text_input) = text {
            let password = text_input.text.clone();

            // TODO: use const path?
            let wallet_path = Path::new("save.data");

            let cocoon = Cocoon::new(password.as_bytes());
            let mut file = File::open(wallet_path).unwrap();
            let encoded = cocoon.parse(&mut file);
            if let Ok(encoded) = encoded {
                let wallet = Keypair::from_bytes(&encoded);
                if let Ok(wallet) = wallet {
                    let wallet = Arc::new(wallet);
                    commands.insert_resource(AppWallet {
                        wallet,
                        sol_balance: 0.0,
                        ore_balance: 0.0,
                    });
                    next_state.set(GameState::Mining);
                } else {
                    error!("Failed to parse keypair from bytes. (events.rs: handle_event_unlock)");
                }
            } else {
                error!("Failed to decrypt file. (events.rs: handle_event_unlock)");
            }
        } else {
            error!("Failed to get_single on TextPasswordInput (events.rs: handle_event_unlock)");
        }
    }
}

pub fn handle_event_save_config(
    mut event_reader: EventReader<EventSaveConfig>,
    mut ore_app_state: ResMut<OreAppState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for ev in event_reader.read() {
        let new_config = ev.0.clone();
        let toml_string = toml::to_string(&new_config).unwrap();
        let data = toml_string.into_bytes();

        let mut f = File::create("config.toml").expect("Unable to create file");
        f.write_all(&data).expect("Unable to write data");

        ore_app_state.config = new_config;
        next_state.set(GameState::Locked);
    }
}

pub fn handle_event_generate_wallet(
    mut event_reader: EventReader<EventGenerateWallet>,
    // mut text_query: Query<&mut Text, With<TextGeneratedPubkey>>,
    // mut ore_app_state: ResMut<OreAppState>,
    // mut next_state: ResMut<NextState<GameState>>,
    mut set: ParamSet<(
        Query<(&mut Text, &mut TextGeneratedKeypair)>,
        Query<&mut Text, With<TextMnemonicLine1>>,
        Query<&mut Text, With<TextMnemonicLine2>>,
        Query<&mut Text, With<TextMnemonicLine3>>,
    )>,
) {
    for _ev in event_reader.read() {
        let new_mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

        let phrase = new_mnemonic.clone().into_phrase();

        let words: Vec<&str> = phrase.split(" ").collect();

        let seed = Seed::new(&new_mnemonic, "");

        let derivation_path = DerivationPath::from_absolute_path_str("m/44'/501'/0'/0'").unwrap();

        let new_key = Keypair::from_seed_and_derivation_path(seed.as_bytes(), Some(derivation_path));
        if let Ok(new_key) = new_key {
            let new_key = Arc::new(new_key);
            let pubkey = new_key.pubkey().to_string();
            for (mut text, mut text_keypair) in set.p0().iter_mut() {
                text.sections[0].value = pubkey.clone();
                text_keypair.0 = new_key.clone();
            }
            for mut text in set.p1().iter_mut() {
                let mut value = String::new();
                for word in &words[0..4] {
                    value += word;
                    value += "     ";
                }
                text.sections[0].value = value;
            }
            for mut text in set.p2().iter_mut() {
                let mut value = String::new();
                for word in &words[4..8] {
                    value += word;
                    value += "     ";
                }
                text.sections[0].value = value;
            }
            for mut text in set.p3().iter_mut() {
                let mut value = String::new();
                for word in &words[8..12] {
                    value += word;
                    value += "     ";
                }
                text.sections[0].value = value;
            }
        } else {
            error!("Failed to generate keypair from seed as bytes");
        }
    }
}

pub fn handle_event_load_keypair_file(
    mut event_reader: EventReader<EventLoadKeypairFile>,
    // mut text_query: Query<&mut Text, With<TextGeneratedPubkey>>,
    // mut ore_app_state: ResMut<OreAppState>,
    // mut next_state: ResMut<NextState<GameState>>,
    mut set: ParamSet<(
        Query<(&mut Text, &mut TextGeneratedKeypair)>,
        Query<&mut Text, With<TextMnemonicLine1>>,
        Query<&mut Text, With<TextMnemonicLine2>>,
        Query<&mut Text, With<TextMnemonicLine3>>,
    )>,
) {
    for ev in event_reader.read() {
        let path = &ev.0;
        if let Ok(keypair) = read_keypair_file(path) {
            let keypair = Arc::new(keypair);
            let pubkey = keypair.pubkey().to_string();
            for (mut text, mut text_keypair) in set.p0().iter_mut() {
                text.sections[0].value = pubkey.clone();
                text_keypair.0 = keypair.clone();
            }
            for mut text in set.p1().iter_mut() {
                let value = String::new();
                text.sections[0].value = value;
            }
            for mut text in set.p2().iter_mut() {
                let value = String::new();
                text.sections[0].value = value;
            }
            for mut text in set.p3().iter_mut() {
                let value = String::new();
                text.sections[0].value = value;
            }
        } else {
            error!("Error: Failed to load keypair file from path: {}", path.display());
        }

    }
}

pub fn handle_event_save_wallet(
    mut event_reader: EventReader<EventSaveWallet>,
    mut set: ParamSet<(
        Query<&TextGeneratedKeypair>,
        Query<&TextInput, With<TextPasswordInput>>,
    )>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ev in event_reader.read() {
        let generated_keypair = set.p0().single().0.clone();

        let password = set.p1().single().text.clone();

        let wallet_path = Path::new("save.data");

        let cocoon = Cocoon::new(password.as_bytes());
        let wallet_bytes = generated_keypair.to_bytes();
        let file = File::create(wallet_path);

        if let Ok(mut file) = file {
            let container = cocoon.dump(wallet_bytes.to_vec(), &mut file);

            if let Ok(_) = container {
                // go to locked screen
                next_state.set(GameState::Locked);
            } else {
                error!("Error: Failed to save wallet file.");
            }
        } else {
            error!("Error: failed to create file at path: {}", wallet_path.display());
        }
    }
}
