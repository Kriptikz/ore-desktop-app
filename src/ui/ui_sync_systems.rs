use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use chrono::DateTime;
use ore::ONE_DAY;

use crate::ore_utils::get_ore_decimals;
use crate::utils::{get_unix_timestamp, human_bytes, shorten_string};
use crate::AppWallet;
use crate::BussesResource;
use crate::MinerStatusResource;
use crate::OreAppState;
use crate::ProofAccountResource;
use crate::TreasuryAccountResource;

use super::components::ButtonCaptureTextInput;
use super::components::FpsRoot;
use super::components::FpsText;
use super::components::ScrollingList;
use super::components::TextBurnAmount;
use super::components::TextBus1;
use super::components::TextBus2;
use super::components::TextBus3;
use super::components::TextBus4;
use super::components::TextBus5;
use super::components::TextBus6;
use super::components::TextBus7;
use super::components::TextBus8;
use super::components::TextCurrentStake;
use super::components::TextCurrentChallenge;
use super::components::TextCursor;
use super::components::TextInput;
use super::components::TextLastClaimAt;
use super::components::TextLastHashAt;
use super::components::TextMinerStatusCpuUsage;
use super::components::TextMinerStatusRamUsage;
use super::components::TextMinerStatusStatus;
use super::components::TextMinerStatusThreads;
use super::components::TextMinerStatusTime;
use super::components::TextTotalHashes;
use super::components::TextTreasuryAdmin;
use super::components::TextTreasuryBalance;
use super::components::TextTreasuryLastResetAt;
use super::components::TextTreasuryRewardRate;
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
            if let Ok(query_node_parent) = query_node.get(parent.get()) {
                let container_height = query_node_parent.size().y;

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
    text_ore_balance.sections[0].value = app_wallet.ore_balance.to_string() + " ORE";
}

