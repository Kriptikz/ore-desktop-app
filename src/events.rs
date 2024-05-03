use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, signer::Signer};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    get_proof, get_treasury, treasury_tokens_pubkey, ui::{components::MovingScrollPanel, layout::spawn_new_list_item}, AppWallet, EntityTaskHandler, OreAppState, ProofAccountResource, RpcConnection, TaskConfirmTx, TaskGenerateHash, TaskSendAndConfirmTx, TaskSendTx, TaskUpdateAppWalletSolBalance, TaskUpdateAppWalletSolBalanceData, TreasuryAccountResource
};

// Events
#[derive(Event)]
pub struct EventStartStopMining;

#[derive(Event)]
pub struct EventSubmitHashTx(pub String);

pub struct TxResult {
    pub sig: String,
    pub tx_time: u64,
    pub hash_time: u64,
    // TODO: create a TxStatus struct that will be able to show different colors based on status enums
    pub status: String,
}

#[derive(Event)]
pub struct EventTxResult;

#[derive(Event)]
pub struct EventFetchUiDataFromRpc;

pub fn handle_event_start_stop_mining_clicked(
    mut commands: Commands,
    mut ev_start_stop_mining: EventReader<EventStartStopMining>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in ev_start_stop_mining.read() {
        info!("Start/Stop Mining Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();
        let task = pool.spawn(async move { "NEWGENERATEDHASH".to_string() });

        commands
            .entity(task_handler_entity)
            .insert(TaskGenerateHash { task });
    }
}

pub fn handle_event_submit_hash_tx(
    mut commands: Commands,
    mut ev_submit_hash_tx: EventReader<EventSubmitHashTx>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in ev_submit_hash_tx.read() {
        info!("Submit Hash Tx Event Handler.");
        // TODO: sign the tx
        let task_handler_entity = query_task_handler.get_single().unwrap();
        let pool = AsyncComputeTaskPool::get();

        // TODO: spawn the tx sender task
        let task = pool.spawn(async move {
            // start a timer
            // sign the transaction
            // send the transaction
            let mut i = 0;
            // loop
            loop {
                // based on timer, resend signed tx
                // based on timer, check tx status
                // if blockhash expired, return with FAILED - Blockhash Expired
                i += 1;
                if i > 100 {
                    return ("SIGNATURE".to_string(), "SUCCESS".to_string());
                }
            }
        });

        commands
            .entity(task_handler_entity)
            .insert(TaskSendAndConfirmTx { task });
    }
}

pub fn handle_event_tx_result(
    mut commands: Commands,
    mut ev_tx_result: EventReader<EventTxResult>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MovingScrollPanel>>,
) {
    for _ev in ev_tx_result.read() {
        info!("Tx Result Event Handler.");
        let scroll_panel_entity = query.get_single().unwrap();
        spawn_new_list_item(&mut commands, &asset_server, scroll_panel_entity);
    }
}

pub fn handle_event_fetch_ui_data_from_rpc(
    mut commands: Commands,
    app_wallet: Res<AppWallet>,
    ore_app_state: Res<OreAppState>,
    rpc_connection: ResMut<RpcConnection>,
    mut event_reader: EventReader<EventFetchUiDataFromRpc>,
    query_task_handler: Query<Entity, With<EntityTaskHandler>>,
) {
    for _ev in event_reader.read() {
        info!("Fetch UI Data From RPC Event Handler.");
        let task_handler_entity = query_task_handler.get_single().unwrap();
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
                let reward_rate =
                    (treasury_account.reward_rate as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
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
            .entity(task_handler_entity)
            .insert(TaskUpdateAppWalletSolBalance { task });
    }
}
