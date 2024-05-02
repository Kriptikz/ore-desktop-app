
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use spl_associated_token_account::get_associated_token_address;
use crate::*;

use super::{components::*, styles::*};

pub fn button_update_sol_balance(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonUpdateSolOreBalances>),
    >,
    app_wallet: Res<AppWallet>,
    ore_app_state: Res<OreAppState>,
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
                        let reward_rate = (treasury_account.reward_rate as f64)
                            / 10f64.powf(ore::TOKEN_DECIMALS as f64);
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

pub fn button_copy_text(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
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

pub fn button_start_stop_mining(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonStartStopMining>),
    >,
) {
    for (entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                
                info!("Start Mining");
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
    text_ore_balance.sections[0].value = app_wallet.ore_balance.to_string() + " ORE";
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
    text_total_rewards.sections[0].value =
        "Total Rewards: ".to_string() + &proof_account_res.total_rewards.to_string();

    let mut text_claimable_rewards_query = set.p3();
    let mut text_claimable_rewards = text_claimable_rewards_query.single_mut();
    text_claimable_rewards.sections[0].value =
        "Claimable Rewards: ".to_string() + &proof_account_res.claimable_rewards.to_string();
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
    text_3.sections[0].value =
        "Last Reset At: ".to_string() + &treasury_account_res.last_reset_at.to_string();

    let mut text_query_4 = set.p4();
    let mut text_4 = text_query_4.single_mut();
    text_4.sections[0].value =
        "Reward Rate: ".to_string() + &treasury_account_res.reward_rate.to_string();

    let mut text_query_5 = set.p5();
    let mut text_5 = text_query_5.single_mut();
    text_5.sections[0].value = "Total Claimed Rewards: ".to_string()
        + &treasury_account_res.total_claimed_rewards.to_string();
}

pub fn update_miner_status_ui(
    res: Res<MinerStatusResource>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextMinerStatusStatus>>,
        Query<&mut Text, With<TextMinerStatusCpuUsage>>,
        Query<&mut Text, With<TextMinerStatusRamUsage>>,
    )>,
) {
    let mut text_query_0 = set.p0();
    let mut text_0 = text_query_0.single_mut();
    text_0.sections[0].value = "Miner Status: ".to_string() + &res.miner_status.clone();

    let mut text_query_1 = set.p1();
    let mut text_1 = text_query_1.single_mut();
    text_1.sections[0].value = "CPU Usage: ".to_string() + &res.cpu_usage.to_string();

    let mut text_query_2 = set.p2();
    let mut text_2 = text_query_2.single_mut();
    text_2.sections[0].value =
        "RAM Usage: ".to_string() + &res.ram_usage.to_string();
}

pub fn spawn_copyable_text(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    copy_text: String,
    display_text: String,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        right: Val::Px(10.0),
                        bottom: Val::Px(0.0),
                    },
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
                        font_size: FONT_SIZE,
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
                            width: Val::Px(30.0),
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
                    ButtonCopyText,
                    Name::new("ButtonCopyText"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Copy",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FONT_SIZE,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}