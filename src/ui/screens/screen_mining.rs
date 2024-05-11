use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};
use solana_sdk::signer::Signer;

use crate::{
    ui::{
        components::{ButtonStakeOre, TextBus1, TextLastHashAt, TextMinerStatusThreads}, spawn_utils::spawn_copyable_text, styles::{
            hex_black, BUTTON_CLAIM, BUTTON_RESET_EPOCH, BUTTON_STAKE, BUTTON_START_MINING, CURRENT_TX_STATUS_BACKGROUND, FONT_ROBOTO, FONT_ROBOTO_MEDIUM, FONT_SIZE_TITLE, PROOF_ACCOUNT_BACKGROUND, SYSTEM_OVERVIEW_BACKGROUND, TREASURY_BACKGROUND, TX_RESULTS_BACKGROUND
        }
    },
    utils::shorten_string,
    AppWallet,
};

use crate::ui::{
    components::{
        BaseScreenNode, ButtonClaimOreRewards, ButtonLock, ButtonResetEpoch, ButtonStartStopMining,
        MovingScrollPanel, ScrollingList, TextCurrentChallenge, TextCurrentStake,
        TextCurrentTxElapsed, TextCurrentTxSig, TextCurrentTxStatus, TextMinerStatusCpuUsage,
        TextMinerStatusRamUsage, TextMinerStatusStatus, TextMinerStatusTime, TextTotalHashes,
        TextTotalRewards, TextTreasuryAdmin, TextTreasuryBalance, TextTreasuryDifficulty,
        TextTreasuryLastResetAt, TextTreasuryNeedEpochReset, TextTreasuryRewardRate,
        TextTreasuryTotalClaimedRewards, TextWalletOreBalance, TextWalletSolBalance,
    },
    styles::{FONT_SIZE, NORMAL_BUTTON},
};

