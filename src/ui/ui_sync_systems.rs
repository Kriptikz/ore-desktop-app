
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use chrono::DateTime;

use crate::ore_utils::get_ore_decimals;
use crate::utils::{get_unix_timestamp, human_bytes, shorten_string};
use crate::AppWallet;
use crate::CurrentTx;
use crate::MinerStatusResource;
use crate::ProofAccountResource;
use crate::TreasuryAccountResource;

use super::components::FpsRoot;
use super::components::FpsText;
use super::components::ScrollingList;
use super::components::TextClaimableRewards;
use super::components::TextCurrentHash;
use super::components::TextCurrentTxElapsed;
use super::components::TextCurrentTxSig;
use super::components::TextCurrentTxStatus;
use super::components::TextInput;
use super::components::TextMinerStatusCpuUsage;
use super::components::TextMinerStatusRamUsage;
use super::components::TextMinerStatusStatus;
use super::components::TextMinerStatusTime;
use super::components::TextTotalHashes;
use super::components::TextTotalRewards;
use super::components::TextTreasuryAdmin;
use super::components::TextTreasuryBalance;
use super::components::TextTreasuryDifficulty;
use super::components::TextTreasuryLastResetAt;
use super::components::TextTreasuryNeedEpochReset;
use super::components::TextTreasuryRewardRate;
use super::components::TextTreasuryTotalClaimedRewards;
use super::components::TextWalletOreBalance;
use super::components::TextWalletSolBalance;


pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

pub fn update_app_wallet_ui(
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
    text_ore_balance.sections[0].value = app_wallet.ore_balance.to_string() + " ORZ";
}

pub fn update_proof_account_ui(
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
    let amount = (proof_account_res.total_rewards as f64) / 10f64.powf(get_ore_decimals() as f64);
    text_total_rewards.sections[0].value = format!("Total Rewards: {}", amount);

    let mut text_claimable_rewards_query = set.p3();
    let mut text_claimable_rewards = text_claimable_rewards_query.single_mut();
    let amount = (proof_account_res.claimable_rewards as f64) / 10f64.powf(get_ore_decimals() as f64);
    text_claimable_rewards.sections[0].value = format!("Claimable Rewards: {}", amount);
}

pub fn update_treasury_account_ui(
    treasury_account_res: Res<TreasuryAccountResource>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextTreasuryBalance>>,
        Query<&mut Text, With<TextTreasuryAdmin>>,
        Query<&mut Text, With<TextTreasuryDifficulty>>,
        Query<&mut Text, With<TextTreasuryLastResetAt>>,
        Query<&mut Text, With<TextTreasuryRewardRate>>,
        Query<&mut Text, With<TextTreasuryTotalClaimedRewards>>,
        Query<&mut Text, With<TextTreasuryNeedEpochReset>>,
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
    let date_time = if let Some(dt) = DateTime::from_timestamp(treasury_account_res.last_reset_at, 0) {
        dt.to_string()
    } else {
        "Err".to_string()
    };

    text_3.sections[0].value = format!("Last Reset At: {}", date_time);

    let mut text_query_4 = set.p4();
    let mut text_4 = text_query_4.single_mut();
    text_4.sections[0].value =
        "Reward Rate: ".to_string() + &treasury_account_res.reward_rate.to_string();

    let mut text_query_5 = set.p5();
    let mut text_5 = text_query_5.single_mut();
    text_5.sections[0].value = "Total Claimed Rewards: ".to_string()
        + &treasury_account_res.total_claimed_rewards.to_string();

    let mut text_query_6 = set.p6();
    let mut text_6 = text_query_6.single_mut();
    let needs_reset_string = if treasury_account_res.need_epoch_reset {
        "TRUE"
    } else {
        "FALSE"
    };

    text_6.sections[0].value = format!("Need Epoch Reset: {}", needs_reset_string);
        
}

