use bevy::{
    prelude::*, tasks::{block_on, futures_lite::future, Task}
};

use crate::{AppWallet, ProofAccountResource, TreasuryAccountResource};

// Task Components
// TODO: tasks should return results so errors can be dealt with by the task handler system
pub struct TaskUpdateAppWalletSolBalanceData {
    pub sol_balance: f64,
    pub ore_balance: f64,
    pub proof_account_data: ProofAccountResource,
    pub treasury_account_data: TreasuryAccountResource,
}
#[derive(Component)]
pub struct TaskUpdateAppWalletSolBalance {
    pub task: Task<TaskUpdateAppWalletSolBalanceData>,
}

#[derive(Component)]
pub struct TaskGenerateHash {
    pub task: Task<String>,
}

#[derive(Component)]
pub struct TaskSendTx {
    pub task: Task<String>,
}

#[derive(Component)]
pub struct TaskConfirmTransaction {
    pub task: Task<String>,
}

pub fn task_update_app_wallet_sol_balance(
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