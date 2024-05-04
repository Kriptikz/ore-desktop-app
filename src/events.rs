use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, IoTaskPool},
};
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_transaction_status::UiTransactionEncoding;
use spl_associated_token_account::get_associated_token_address;

use crate::{
    ui::{
        components::MovingScrollPanel,
        layout::{spawn_new_list_item, UiListItem},
    },
    AppWallet, EntityTaskFetchUiData, EntityTaskHandler, MinerStatusResource, OreAppState,
    ProofAccountResource, RpcConnection, TaskGenerateHash, TaskProcessTx, TaskRegisterWallet,
    TaskUpdateAppWalletSolBalance, TaskUpdateAppWalletSolBalanceData, TaskUpdateCurrentTx,
    TreasuryAccountResource, TxStatus,
};

use std::{
    io::{stdout, Write},
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Instant,
};

use orz::{
    self,
    state::{Proof, Treasury},
    utils::AccountDeserialize,
    BUS_ADDRESSES, EPOCH_DURATION, MINT_ADDRESS, PROOF, TREASURY_ADDRESS,
};
use solana_sdk::{
    account::ReadableAccount,
    clock::Clock,
    commitment_config::CommitmentLevel,
    keccak::{hashv, Hash as KeccakHash},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    sysvar,
    transaction::Transaction,
};

// Events
#[derive(Event)]
pub struct EventStartStopMining;

// Events
#[derive(Event)]
pub struct EventMineForHash;

#[derive(Event)]
pub struct EventSubmitHashTx(pub (solana_program::keccak::Hash, u64, u64));

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
    pub hash_time: Option<u64>,
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
    pub hash_time: Option<u64>,
}