pub fn spawn_mining_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_wallet: Res<AppWallet>,
) {
    let full_addr = app_wallet.wallet.pubkey().to_string();
    let wallet_str = shorten_string(full_addr, 10);
    let sol_balance_str = app_wallet.sol_balance.to_string();
    let ore_balance_str = app_wallet.ore_balance.to_string();
    commands
        .spawn((
            NodeBundle {
                background_color: hex_black().into(),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("Screen Node"),
            BaseScreenNode,
        ))
        .with_children(|parent| {
            // Top Left Ore Logo
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        margin: UiRect {
                            top: Val::Px(10.0),
                            left: Val::Px(50.0),
                            right: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(36.0),
                                    height: Val::Px(36.0),
                                    ..default()
                                },
                                // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            UiImage::new(asset_server.load("design_1/ore_icon_small.png")),
                        ))
                        .with_children(|parent| {
                            // alt text
                            // This UI node takes up no space in the layout and the `Text` component is used by the accessibility module
                            // and is not rendered.
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        display: Display::None,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                Text::from_section("Ore logo", TextStyle::default()),
                            ));
                        });
                });
            // backbround ore logo
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        margin: UiRect {
                            top: Val::Px(100.0),
                            left: Val::Percent(60.0),
                            right: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                z_index: ZIndex::Global(-1),
                                style: Style {
                                    width: Val::Px(430.0),
                                    height: Val::Px(430.0),
                                    ..default()
                                },
                                // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            UiImage::new(asset_server.load("design_1/ore_icon_big.png")),
                        ))
                        .with_children(|parent| {
                            // alt text
                            // This UI node takes up no space in the layout and the `Text` component is used by the accessibility module
                            // and is not rendered.
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        display: Display::None,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                Text::from_section("Ore logo", TextStyle::default()),
                            ));
                        });
                });

            // Left screen node
            parent
                .spawn((
                    NodeBundle {
                        // border_color: Color::BLUE.into(),
                        style: Style {
                            width: Val::Percent(70.0),
                            height: Val::Percent(90.0),
                            flex_direction: FlexDirection::Column,
                            // border: UiRect::all(Val::Px(1.0)),
                            margin: UiRect::top(Val::Px(50.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Left Screen Node"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                background_color: Color::WHITE.into(),
                                // border_color: Color::ORANGE.into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(50.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.0)),
                                    padding: UiRect {
                                        top: Val::Px(10.0),
                                        left: Val::Px(0.0),
                                        right: Val::Px(0.0),
                                        bottom: Val::Px(15.0),
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                            UiImage::new(asset_server.load(TREASURY_BACKGROUND)),
                            Name::new("Left Top Node"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        background_color: Color::WHITE.into(),
                                        // border_color: Color::ORANGE.into(),
                                        style: Style {
                                            width: Val::Percent(90.0),
                                            flex_direction: FlexDirection::Column,
                                            border: UiRect::all(Val::Px(1.0)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    UiImage::new(asset_server.load(TREASURY_BACKGROUND)),
                                    Name::new("Left Top Node Title"),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style { ..default() },
                                                ..default()
                                            },
                                            //UiImage::new(asset_server.load(TX_RESULTS_BACKGROUND)),
                                            Name::new("Treasury Title node"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Treasury",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE_TITLE,
                                                        color: Color::hex("#FFFFFF").unwrap(),
                                                    },
                                                ),
                                                Name::new("TextTitleTreasury"),
                                            ));
                                        });
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        background_color: Color::WHITE.into(),
                                        // border_color: Color::ORANGE.into(),
                                        style: Style {
                                            width: Val::Percent(90.0),
                                            height: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::SpaceBetween,
                                            border: UiRect::all(Val::Px(1.0)),
                                            padding: UiRect {
                                                top: Val::Px(10.0),
                                                left: Val::Px(0.0),
                                                right: Val::Px(0.0),
                                                bottom: Val::Px(15.0),
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    UiImage::new(asset_server.load(TREASURY_BACKGROUND)),
                                    Name::new("Left Top Node Content"),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                //background_color: Color::WHITE.into(),
                                                // border_color: Color::ORANGE.into(),
                                                style: Style {
                                                    // width: Val::Percent(70.0),
                                                    height: Val::Percent(100.0),
                                                    flex_direction: FlexDirection::Row,
                                                    border: UiRect::all(Val::Px(1.0)),
                                                    row_gap: Val::Px(20.0),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            //UiImage::new(asset_server.load(TX_RESULTS_BACKGROUND)),
                                            Name::new("Treasury Account node"),
                                        ))
                                        .with_children(|parent| {
                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            // justify_content: JustifyContent::Center,
                                                            flex_direction: FlexDirection::Column,
                                                            row_gap: Val::Px(15.0),
                                                            // height: Val::Px(50.0),
                                                            // align_items: AlignItems::Center,
                                                            // margin: UiRect {
                                                            //     top: Val::Px(10.0),
                                                            //     left: Val::Px(0.0),
                                                            //     right: Val::Px(0.0),
                                                            //     bottom: Val::Px(0.0),
                                                            // },
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("Text Titles"),
                                                ))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Balance:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::hex("#FFFFFF")
                                                                    .unwrap(),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryBalance"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Admin:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryAdmin"),
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Last Reset At:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryLastResetAt"),
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Need Epoch Reset:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryNeedEpochReset"),
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Base Reward Rate:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryRewardRate"),
                                                    ));
                                                });
                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            flex_direction: FlexDirection::Column,
                                                            row_gap: Val::Px(15.0),
                                                            padding: UiRect::left(Val::Px(5.0)),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("Text Values"),
                                                ))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::hex("#FFFFFF")
                                                                    .unwrap(),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryBalance"),
                                                        TextTreasuryBalance,
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryAdmin"),
                                                        TextTreasuryAdmin,
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryLastResetAt"),
                                                        TextTreasuryLastResetAt,
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryNeedEpochReset"),
                                                        TextTreasuryNeedEpochReset,
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTreasuryRewardRate"),
                                                        TextTreasuryRewardRate,
                                                    ));
                                                });
                                        });
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                // background_color: Color::WHITE.into(),
                                                // border_color: Color::ORANGE.into(),
                                                style: Style {
                                                    width: Val::Percent(30.0),
                                                    height: Val::Percent(50.0),
                                                    flex_direction: FlexDirection::Column,
                                                    padding: UiRect::right(Val::Px(10.0)),
                                                    border: UiRect::all(Val::Px(1.0)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            // UiImage::new(asset_server.load(TX_RESULTS_BACKGROUND)),
                                            Name::new("Wallet Info Node"),
                                        ))
                                        .with_children(|parent| {
                                            spawn_copyable_text(
                                                parent,
                                                &asset_server,
                                                app_wallet.wallet.pubkey().to_string(),
                                                wallet_str,
                                            );
                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            flex_direction: FlexDirection::Column,
                                                            align_items: AlignItems::End,
                                                            padding: UiRect {
                                                                left: Val::Px(0.0),
                                                                right: Val::Px(20.0),
                                                                top: Val::Px(0.0),
                                                                bottom: Val::Px(0.0),
                                                            },
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("WalletBalance Nodes"),
                                                ))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            &(sol_balance_str + " SOL"),
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        TextWalletSolBalance,
                                                        Name::new("TextWalletSolBalance"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            &(ore_balance_str + " ORE"),
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        TextWalletOreBalance,
                                                        Name::new("TextWalletOreBalance"),
                                                    ));
                                                });
                                        });
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                border_color: Color::ORANGE.into(),
                                                style: Style {
                                                    position_type: PositionType::Absolute,
                                                    width: Val::Percent(100.0),
                                                    height: Val::Percent(10.0),
                                                    flex_direction: FlexDirection::Row,
                                                    align_self: AlignSelf::End,
                                                    border: UiRect::all(Val::Px(1.0)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Text Busses Node"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Bus 1: loading...",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextBus1,
                                                Name::new("TextBus1"),
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Bus 1: loading...",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextBus1,
                                                Name::new("TextBus1"),
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Bus 1: loading...",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextBus1,
                                                Name::new("TextBus1"),
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Bus 1: loading...",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextBus1,
                                                Name::new("TextBus1"),
                                            ));
                                        });
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    position_type: PositionType::Absolute,
                                                    width: Val::Percent(100.0),
                                                    height: Val::Percent(10.0),
                                                    flex_direction: FlexDirection::Row,
                                                    align_items: AlignItems::End,
                                                    align_self: AlignSelf::End,
                                                    justify_content: JustifyContent::End,
                                                    border: UiRect::all(Val::Px(1.0)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Top Left Buttons Node"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        width: Val::Px(164.53),
                                                        height: Val::Px(38.0),
                                                        ..default()
                                                    },
                                                    image: UiImage::new(
                                                        asset_server.load(BUTTON_START_MINING),
                                                    ),
                                                    ..default()
                                                },
                                                ButtonStartStopMining,
                                                Name::new("ButtonStartMining"),
                                            ));
                                            parent.spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        width: Val::Px(164.53),
                                                        height: Val::Px(38.0),
                                                        ..default()
                                                    },
                                                    image: UiImage::new(
                                                        asset_server.load(BUTTON_RESET_EPOCH),
                                                    ),
                                                    ..default()
                                                },
                                                ButtonResetEpoch,
                                                Name::new("ButtonResetEpoch"),
                                            ));
                                        });
                                });
                        });
                    parent
                        .spawn((
                            NodeBundle {
                                background_color: Color::WHITE.into(),
                                // border_color: Color::ORANGE.into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(45.0),
                                    flex_direction: FlexDirection::Column,
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            UiImage::new(asset_server.load(TX_RESULTS_BACKGROUND)),
                            Name::new("Left Bottom Node"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            padding: UiRect::all(Val::Percent(2.0)),
                                            min_height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("TxResultList Node"),
                                ))
                                .with_children(|parent| {
                                    // Title
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    flex_direction: FlexDirection::Row,
                                                    align_items: AlignItems::Center,
                                                    justify_content: JustifyContent::SpaceAround,
                                                    width: Val::Percent(100.0),
                                                    ..default()
                                                },
                                                background_color: Color::rgb(0.15, 0.15, 0.15)
                                                    .into(),
                                                ..default()
                                            },
                                            Name::new("Tx Status Title"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((TextBundle::from_section(
                                                "Tx Type",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO),
                                                    font_size: FONT_SIZE_TITLE,
                                                    ..default()
                                                },
                                            ),));
                                            parent.spawn((TextBundle::from_section(
                                                "Signature",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO),
                                                    font_size: FONT_SIZE,
                                                    ..default()
                                                },
                                            ),));
                                            parent.spawn((TextBundle::from_section(
                                                "Tx Time",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO),
                                                    font_size: FONT_SIZE,
                                                    ..default()
                                                },
                                            ),));
                                            parent.spawn((TextBundle::from_section(
                                                "Hash Time - Difficulty",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO),
                                                    font_size: FONT_SIZE,
                                                    ..default()
                                                },
                                            ),));
                                            parent.spawn((TextBundle::from_section(
                                                "Status",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO),
                                                    font_size: FONT_SIZE,
                                                    ..default()
                                                },
                                            ),));
                                        });
                                    // List with hidden overflow
                                    // TODO: look into lazy loading items
                                    // TODO: add the scroll bar on the right
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    flex_direction: FlexDirection::Column,
                                                    align_self: AlignSelf::Stretch,
                                                    min_height: Val::Percent(90.0),
                                                    max_height: Val::Percent(90.0),
                                                    overflow: Overflow::clip_y(),
                                                    ..default()
                                                },
                                                background_color: Color::rgb(0.10, 0.10, 0.10)
                                                    .into(),
                                                ..default()
                                            },
                                            Name::new("ScrollingList Node"),
                                        ))
                                        .with_children(|parent| {
                                            // Moving panel
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        flex_direction: FlexDirection::Column,
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                ScrollingList::default(),
                                                AccessibilityNode(NodeBuilder::new(Role::List)),
                                                MovingScrollPanel,
                                                Name::new("MovingScrollPanel"),
                                            ));
                                        });
                                });
                        });
                });

            // right screen node
            parent
                .spawn((
                    NodeBundle {
                        // border_color: Color::RED.into(),
                        style: Style {
                            width: Val::Percent(26.0),
                            height: Val::Percent(90.0),
                            flex_direction: FlexDirection::Column,
                            // border: UiRect::all(Val::Px(1.0)),
                            margin: UiRect::top(Val::Px(40.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Right Screen Node"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                background_color: Color::WHITE.into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(30.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            UiImage::new(asset_server.load(SYSTEM_OVERVIEW_BACKGROUND)),
                            Name::new("System Overview Node"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        background_color: Color::WHITE.into(),
                                        // border_color: Color::ORANGE.into(),
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(60.0),
                                            flex_direction: FlexDirection::Column,
                                            padding: UiRect::all(Val::Px(10.0)),
                                            margin: UiRect::left(Val::Px(35.0)),
                                            // border: UiRect::all(Val::Px(1.0)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    UiImage::new(asset_server.load(SYSTEM_OVERVIEW_BACKGROUND)),
                                    Name::new("System Overview"),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    flex_direction: FlexDirection::Row,
                                                    align_items: AlignItems::Start,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Mining Status Node"),
                                        ))
                                        .with_children(|parent| {
                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            flex_direction: FlexDirection::Column,
                                                            row_gap: Val::Px(10.0),
                                                            align_items: AlignItems::Start,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("Mining Status Field Headers"),
                                                ))
                                                .with_children(|parent| {
                                                    // miner status field headers
                                                    parent.spawn((TextBundle::from_section(
                                                        format!("Miner Status:"),
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),));

                                                    parent.spawn((TextBundle::from_section(
                                                        format!("Time:"),
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),));

                                                    parent.spawn((TextBundle::from_section(
                                                        format!("Threads:"),
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),));

                                                    parent.spawn((TextBundle::from_section(
                                                        format!("CPU Usage:"),
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),));
                                                    parent.spawn((TextBundle::from_section(
                                                        format!("RAM Usage:"),
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),));
                                                });

                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            flex_direction: FlexDirection::Column,
                                                            row_gap: Val::Px(10.0),
                                                            align_items: AlignItems::Start,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("Mining Status Field Values"),
                                                ))
                                                .with_children(|parent| {
                                                    // miner status field values
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            format!("STOPPED"),
                                                            TextStyle {
                                                                color: Color::RED.into(),
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                ..default()
                                                            },
                                                        ),
                                                        TextMinerStatusStatus,
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            format!("Loading..."),
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                ..default()
                                                            },
                                                        ),
                                                        TextMinerStatusTime,
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            format!("Loading..."),
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                ..default()
                                                            },
                                                        ),
                                                        TextMinerStatusThreads,
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            format!("2%"),
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                ..default()
                                                            },
                                                        ),
                                                        TextMinerStatusCpuUsage,
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            format!("0.2 GB / 6.0 GB"),
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                ..default()
                                                            },
                                                        ),
                                                        TextMinerStatusRamUsage,
                                                    ));
                                                });
                                        });
                                });
                        });
                    parent
                        .spawn((
                            NodeBundle {
                                background_color: Color::WHITE.into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(60.0),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::top(Val::Px(10.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                                ..default()
                            },
                            UiImage::new(asset_server.load(PROOF_ACCOUNT_BACKGROUND)),
                            Name::new("Right Bottom Node"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(50.0),
                                            flex_direction: FlexDirection::Column,
                                            padding: UiRect {
                                                top: Val::Px(10.0),
                                                left: Val::Px(10.0),
                                                right: Val::Px(0.0),
                                                bottom: Val::Px(0.0),
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Proof Account Node"),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    flex_direction: FlexDirection::Row,
                                                    width: Val::Percent(100.0),
                                                    height: Val::Percent(100.0),
                                                    align_items: AlignItems::Start,
                                                    row_gap: Val::Px(10.0),
                                                    margin: UiRect::left(Val::Px(5.0)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Proof Account Title"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Proof Account",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                                        font_size: FONT_SIZE_TITLE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                Name::new("TextTitleProofAccount"),
                                            ));
                                        });
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    flex_direction: FlexDirection::Row,
                                                    width: Val::Percent(90.0),
                                                    height: Val::Percent(90.0),
                                                    margin: UiRect {
                                                        top: Val::Px(0.0),
                                                        left: Val::Px(5.0),
                                                        right: Val::Px(5.0),
                                                        bottom: Val::Px(0.0),
                                                    },
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Proof Account Content"),
                                        ))
                                        .with_children(|parent| {
                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            flex_direction: FlexDirection::Column,
                                                            align_items: AlignItems::Start,
                                                            width: Val::Percent(32.0),
                                                            height: Val::Percent(50.0),
                                                            row_gap: Val::Px(6.0),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("Proof Account Field Headers"),
                                                ))
                                                .with_children(|parent| {
                                                    parent
                                                        .spawn((
                                                            NodeBundle {
                                                                style: Style {
                                                                    width: Val::Px(60.0),
                                                                    height: Val::Px(37.0),
                                                                    ..default()
                                                                },
                                                                ..default()
                                                            },
                                                            Name::new("Current Hash Text Wrapper"),
                                                        ))
                                                        .with_children(|parent| {
                                                            parent.spawn((
                                                                TextBundle::from_section(
                                                                    "Current Challenge:",
                                                                    TextStyle {
                                                                        font: asset_server
                                                                            .load(FONT_ROBOTO),
                                                                        font_size: FONT_SIZE,
                                                                        color: Color::rgb(
                                                                            0.9, 0.9, 0.9,
                                                                        ),
                                                                    },
                                                                ),
                                                                Name::new("TextCurrentChallenge"),
                                                            ));
                                                        });
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Last Hash At:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextLastHashAt"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Total Hashes:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTotalHashes"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            " - Rewards:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::hex("#0ECF86").unwrap(),
                                                            },
                                                        ),
                                                        Name::new("TextRewardsTitle"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Total:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTotalRewards"),
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Staked:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextCurrentStake"),
                                                    ));
                                                });
                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            flex_direction: FlexDirection::Column,
                                                            width: Val::Percent(65.0),
                                                            height: Val::Percent(50.0),
                                                            row_gap: Val::Px(6.0),
                                                            align_items: AlignItems::Start,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("Proof Account Field Values"),
                                                ))
                                                .with_children(|parent| {
                                                    parent
                                                        .spawn((
                                                            NodeBundle {
                                                                style: Style {
                                                                    width: Val::Px(200.0),
                                                                    height: Val::Px(37.0),
                                                                    ..default()
                                                                },
                                                                ..default()
                                                            },
                                                            Name::new("Current Hash Text Wrapper"),
                                                        ))
                                                        .with_children(|parent| {
                                                            parent.spawn((
                                                                TextBundle {
                                                                    text: Text {
                                                                        sections: vec![
                                                                            TextSection {
                                                                                value: "loading...".to_string(),
                                                                                style: TextStyle {
                                                                                    font: asset_server
                                                                                        .load(FONT_ROBOTO),
                                                                                    font_size: FONT_SIZE,
                                                                                    color: Color::rgb(
                                                                                        0.9, 0.9, 0.9,
                                                                                    ),
                                                                                },
                                                                                ..Default::default()
                                                                            }
                                                                        ],
                                                                        linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
                                                                        ..Default::default()
                                                                    },
                                                                    ..Default::default()
                                                                },
                                                                Name::new("TextCurrentChallenge"),
                                                                TextCurrentChallenge,
                                                            ));
                                                        });
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextLastHashAt"),
                                                        TextLastHashAt,
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTotalHashes"),
                                                        TextTotalHashes,
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::hex("#0ECF86").unwrap(),
                                                            },
                                                        ).with_style(Style {
                                                            height: Val::Px(FONT_SIZE),
                                                            ..Default::default()
                                                        }),
                                                        Name::new("TextRewardsTitle"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextTotalRewards"),
                                                        TextTotalRewards,
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextCurrentStake"),
                                                        TextCurrentStake,
                                                    ));
                                                });
                                        });
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    flex_direction: FlexDirection::Row,
                                                    width: Val::Percent(100.0),
                                                    height: Val::Percent(100.0),
                                                    align_items: AlignItems::Start,
                                                    row_gap: Val::Px(10.0),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Proof Account Buttons"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        width: Val::Px(108.0),
                                                        height: Val::Px(37.0),
                                                        align_self: AlignSelf::Center,
                                                        justify_self: JustifySelf::Center,
                                                        ..default()
                                                    },
                                                    image: UiImage::new(
                                                        asset_server.load(BUTTON_CLAIM),
                                                    ),
                                                    ..default()
                                                },
                                                ButtonClaimOreRewards,
                                                Name::new("ButtonClaimOreRewards"),
                                            ));
                                            parent.spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        width: Val::Px(218.0),
                                                        height: Val::Px(37.0),
                                                        align_self: AlignSelf::Center,
                                                        justify_self: JustifySelf::Center,
                                                        ..default()
                                                    },
                                                    image: UiImage::new(
                                                        asset_server.load(BUTTON_STAKE),
                                                    ),
                                                    ..default()
                                                },
                                                ButtonStakeOre,
                                                Name::new("ButtonStakeOre"),
                                            ));
                                        });
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        background_color: Color::WHITE.into(),
                                        // border_color: Color::ORANGE.into(),
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(30.0),
                                            flex_direction: FlexDirection::Column,
                                            padding: UiRect {
                                                top: Val::Px(10.0),
                                                right: Val::Px(1.0),
                                                left: Val::Px(16.0),
                                                bottom: Val::Px(1.0),
                                            },
                                            row_gap: Val::Px(6.0),
                                            // border: UiRect::all(Val::Px(1.0)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    UiImage::new(asset_server.load(CURRENT_TX_STATUS_BACKGROUND)),
                                    Name::new("Current Tx Status Node"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((TextBundle::from_section(
                                        format!("Current Transaction"),
                                        TextStyle {
                                            font: asset_server.load(FONT_ROBOTO),
                                            font_size: FONT_SIZE_TITLE,
                                            ..default()
                                        },
                                    ),));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Signature:   COPY"),
                                            TextStyle {
                                                font: asset_server.load(FONT_ROBOTO),
                                                font_size: FONT_SIZE,
                                                ..default()
                                            },
                                        ),
                                        TextCurrentTxSig,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Status:"),
                                            TextStyle {
                                                font: asset_server.load(FONT_ROBOTO),
                                                font_size: FONT_SIZE,
                                                ..default()
                                            },
                                        ),
                                        TextCurrentTxStatus,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Elapsed:"),
                                            TextStyle {
                                                font: asset_server.load(FONT_ROBOTO),
                                                font_size: FONT_SIZE,
                                                ..default()
                                            },
                                        ),
                                        TextCurrentTxElapsed,
                                    ));
                                });
                        });
                });
        });
}
