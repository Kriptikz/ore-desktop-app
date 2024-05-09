use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, IoTaskPool},
};
use cocoon::Cocoon;
use ore::state::Proof;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_transaction_status::UiTransactionEncoding;
use spl_associated_token_account::get_associated_token_address;

use crate::{
    ore_utils::{
        get_claim_ix, get_clock_account, get_cutoff, get_mine_ix, get_ore_epoch_duration, get_ore_mint, get_proof, get_proof_and_treasury, get_register_ix, get_reset_ix, get_treasury, proof_pubkey, treasury_tokens_pubkey
    }, tasks::{
        TaskGenerateHash, TaskProcessTx, TaskRegisterWallet, TaskUpdateAppWalletSolBalance,
        TaskUpdateAppWalletSolBalanceData, TaskUpdateCurrentTx,
    }, ui::{
        components::{MovingScrollPanel, TextInput, TextPasswordInput},
        spawn_utils::{spawn_new_list_item, UiListItem},
    }, AppWallet, Config, EntityTaskFetchUiData, EntityTaskHandler, GameState, MinerStatusResource, OreAppState, ProofAccountResource, RpcConnection, TreasuryAccountResource, TxStatus
};

use std::{
    fs::File, io::{stdout, Write}, path::Path, str::FromStr, sync::{atomic::AtomicBool, Arc, Mutex}, time::Instant
};

use solana_sdk::{
    bs58, commitment_config::CommitmentLevel, keccak::{hashv, Hash as KeccakHash}, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{Keypair, Signer}, transaction::Transaction
};

// Events
#[derive(Event)]
pub struct EventStartStopMining;

// Events
#[derive(Event)]
pub struct EventMineForHash;

#[derive(Event)]
pub struct EventSubmitHashTx(pub (solana_program::keccak::Hash, u64, u32, u64));

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
    pub hash_status: Option<(u64, u32)>,
    pub tx_status: TxStatus,
}

#[derive(Event)]
pub struct EventFetchUiDataFromRpc;

#[derive(Event)]
pub struct EventResetEpoch;

#[derive(Event)]
pub struct EventRegisterWallet;