pub fn update_busses_ui(
    busses_res: Res<BussesResource>,
    miner_status: Res<MinerStatusResource>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextBus1>>,
        Query<&mut Text, With<TextBus2>>,
        Query<&mut Text, With<TextBus3>>,
        Query<&mut Text, With<TextBus4>>,
        Query<&mut Text, With<TextBus5>>,
        Query<&mut Text, With<TextBus6>>,
        Query<&mut Text, With<TextBus7>>,
        Query<&mut Text, With<TextBus8>>,
    )>,
) {
    if busses_res.busses.len() > 7 {
        let mut text_bus_query = set.p0();
        let mut text_bus_text = text_bus_query.single_mut();
        let rewards = (busses_res.busses[0].rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
        text_bus_text.sections[0].value = format!("Bus 1: {}    ", rewards);
        let selected_color = if miner_status.miner_status.as_str() == "PROCESSING" {
            Color::GREEN
        } else {
            Color::ORANGE
        };
        
        if busses_res.current_bus_id == 0 {
            text_bus_text.sections[0].style.color = selected_color.into();
        } else {
            text_bus_text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9);
        }

        let mut text_bus_query = set.p1();
        let mut text_bus_text = text_bus_query.single_mut();
        let rewards = (busses_res.busses[1].rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
        text_bus_text.sections[0].value = format!("Bus 2: {}    ", rewards);
        if busses_res.current_bus_id == 1 {
            text_bus_text.sections[0].style.color = selected_color.into();
        } else {
            text_bus_text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9);
        }

        let mut text_bus_query = set.p2();
        let mut text_bus_text = text_bus_query.single_mut();
        let rewards = (busses_res.busses[2].rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
        text_bus_text.sections[0].value = format!("Bus 3: {}    ", rewards);
        if busses_res.current_bus_id == 2 {
            text_bus_text.sections[0].style.color = selected_color.into();
        } else {
            text_bus_text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9);
        }


        let mut text_bus_query = set.p3();
        let mut text_bus_text = text_bus_query.single_mut();
        let rewards = (busses_res.busses[3].rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
        text_bus_text.sections[0].value = format!("Bus 4: {}    ", rewards);
        if busses_res.current_bus_id == 3 {
            text_bus_text.sections[0].style.color = selected_color.into();
        } else {
            text_bus_text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9);
        }

        let mut text_bus_query = set.p4();
        let mut text_bus_text = text_bus_query.single_mut();
        let rewards = (busses_res.busses[4].rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
        text_bus_text.sections[0].value = format!("Bus 5: {}    ", rewards);
        if busses_res.current_bus_id == 4 {
            text_bus_text.sections[0].style.color = selected_color.into();
        } else {
            text_bus_text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9);
        }

        let mut text_bus_query = set.p5();
        let mut text_bus_text = text_bus_query.single_mut();
        let rewards = (busses_res.busses[5].rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
        text_bus_text.sections[0].value = format!("Bus 6: {}    ", rewards);
        if busses_res.current_bus_id == 5 {
            text_bus_text.sections[0].style.color = selected_color.into();
        } else {
            text_bus_text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9);
        }

        let mut text_bus_query = set.p6();
        let mut text_bus_text = text_bus_query.single_mut();
        let rewards = (busses_res.busses[6].rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
        text_bus_text.sections[0].value = format!("Bus 7: {}    ", rewards);
        if busses_res.current_bus_id == 6 {
            text_bus_text.sections[0].style.color = selected_color.into();
        } else {
            text_bus_text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9);
        }

        let mut text_bus_query = set.p7();
        let mut text_bus_text = text_bus_query.single_mut();
        let rewards = (busses_res.busses[7].rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
        text_bus_text.sections[0].value = format!("Bus 8: {}    ", rewards);
        if busses_res.current_bus_id == 7 {
            text_bus_text.sections[0].style.color = selected_color.into();
        } else {
            text_bus_text.sections[0].style.color = Color::rgb(0.9, 0.9, 0.9);
        }
    }
}

pub fn update_proof_account_ui(
    proof_account_res: Res<ProofAccountResource>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextCurrentChallenge>>,
        Query<&mut Text, With<TextTotalHashes>>,
        Query<&mut Text, With<TextLastClaimAt>>,
        Query<&mut Text, With<TextCurrentStake>>,
        Query<&mut Text, With<TextLastHashAt>>,
        Query<&mut Text, With<TextBurnAmount>>,
    )>,
) {
    let mut text_current_hash_query = set.p0();
    let mut text_current_hash = text_current_hash_query.single_mut();
    text_current_hash.sections[0].value =
        proof_account_res.challenge.clone();

    let mut text_total_hashes_query = set.p1();
    let mut text_total_hashes = text_total_hashes_query.single_mut();
    text_total_hashes.sections[0].value =
        proof_account_res.total_hashes.to_string();

    let mut text_last_claim_at_query = set.p2();
    let mut text_last_claim_at = text_last_claim_at_query.single_mut();
    let last_claim_at = proof_account_res.last_claim_at;
    let last_claim_at_date_time =
        if let Some(dt) = DateTime::from_timestamp(last_claim_at, 0) {
            dt.to_string()
        } else {
            "Err".to_string()
        };
    text_last_claim_at.sections[0].value = format!("{}", last_claim_at_date_time);


    let mut text_claimable_rewards_query = set.p3();
    let mut text_claimable_rewards = text_claimable_rewards_query.single_mut();
    let amount =
        (proof_account_res.stake as f64) / 10f64.powf(get_ore_decimals() as f64);
    text_claimable_rewards.sections[0].value = format!("{}", amount);

    let mut text_query_3 = set.p4();
    let mut text_3 = text_query_3.single_mut();
    let date_time =
        if let Some(dt) = DateTime::from_timestamp(proof_account_res.last_hash_at, 0) {
            dt.to_string()
        } else {
            "Err".to_string()
        };

    text_3.sections[0].value = format!("{}", date_time);

    let amount = proof_account_res.stake;

    let clock = get_unix_timestamp() as i64;
    let t = last_claim_at.saturating_add(ONE_DAY);
    let burn_amount = if clock.lt(&t) {
        // Calculate burn amount
        (amount
            .saturating_mul(t.saturating_sub(clock) as u64)
            .saturating_div(ONE_DAY as u64) as f64) / 10f64.powf(get_ore_decimals() as f64)
    } else {
        0.0
    };
    let mut text_query_4 = set.p5();
    let mut text_4 = text_query_4.single_mut();
    text_4.sections[0].value = format!("{}", burn_amount);
}