pub fn update_miner_status_ui(
    mut res: ResMut<MinerStatusResource>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextMinerStatusStatus>>,
        Query<&mut Text, With<TextMinerStatusCpuUsage>>,
        Query<&mut Text, With<TextMinerStatusRamUsage>>,
        Query<&mut Text, With<TextMinerStatusTime>>,
    )>,
    time: Res<Time>
) {
    res.sys_refresh_timer.tick(time.delta());

    if res.sys_refresh_timer.just_finished() {
        res.sys_info.refresh_all();
        res.sys_refresh_timer.reset();
    }

    let mut text_query_0 = set.p0();
    let mut text_0 = text_query_0.single_mut();
    text_0.sections[0].value = "Miner Status: ".to_string() + &res.miner_status.clone();

    let mut text_query_1 = set.p1();
    let mut text_1 = text_query_1.single_mut();

    let mut cpu_usage = 0.0;
    for (_index, cpu) in res.sys_info.cpus().iter().enumerate() {
        cpu_usage += cpu.cpu_usage();
    }

    let cpu_usage = format!("CPU Usage: {:.2}  % / {} %", cpu_usage, res.sys_info.cpus().len() * 100);
    text_1.sections[0].value = cpu_usage;

    let mut text_query_2 = set.p2();
    let mut text_2 = text_query_2.single_mut();
    let total_memory = res.sys_info.total_memory();
    let used_memory = res.sys_info.used_memory();
    let ram_usage = format!("RAM Usage: {} / {}", human_bytes(used_memory as f64), human_bytes(total_memory as f64));

    text_2.sections[0].value = ram_usage;

    let ts = get_unix_timestamp();
    let date_time = if let Some(dt) = DateTime::from_timestamp(ts as i64, 0) {
        dt.to_string()
    } else {
        "Err".to_string()
    };

    let mut text_query_3 = set.p3();
    let mut text_3 = text_query_3.single_mut();

    text_3.sections[0].value = format!("Time: {}", date_time);
}

pub fn update_current_tx_ui(
    mut res: ResMut<CurrentTx>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextCurrentTxSig>>,
        Query<&mut Text, With<TextCurrentTxStatus>>,
        Query<&mut Text, With<TextCurrentTxElapsed>>,
    )>,
) {
    let mut text_query_0 = set.p0();
    let mut text_0 = text_query_0.single_mut();
    if let Some((_tx, sig)) = res.tx_sig.clone() {

        text_0.sections[0].value = format!("Signature: {}", shorten_string(sig.to_string(), 10));
    } else {
        text_0.sections[0].value = "Signature: ".to_string() + "None";
    }

    let mut text_query_1 = set.p1();
    let mut text_1 = text_query_1.single_mut();
    text_1.sections[0].value = "Status: ".to_string() + &res.tx_status.status;

    if res.tx_status.status != "SUCCESS" && res.tx_status.status != "FAILED" {
        res.elapsed_seconds = res.elapsed_instant.elapsed().as_secs();
    }
    let mut text_query_2 = set.p2();
    let mut text_2 = text_query_2.single_mut();
    text_2.sections[0].value =
        "Elapsed: ".to_string() + &res.elapsed_seconds.to_string();
}

pub fn update_text_input_ui(
    mut active_text_query: Query<(&mut Text, &TextInput)>
) {
    for (mut active_text_text, text_input) in active_text_query.iter_mut() {
        if text_input.hidden {
            let text_len = text_input.text.len();
            let mut displayed_text = String::with_capacity(text_len);
            for _ in 0..text_len {
                displayed_text.push('*');
            }
            active_text_text.sections[0].value = displayed_text;
        } else {
            active_text_text.sections[0].value = text_input.text.clone();
        }
    }

}



pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb(
                    (1.0 - (value - 60.0) / (120.0 - 60.0)) as f32,
                    1.0,
                    0.0,
                )
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(
                    1.0,
                    ((value - 30.0) / (60.0 - 30.0)) as f32,
                    0.0,
                )
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

/// Toggle the FPS counter when pressing F12
pub fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FpsRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
