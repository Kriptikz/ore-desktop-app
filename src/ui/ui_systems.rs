
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use chrono::NaiveDateTime;
use orz::TOKEN_DECIMALS;
use crate::*;

use super::{components::*, styles::*};

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
    mut ev_start_stop_mining: EventWriter<EventStartStopMining>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonStartStopMining>),
    >,
) {
    for (_entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                
                ev_start_stop_mining.send(EventStartStopMining);
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

pub fn button_reset_epoch(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonResetEpoch>),
    >,
    mut event_writer: EventWriter<EventResetEpoch>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                event_writer.send(EventResetEpoch);
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

pub fn button_claim_ore_rewards(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonClaimOreRewards>),
    >,
    mut event_writer: EventWriter<EventClaimOreRewards>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                event_writer.send(EventClaimOreRewards);
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
    let amount = (proof_account_res.total_rewards as f64) / 10f64.powf(TOKEN_DECIMALS as f64);
    text_total_rewards.sections[0].value = format!("Total Rewards: {}", amount);

    let mut text_claimable_rewards_query = set.p3();
    let mut text_claimable_rewards = text_claimable_rewards_query.single_mut();
    let amount = (proof_account_res.claimable_rewards as f64) / 10f64.powf(TOKEN_DECIMALS as f64);
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
    let date_time = NaiveDateTime::from_timestamp(treasury_account_res.last_reset_at, 0);
    text_3.sections[0].value =
        "Last Reset At: ".to_string() + &date_time.to_string();

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
    for (index, cpu) in res.sys_info.cpus().iter().enumerate() {
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
    let date_time = NaiveDateTime::from_timestamp(ts as i64, 0);
    let mut text_query_3 = set.p3();
    let mut text_3 = text_query_3.single_mut();

    text_3.sections[0].value = format!("Time: {}", date_time.to_string());
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
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
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

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
pub struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
pub struct FpsText;

pub fn setup_fps_counter(
    mut commands: Commands,
) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands.spawn((
        FpsRoot,
        NodeBundle {
            // give it a dark background for readability
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            // make it "always on top" by setting the Z index to maximum
            // we want it to be displayed over all other UI
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                // position it at the top-right corner
                // 1% away from the top window edge
                right: Val::Percent(1.),
                top: Val::Percent(1.),
                // set bottom/left to Auto, so it can be
                // automatically sized depending on the text
                bottom: Val::Auto,
                left: Val::Auto,
                // give it some padding for readability
                padding: UiRect::all(Val::Px(4.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    )).id();
    // create our text
    let text_fps = commands.spawn((
        FpsText,
        TextBundle {
            // use two sections, so it is easy to update just the number
            text: Text::from_sections([
                TextSection {
                    value: "FPS: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    }
                },
                TextSection {
                    value: " N/A".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    }
                },
            ]),
            ..Default::default()
        },
    )).id();
    commands.entity(root).push_children(&[text_fps]);
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

const SUFFIX: [&str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

const UNIT: f64 = 1024.0;

/// Converts bytes to human-readable values
pub fn human_bytes<T: Into<f64>>(bytes: T) -> String {
    let size = bytes.into();

    if size <= 0.0 {
        return "0 B".to_string();
    }

    let base = size.log10() / UNIT.log10();

    let result = format!("{:.1}", UNIT.powf(base - base.floor()),)
        .trim_end_matches(".0")
        .to_owned();

    // Add suffix
    [&result, SUFFIX[base.floor() as usize]].join(" ")
}

pub fn button_lock(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonLock>),
    >,
     mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                next_state.set(GameState::Locked);
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

pub fn button_unlock(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ButtonUnlock>),
    >,
     mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                next_state.set(GameState::Mining);
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