pub fn update_treasury_account_ui(
    treasury_account_res: Res<TreasuryAccountResource>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextTreasuryBalance>>,
        Query<&mut Text, With<TextTreasuryAdmin>>,
        Query<&mut Text, With<TextTreasuryLastResetAt>>,
        Query<&mut Text, With<TextTreasuryRewardRate>>,
    )>,
) {
    let mut text_query_0 = set.p0();
    let mut text_0 = text_query_0.single_mut();
    text_0.sections[0].value = treasury_account_res.balance.clone();

    let mut text_query_1 = set.p1();
    let mut text_1 = text_query_1.single_mut();
    text_1.sections[0].value = treasury_account_res.admin.clone();


    if treasury_account_res.last_reset_at != 0 {
        let mut text_query_3 = set.p2();
        let mut text_3 = text_query_3.single_mut();

        let date_time =
            if let Some(dt) = DateTime::from_timestamp(treasury_account_res.last_reset_at, 0) {
                dt.to_string()
            } else {
                "Err".to_string()
            };
        text_3.sections[0].value = format!("{}", date_time);

    }

    let mut text_query_4 = set.p3();
    let mut text_4 = text_query_4.single_mut();
    text_4.sections[0].value =
        treasury_account_res.base_reward_rate.to_string();

}

pub fn update_miner_status_ui(
    mut res: ResMut<MinerStatusResource>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextMinerStatusStatus>>,
        Query<&mut Text, With<TextMinerStatusCpuUsage>>,
        Query<&mut Text, With<TextMinerStatusRamUsage>>,
        Query<&mut Text, With<TextMinerStatusTime>>,
        Query<&mut Text, With<TextMinerStatusThreads>>,
    )>,
    time: Res<Time>,
) {
    res.sys_refresh_timer.tick(time.delta());

    if res.sys_refresh_timer.just_finished() {
        res.sys_info.refresh_all();
        res.sys_refresh_timer.reset();
    }

    let mut text_query_0 = set.p0();
    let mut text_0 = text_query_0.single_mut();
    match res.miner_status.as_str() {
        "PROCESSING" => {
            if text_0.sections[0].style.color != Color::ORANGE {
                text_0.sections[0].style.color = Color::ORANGE;
            }
        },
        "MINING" => {
            if text_0.sections[0].style.color != Color::GREEN {
                text_0.sections[0].style.color = Color::GREEN;
            }
        },
        _ => {
            if text_0.sections[0].style.color != Color::RED {
                text_0.sections[0].style.color = Color::RED;
            }
        },
    }

    text_0.sections[0].value = res.miner_status.clone();

    let mut text_query_1 = set.p1();
    let mut text_1 = text_query_1.single_mut();

    let mut cpu_usage = 0.0;
    for (_index, cpu) in res.sys_info.cpus().iter().enumerate() {
        cpu_usage += cpu.cpu_usage();
    }

    let cpu_usage = format!(
        "{:.2}  % / {} %",
        cpu_usage,
        res.sys_info.cpus().len() * 100
    );
    text_1.sections[0].value = cpu_usage;

    let mut text_query_2 = set.p2();
    let mut text_2 = text_query_2.single_mut();
    let total_memory = res.sys_info.total_memory();
    let used_memory = res.sys_info.used_memory();
    let ram_usage = format!(
        "{} / {}",
        human_bytes(used_memory as f64),
        human_bytes(total_memory as f64)
    );

    text_2.sections[0].value = ram_usage;

    let ts = get_unix_timestamp();
    let date_time = if let Some(dt) = DateTime::from_timestamp(ts as i64, 0) {
        dt.to_string()
    } else {
        "Err".to_string()
    };

    let mut text_query_3 = set.p3();
    let mut text_3 = text_query_3.single_mut();

    text_3.sections[0].value = format!("{}", date_time);

    let mut text_query_4 = set.p4();
    let mut text_4 = text_query_4.single_mut();

    text_4.sections[0].value = format!("{}", res.miner_threads);
}

pub fn update_text_input_ui(mut active_text_query: Query<(&mut Text, &TextInput)>) {
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

pub fn update_active_text_input_cursor_vis(
    ore_app_state: Res<OreAppState>,
    captured_text_query: Query<(Entity, &Children), With<ButtonCaptureTextInput>>,
    mut text_cursor_query: Query<(Entity, &mut Visibility), With<TextCursor>>,
) {
    if let Some(active_text_entity) = ore_app_state.active_input_node {
        for (captured_text_entity, captured_text_children) in captured_text_query.iter() {
            if captured_text_entity == active_text_entity {
                for child in captured_text_children {
                    for (tc_entity, mut visibility) in text_cursor_query.iter_mut() {
                        if tc_entity == *child {
                            if *visibility != Visibility::Visible {
                                *visibility = Visibility::Visible;
                            } else {
                                *visibility = Visibility::Hidden;
                            }
                        } else {
                            if *visibility != Visibility::Hidden {
                                *visibility = Visibility::Hidden;
                            }
                        }
                    }
                }
            }
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
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
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