pub fn handle_event_start_stop_mining_clicked(
    mut ev_start_stop_mining: EventReader<EventStartStopMining>,
    mut event_writer: EventWriter<EventMineForHash>,
    mut event_writer_register: EventWriter<EventRegisterWallet>,
    app_wallet: Res<AppWallet>,
    rpc_connection: Res<RpcConnection>,
) {
    for _ev in ev_start_stop_mining.read() {
        info!("Start/Stop Mining Event Handler.");
        // check for proof account.
        // if no proof account. create one.
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
        let wallet = app_wallet.wallet.insecure_clone();
        let client = rpc_connection.rpc.clone();
        let threads = miner_status.miner_threads;
        let task = pool.spawn(async move {
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
            let (next_hash, nonce) =
                find_next_hash_par(wallet, proof.hash.into(), treasury.difficulty.into(), threads);
            info!("NEXT HASH: {}", next_hash.to_string());
            info!("NONCE: {}", nonce.to_string());

            (next_hash, nonce, hash_time.elapsed().as_secs())
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

        // TODO: spawn the tx sender task
        // TODO: MAKE AppWallet Wallet Arc, so can clone properly
        //let wallet = app_wallet.wallet.insecure_clone();
        let client = rpc_connection.rpc.clone();
        let tx_type = ev.tx_type.clone();
        let tx = ev.tx.clone();
        let hash_time = ev.hash_time.clone();
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
        let wallet = app_wallet.wallet.insecure_clone();
        let client = rpc_connection.rpc.clone();

        let (next_hash, nonce, hash_time) = ev.0;
        let task = pool.spawn(async move {
            let signer = wallet;
            // let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(CU_LIMIT_MINE);
            // let cu_price_ix =
            //     ComputeBudgetInstruction::set_compute_unit_price(self.priority_fee);
            let ix_mine =
                orz::instruction::mine(signer.pubkey(), BUS_ADDRESSES[0], next_hash.into(), nonce);
            let (hash, _slot) = client
                .get_latest_blockhash_with_commitment(client.commitment())
                .unwrap();
            let mut tx = Transaction::new_with_payer(&[ix_mine], Some(&signer.pubkey()));

            tx.sign(&[&signer], hash);

            return Some(("Mine".to_string(), tx, Some(hash_time)));
        });

        commands
            .entity(task_handler_entity)
            .insert(TaskProcessTx { task });
    }
}

pub fn handle_event_tx_result(
    mut commands: Commands,
    mut ev_tx_result: EventReader<EventTxResult>,
    mut miner_status: ResMut<MinerStatusResource>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MovingScrollPanel>>,
) {
    for ev in ev_tx_result.read() {
        info!("Tx Result Event Handler.");
        let hash_time = if let Some(ht) = ev.hash_time {
            ht.to_string()
        } else {
            "N/A".to_string()
        };
        miner_status.miner_status = "STOPPED".to_string();
        let scroll_panel_entity = query.get_single().unwrap();
        let status = format!(
            "{}  {}",
            ev.tx_status.status.clone(),
            ev.tx_status.error.clone()
        );
        let item_data = UiListItem {
            id: ev.tx_type.clone(),
            sig: ev.sig.clone(),
            tx_time: ev.tx_time.to_string(),
            hash_time,
            status,
        };
        spawn_new_list_item(&mut commands, &asset_server, scroll_panel_entity, item_data);
    }
}

pub fn handle_event_fetch_ui_data_from_rpc(
    mut commands: Commands,
    app_wallet: Res<AppWallet>,
    ore_app_state: Res<OreAppState>,
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
        let ore_mint = MINT_ADDRESS;
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

            let (proof_account, treasury_account) = get_proof_and_treasury(&connection, pubkey);

            //let proof_account = get_proof(&connection, pubkey);
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

            //let treasury_account = get_treasury(&connection);
            let treasury_account_res_data;
            if let Ok(treasury_account) = treasury_account {
                let reward_rate =
                    (treasury_account.reward_rate as f64) / 10f64.powf(orz::TOKEN_DECIMALS as f64);
                let total_claimed_rewards = (treasury_account.total_claimed_rewards as f64)
                    / 10f64.powf(orz::TOKEN_DECIMALS as f64);

                let clock = get_clock_account(&connection);
                let threshold = treasury_account
                    .last_reset_at
                    .saturating_add(EPOCH_DURATION);

                let need_epoch_reset = if clock.unix_timestamp.ge(&threshold) {
                    true
                } else {
                    false
                };

                treasury_account_res_data = TreasuryAccountResource {
                    balance: treasury_ore_balance.to_string(),
                    admin: treasury_account.admin.to_string(),
                    difficulty: treasury_account.difficulty.to_string(),
                    last_reset_at: treasury_account.last_reset_at,
                    need_epoch_reset,
                    reward_rate,
                    total_claimed_rewards,
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
        let wallet = app_wallet.wallet.insecure_clone();
        let client = rpc_connection.rpc.clone();
        let task = pool.spawn(async move {
            //  get proof account data
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

                let ix = orz::instruction::register(signer.pubkey());
                // Build tx
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
        let wallet = app_wallet.wallet.insecure_clone();
        let client = rpc_connection.rpc.clone();
        let task = pool.spawn(async move {
            let ix = orz::instruction::reset(wallet.pubkey());
            // Build tx
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
        let wallet = app_wallet.wallet.insecure_clone();
        let client = rpc_connection.rpc.clone();
        let claim_amount = proof_account.claimable_rewards;
        let task = pool.spawn(async move {
            // if no ata, make ata transaction first.
            let token_account_pubkey = spl_associated_token_account::get_associated_token_address(
                &wallet.pubkey(),
                &MINT_ADDRESS,
            );

            // Check if ata already exists
            if let Ok(Some(_ata)) = client.get_token_account(&token_account_pubkey) {
                let ix = orz::instruction::claim(wallet.pubkey(), token_account_pubkey, claim_amount);

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
                    &MINT_ADDRESS,
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

fn find_next_hash_par(
    signer: Keypair,
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

    let ix = orz::instruction::register(signer.pubkey());
    // Build tx
    let (hash, _slot) = client
        .get_latest_blockhash_with_commitment(client.commitment())
        .unwrap();
    // let send_cfg = RpcSendTransactionConfig {
    //     skip_preflight: true,
    //     preflight_commitment: Some(CommitmentLevel::Confirmed),
    //     encoding: Some(UiTransactionEncoding::Base64),
    //     max_retries: Some(0),
    //     min_context_slot: None,
    // };
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

// ORE Utility Functions
pub fn get_proof_and_treasury(
    client: &RpcClient,
    authority: Pubkey,
) -> (Result<Proof, ()>, Result<Treasury, ()>) {
    let account_pubkeys = vec![TREASURY_ADDRESS, proof_pubkey(authority)];
    let datas = client.get_multiple_accounts(&account_pubkeys);
    if let Ok(datas) = datas {
        let treasury = if let Some(data) = &datas[0] {
            Ok(*Treasury::try_from_bytes(data.data()).expect("Failed to parse treasury account"))
        } else {
            Err(())
        };

        let proof = if let Some(data) = &datas[1] {
            Ok(*Proof::try_from_bytes(data.data()).expect("Failed to parse treasury account"))
        } else {
            Err(())
        };

        (proof, treasury)
    } else {
        (Err(()), Err(()))
    }
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
    Pubkey::find_program_address(&[PROOF, authority.as_ref()], &orz::ID).0
}

pub fn treasury_tokens_pubkey() -> Pubkey {
    get_associated_token_address(&TREASURY_ADDRESS, &MINT_ADDRESS)
}

pub fn get_clock_account(client: &RpcClient) -> Clock {
    let data = client
        .get_account_data(&sysvar::clock::ID)
        .expect("Failed to get miner account");
    bincode::deserialize::<Clock>(&data).expect("Failed to deserialize clock")
}