#[derive(Event)]
pub struct EventClaimOreRewards;

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
    rpc_connection: Res<RpcConnection>,
) {
    for _ev in ev_start_stop_mining.read() {
        info!("Start/Stop Mining Event Handler.");
        let client = rpc_connection.rpc.clone();
        let proof_address = proof_pubkey(app_wallet.wallet.pubkey());
        if client.get_account(&proof_address).is_ok() {
            info!("Is Successfully registered!!!");
            info!("Sending EventMineForHash");
            event_writer.send(EventMineForHash);
        } else {
            info!("Sending Register Event.");
            event_writer_register.send(EventRegisterWallet);
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
        info!("Mine For Hash Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();
        let wallet = app_wallet.wallet.clone();
        let client = rpc_connection.rpc.clone();
        let threads = miner_status.miner_threads;
        let task = pool.spawn(async move {
            // TODO: use proof resource cached proof. May need LatestHash Resource to ensure a new proof if loaded before mining.
            //  get proof account data
            let proof = get_proof(&client, wallet.pubkey())
                .expect("Should have succesfully got proof account");
            // TODO: use treasury resource cached difficulty
            let treasury =
                get_treasury(&client).expect("Should have succesfully got treasury account.");
            // ensure proof account is hash is not the same as the last generated one.
            // which results in 0x3 - Hash already submitted. Stale RPC Data...
            info!("\nMining for a valid hash...");

            let hash_time = Instant::now();
            let (best_nonce, best_difficulty, best_hash) = find_hash_par(
                wallet.pubkey(),
                2,
                threads,
                &client,
                proof,
            );
            info!("BEST HASH: {}", best_hash.to_string());
            info!("BEST DIFFICULTY: {}", best_difficulty.to_string());
            info!("BEST NONCE: {}", best_nonce.to_string());

            let best_hash = KeccakHash::from_str(&best_hash).unwrap();

            (best_hash, best_nonce, best_difficulty, hash_time.elapsed().as_secs())
        });
        miner_status.miner_status = "MINING".to_string();

        commands
            .entity(task_handler_entity)
            .insert(TaskGenerateHash { task });
    }
}

pub fn handle_event_process_tx(
    mut commands: Commands,
    mut ev_submit_hash_tx: EventReader<EventProcessTx>,
    mut miner_status: ResMut<MinerStatusResource>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
    rpc_connection: Res<RpcConnection>,
) {
    for ev in ev_submit_hash_tx.read() {
        info!("ProcessTx Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();

        let client = rpc_connection.rpc.clone();
        let tx_type = ev.tx_type.clone();
        let tx = ev.tx.clone();
        let hash_time = ev.hash_status.clone();
        let task = pool.spawn(async move {
            let send_cfg = RpcSendTransactionConfig {
                skip_preflight: true,
                preflight_commitment: Some(CommitmentLevel::Confirmed),
                encoding: Some(UiTransactionEncoding::Base64),
                max_retries: Some(0),
                min_context_slot: None,
            };

            let sig = client.send_transaction_with_config(&tx, send_cfg);
            if sig.is_err() {}
            if let Ok(sig) = sig {
                return Some((tx_type, tx, sig, hash_time));
            } else {
                info!("Failed to send initial transaction...");
                return None;
            }
        });
        miner_status.miner_status = "PROCESSING".to_string();

        commands
            .entity(task_handler_entity)
            .insert(TaskUpdateCurrentTx { task });
    }
}

pub fn handle_event_submit_hash_tx(
    mut commands: Commands,
    mut ev_submit_hash_tx: EventReader<EventSubmitHashTx>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
    app_wallet: Res<AppWallet>,
    rpc_connection: Res<RpcConnection>,
) {
    for ev in ev_submit_hash_tx.read() {
        info!("Submit Hash Tx Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();
        let wallet = app_wallet.wallet.clone();
        let client = rpc_connection.rpc.clone();

        let (next_hash, nonce, difficulty, hash_time) = ev.0;
        let task = pool.spawn(async move {
            let signer = wallet;
            // TODO: set cu's
            // let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(CU_LIMIT_MINE);
            // let cu_price_ix =
            //     ComputeBudgetInstruction::set_compute_unit_price(self.priority_fee);
            let ix_mine = get_mine_ix(signer.pubkey(), nonce);
            let (hash, _slot) = client
                .get_latest_blockhash_with_commitment(client.commitment())
                .unwrap();
            let mut tx = Transaction::new_with_payer(&[ix_mine], Some(&signer.pubkey()));

            tx.sign(&[&signer], hash);

            return Some(("Mine".to_string(), tx, Some((hash_time, difficulty))));
        });

        commands
            .entity(task_handler_entity)
            .insert(TaskProcessTx { task });
    }
}

pub fn handle_event_tx_result(
    mut commands: Commands,
    mut ev_tx_result: EventReader<EventTxResult>,
    mut event_writer: EventWriter<EventMineForHash>,
    mut miner_status: ResMut<MinerStatusResource>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MovingScrollPanel>>,
) {
    for ev in ev_tx_result.read() {
        info!("Tx Result Event Handler.");
        let (hash_time, difficulty) = if let Some(ht) = ev.hash_status {
            (ht.0.to_string(), ht.1.to_string())
        } else {
            ("N/A".to_string(), "".to_string())
        };
        miner_status.miner_status = "STOPPED".to_string();
        let scroll_panel_entity = query.get_single().unwrap();
        let status = format!(
            "{}  {}",
            ev.tx_status.status.clone(),
            ev.tx_status.error.clone()
        );

        let hash_time = format!("{} - {}", hash_time, difficulty);
        let item_data = UiListItem {
            id: ev.tx_type.clone(),
            sig: ev.sig.clone(),
            tx_time: ev.tx_time.to_string(),
            hash_time,
            status,
        };
        spawn_new_list_item(&mut commands, &asset_server, scroll_panel_entity, item_data);
         
        if ev.tx_type == "Mine" {
            event_writer.send(EventMineForHash);
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
        info!("Fetch UI Data From RPC Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pubkey = app_wallet.wallet.pubkey();

        let pool = IoTaskPool::get();

        let connection = rpc_connection.rpc.clone();
        let ore_mint = get_ore_mint();
        let task = pool.spawn(async move {
            let balance = connection.get_balance(&pubkey).unwrap();
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
            let (proof_account, treasury_account) = get_proof_and_treasury(&connection, pubkey);

            let proof_account_res_data;
            if let Ok(proof_account) = proof_account {
                proof_account_res_data = ProofAccountResource {
                    current_hash: KeccakHash::new_from_array(proof_account.challenge).to_string(),
                    total_hashes: proof_account.total_hashes,
                    total_rewards: proof_account.total_rewards,
                    claimable_rewards: proof_account.balance,
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

            let treasury_account_res_data;
            if let Ok(treasury_account) = treasury_account {
                // let reward_rate =
                //     (treasury_account.reward_rate as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
                // let total_claimed_rewards = (treasury_account.total_claimed_rewards as f64)
                //     / 10f64.powf(ore::TOKEN_DECIMALS as f64);
                let reward_rate = 0;

                let clock = get_clock_account(&connection);
                // let threshold = treasury_account
                //     .last_reset_at
                //     .saturating_add(get_ore_epoch_duration());

                // let need_epoch_reset = if clock.unix_timestamp.ge(&threshold) {
                //     true
                // } else {
                //     false
                // };
                let need_epoch_reset = false;

                // treasury_account_res_data = TreasuryAccountResource {
                //     balance: treasury_ore_balance.to_string(),
                //     admin: treasury_account.admin.to_string(),
                //     difficulty: treasury_account.difficulty.to_string(),
                //     last_reset_at: treasury_account.last_reset_at,
                //     need_epoch_reset,
                //     reward_rate,
                //     total_claimed_rewards,
                // };
                treasury_account_res_data = TreasuryAccountResource {
                    balance: treasury_ore_balance.to_string(),
                    admin: "todo".to_string(),
                    difficulty: "todo".to_string(),
                    last_reset_at: 0,
                    need_epoch_reset,
                    reward_rate: 0.0,
                    total_claimed_rewards: 0.0,
                };
            } else {
                treasury_account_res_data = TreasuryAccountResource {
                    balance: "Not Found".to_string(),
                    admin: "".to_string(),
                    difficulty: "".to_string(),
                    last_reset_at: 0,
                    need_epoch_reset: false,
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
            .entity(task_handler_entity)
            .insert(TaskUpdateAppWalletSolBalance { task });
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
        info!("RegisterWallet Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();
        let wallet = app_wallet.wallet.clone();
        let client = rpc_connection.rpc.clone();
        let task = pool.spawn(async move {
            let proof = get_proof(&client, wallet.pubkey());

            if let Ok(_) = proof {
                info!("Proof account already exists!");
                return None;
            } else {
                info!("Failed to get proof account, registering wallet...");
                println!("Generating challenge...");
                let signer = wallet;

                let balance = client.get_balance(&signer.pubkey()).unwrap();
                if balance <= 0 {
                    info!("Insufficient Sol Balance!");
                    return None;
                }

                let ix = get_register_ix(signer.pubkey());
                let (hash, _slot) = client
                    .get_latest_blockhash_with_commitment(client.commitment())
                    .unwrap();
                let mut tx = Transaction::new_with_payer(&[ix], Some(&signer.pubkey()));

                tx.sign(&[&signer], hash);

                return Some(tx);
            }
        });

        commands
            .entity(task_handler_entity)
            .insert(TaskRegisterWallet { task });
    }
}

pub fn handle_event_reset_epoch(
    mut commands: Commands,
    mut event_reader: EventReader<EventResetEpoch>,
    app_wallet: Res<AppWallet>,
    rpc_connection: ResMut<RpcConnection>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in event_reader.read() {
        info!("Reset Treasury Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();
        let wallet = app_wallet.wallet.clone();
        let client = rpc_connection.rpc.clone();
        let task = pool.spawn(async move {
            let ix = get_reset_ix(wallet.pubkey());
            let (hash, _slot) = client
                .get_latest_blockhash_with_commitment(client.commitment())
                .unwrap();
            let mut tx = Transaction::new_with_payer(&[ix], Some(&wallet.pubkey()));

            tx.sign(&[&wallet], hash);

            return Some(("Reset".to_string(), tx, None));
        });

        commands
            .entity(task_handler_entity)
            .insert(TaskProcessTx { task });
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
        info!("Claim Ore Rewards Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();
        let wallet = app_wallet.wallet.clone();
        let client = rpc_connection.rpc.clone();
        let claim_amount = proof_account.claimable_rewards;
        let task = pool.spawn(async move {
            let token_account_pubkey = spl_associated_token_account::get_associated_token_address(
                &wallet.pubkey(),
                &get_ore_mint(),
            );

            if let Ok(Some(_ata)) = client.get_token_account(&token_account_pubkey) {
                let ix = get_claim_ix(wallet.pubkey(), token_account_pubkey, claim_amount);

                let (hash, _slot) = client
                    .get_latest_blockhash_with_commitment(client.commitment())
                    .unwrap();
                let mut tx = Transaction::new_with_payer(&[ix], Some(&wallet.pubkey()));

                tx.sign(&[&wallet], hash);

                return Some(("Claim".to_string(), tx, None));
            } else {
                let ix = spl_associated_token_account::instruction::create_associated_token_account(
                    &wallet.pubkey(),
                    &wallet.pubkey(),
                    &get_ore_mint(),
                    &spl_token::id(),
                );

                let (hash, _slot) = client
                    .get_latest_blockhash_with_commitment(client.commitment())
                    .unwrap();
                let mut tx = Transaction::new_with_payer(&[ix], Some(&wallet.pubkey()));

                tx.sign(&[&wallet], hash);

                return Some(("Create ATA".to_string(), tx, None));
            }
        });

        commands
            .entity(task_handler_entity)
            .insert(TaskProcessTx { task });
    }
}

pub fn handle_event_lock(
    mut commands: Commands,
    mut event_reader: EventReader<EventLock>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ev in event_reader.read() {
        info!("Lock Event Handler.");
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
        info!("Unlock Event Handler.");
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
                    info!("Successfully loaded wallet!");
                    next_state.set(GameState::Mining);
                } else {
                    info!("Failed to parse keypair from bytes. (events.rs: handle_event_unlock)");
                }
            } else {
                info!("Failed to decrypt file. (events.rs: handle_event_unlock)");
            }
        } else {
            info!("Failed to get_single on TextPasswordInput (events.rs: handle_event_unlock)");
        }
    }
}

pub fn handle_event_save_config(
    mut event_reader: EventReader<EventSaveConfig>,
    mut ore_app_state: ResMut<OreAppState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for ev in event_reader.read() {
        info!("Save Config Event Handler.");
        let new_config = ev.0.clone();
        let toml_string = toml::to_string(&new_config).unwrap();
        let data = toml_string.into_bytes();

        let mut f = File::create("config.toml").expect("Unable to create file");
        f.write_all(&data).expect("Unable to write data");

        ore_app_state.config = new_config;
        next_state.set(GameState::Locked);
    }
}

fn find_next_hash_par(
    signer: Arc<Keypair>,
    hash: KeccakHash,
    difficulty: KeccakHash,
    threads: u64,
) -> (KeccakHash, u64) {
    let found_solution = Arc::new(AtomicBool::new(false));
    let solution = Arc::new(Mutex::<(KeccakHash, u64)>::new((
        KeccakHash::new_from_array([0; 32]),
        0,
    )));
    let pubkey = signer.pubkey();
    let thread_handles: Vec<_> = (0..threads)
        .map(|i| {
            std::thread::spawn({
                let found_solution = found_solution.clone();
                let solution = solution.clone();
                let mut stdout = stdout();
                move || {
                    let n = u64::MAX.saturating_div(threads).saturating_mul(i);
                    let mut next_hash: KeccakHash;
                    let mut nonce: u64 = n;
                    loop {
                        next_hash = hashv(&[
                            hash.to_bytes().as_slice(),
                            pubkey.to_bytes().as_slice(),
                            nonce.to_le_bytes().as_slice(),
                        ]);
                        if nonce % 10_000 == 0 {
                            if found_solution.load(std::sync::atomic::Ordering::Relaxed) {
                                return;
                            }
                            if n == 0 {
                                stdout
                                    .write_all(format!("\r{}", next_hash.to_string()).as_bytes())
                                    .ok();
                            }
                        }
                        if next_hash.le(&difficulty) {
                            stdout
                                .write_all(format!("\r{}", next_hash.to_string()).as_bytes())
                                .ok();
                            found_solution.store(true, std::sync::atomic::Ordering::Relaxed);
                            let mut w_solution = solution.lock().expect("failed to lock mutex");
                            *w_solution = (next_hash, nonce);
                            return;
                        }
                        nonce += 1;
                    }
                }
            })
        })
        .collect();

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }

    let r_solution = solution.lock().expect("Failed to get lock");
    *r_solution
}

pub fn register(signer: Keypair, client: &RpcClient) -> bool {
    // Return early if miner is already registered
    let proof_address = proof_pubkey(signer.pubkey());
    if client.get_account(&proof_address).is_ok() {
        return true;
    }
    println!("Generating challenge...");

    let balance = client.get_balance(&signer.pubkey()).unwrap();
    if balance <= 0 {
        info!("Insufficient Sol Balance!");
        return false;
    }

    let ix = get_register_ix(signer.pubkey());
    let (hash, _slot) = client
        .get_latest_blockhash_with_commitment(client.commitment())
        .unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&signer.pubkey()));

    tx.sign(&[&signer], hash);

    info!("Sending and confirming tx...");
    let result = client.send_and_confirm_transaction(&tx);
    info!("Tx Result: {:?}", result);
    if result.is_ok() {
        return true;
    }

    return false;
}

fn find_hash_par(signer: Pubkey, buffer_time: u64, threads: u64, rpc_client: &RpcClient, proof_account: Proof) -> (u64, u32, String) {
    // Check num threads
    // self.check_num_cores(threads);

    // Fetch data
    let proof = proof_account;
    println!(
        "\nStake balance: 0 ORE",
    );
    let cutoff_time = get_cutoff(proof, buffer_time);

    // Dispatch job to each thread
    // let progress_bar = Arc::new(spinner::new_progress_bar());
    // progress_bar.set_message("Mining...");
    let handles: Vec<_> = (0..threads)
        .map(|i| {
            std::thread::spawn({
                let proof = proof.clone();
                // let progress_bar = progress_bar.clone();
                move || {
                    let timer = Instant::now();
                    let first_nonce = u64::MAX.saturating_div(threads).saturating_mul(i);
                    let mut nonce = first_nonce;
                    let mut best_nonce = nonce;
                    let mut best_difficulty = 0;
                    let mut best_hash = [0; 32];
                    loop {
                        // Create hash
                        let hx = drillx::hash(&proof.challenge, &nonce.to_le_bytes());
                        let difficulty = drillx::difficulty(hx);

                        // Check difficulty
                        if difficulty.gt(&best_difficulty) {
                            best_nonce = nonce;
                            best_difficulty = difficulty;
                            best_hash = hx;
                        }

                        // Exit if time has elapsed
                        if nonce % 10_000 == 0 {
                            if (timer.elapsed().as_secs() as i64).ge(&cutoff_time) {
                                if best_difficulty.gt(&ore::MIN_DIFFICULTY) {
                                    // Mine until min difficulty has been met
                                    break;
                                }
                            } else if i == 0 {
                                // progress_bar.set_message(format!(
                                //     "Mining... ({} sec remaining)",
                                //     cutoff_time
                                //         .saturating_sub(timer.elapsed().as_secs() as i64),
                                // ));
                            }
                        }

                        // Increment nonce
                        nonce += 1;
                    }

                    // Return the best nonce
                    (best_nonce, best_difficulty, best_hash)
                }
            })
        })
        .collect();

    // Join handles and return best nonce
    let mut best_nonce = 0;
    let mut best_difficulty = 0;
    let mut best_hash = [0; 32];
    for h in handles {
        if let Ok((nonce, difficulty, hash)) = h.join() {
            if difficulty > best_difficulty {
                best_difficulty = difficulty;
                best_nonce = nonce;
                best_hash = hash;
            }
        }
    }

    let best_hash_str = bs58::encode(best_hash).into_string();
    // info!(format!(
    //     "Best hash: {} (difficulty: {})",
    //     best_hash_str.clone(),
    //     best_difficulty
    // ));

    (best_nonce, best_difficulty, best_hash_str)
}
