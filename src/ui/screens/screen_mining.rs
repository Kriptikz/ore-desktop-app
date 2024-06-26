use std::time::Duration;

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
        components::{
            AutoScrollCheckIcon, ButtonAutoScroll, ButtonCooldownSpinner, ButtonRequestAirdrop, ButtonStakeOre, SpinnerIcon, TextBurnAmount, TextBus1, TextBus2, TextBus3, TextBus4, TextBus5, TextBus6, TextBus7, TextBus8, TextLastClaimAt, TextLastHashAt, TextMinerStatusThreads, TxPopUpArea
        },
        spawn_utils::spawn_copyable_text,
        styles::{
            hex_black, BUTTON_CLAIM, BUTTON_GREEN_MEDIUM, BUTTON_RED_MEDIUM, BUTTON_STAKE, CHECKBOX, CHECK_ICON, FONT_ROBOTO, FONT_ROBOTO_MEDIUM, FONT_SIZE_TITLE, PROOF_ACCOUNT_BACKGROUND, SPINNER_ICON, SYSTEM_OVERVIEW_BACKGROUND, TOGGLE_OFF, TREASURY_BACKGROUND, TX_RESULTS_BACKGROUND
        },
    },
    utils::shorten_string,
    AppWallet, AppConfig,
};

use crate::ui::{
    components::{
        BaseScreenNode, ButtonClaimOreRewards, MovingScrollPanel, ScrollingList,
        TextCurrentChallenge, TextCurrentStake, TextMinerStatusCpuUsage, TextMinerStatusRamUsage,
        TextMinerStatusStatus, TextMinerStatusTime, TextTotalHashes, TextTreasuryAdmin, TextTreasuryBalance, TextTreasuryLastResetAt,
        TextTreasuryRewardRate, TextWalletOreBalance, TextWalletSolBalance, ToggleAutoMine,
    },
    styles::FONT_SIZE,
};

pub fn spawn_mining_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    address: String,
    sol_balance: f64,
    ore_balance: f64,
    config: AppConfig,
) {
    let full_addr = address.clone();
    let wallet_str = shorten_string(full_addr, 10);
    let sol_balance_str = sol_balance.to_string();
    let ore_balance_str = ore_balance.to_string();
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
            if config.is_devnet {
                // devnet airdrop
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Px(250.0),
                            height: Val::Px(100.0),
                            margin: UiRect {
                                top: Val::Px(-20.0),
                                left: Val::Percent(40.0),
                                right: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                            },
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        z_index: ZIndex::Global(14),
                        ..default()
                    },
                    Name::new("Devnet Node"),
                )).with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "DEVNET",
                            TextStyle {
                                font: asset_server.load(FONT_ROBOTO),
                                font_size: FONT_SIZE_TITLE,
                                color: Color::YELLOW.into()
                            },
                        ),
                        Name::new("TextDevnet"),
                    ));
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(129.0),
                                height: Val::Px(37.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            image: UiImage::new(
                                asset_server.load(BUTTON_GREEN_MEDIUM),
                            ),
                            ..default()
                        },
                        ButtonRequestAirdrop {
                            clicked: false,
                            timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
                        },
                        Name::new("ButtonRequestAirdrop"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Airdrop",
                                TextStyle {
                                    font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                    font_size: FONT_SIZE,
                                    color: Color::BLACK.into(),
                                },
                            ),
                            Name::new("TextAirdrop"),
                        ));
                        parent.spawn((
                            NodeBundle {
                                visibility: Visibility::Hidden,
                                background_color: Color::WHITE.into(),
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    width: Val::Px(24.0),
                                    height: Val::Px(24.0),
                                    margin: UiRect::left(Val::Px(160.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            Name::new("SpinnerIcon"),
                            UiImage::new(asset_server.load(SPINNER_ICON)),
                            SpinnerIcon,
                            ButtonCooldownSpinner,
                        ));
                    });
                });
            }
            // pop-up area
            parent.spawn((
                NodeBundle {
                    z_index: ZIndex::Global(15),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        padding: UiRect {
                            top: Val::Px(10.0),
                            right: Val::Px(25.0),
                            left: Val::Px(0.0),
                            bottom: Val::Px(10.0),
                        },
                        position_type: PositionType::Absolute,
                        align_content: AlignContent::End,
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    ..default()
                },
                Name::new("TxPopUpScreen Node"),
            )).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::ColumnReverse,
                            row_gap: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                    TxPopUpArea,
                    Name::new("TxPopUpArea"),
                ));
            });
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
                            height: Val::Percent(93.0),
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
                                                address,
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
                                                // border_color: Color::ORANGE.into(),
                                                style: Style {
                                                    position_type: PositionType::Absolute,
                                                    width: Val::Percent(50.0),
                                                    height: Val::Percent(31.0),
                                                    flex_direction: FlexDirection::Row,
                                                    // flex_wrap: FlexWrap::Wrap,
                                                    align_self: AlignSelf::End,
                                                    // border: UiRect::all(Val::Px(1.0)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Text Busses Node"),
                                        ))
                                        .with_children(|parent| {
                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            flex_direction: FlexDirection::Column,
                                                            width: Val::Px(170.0),
                                                            row_gap: Val::Px(10.0),
                                                            align_items: AlignItems::Start,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("Text Busses 1-4"),
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
                                                            "Bus 2: loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        TextBus2,
                                                        Name::new("TextBus2"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Bus 3: loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        TextBus3,
                                                        Name::new("TextBus3"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Bus 4: loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        TextBus4,
                                                        Name::new("TextBus4"),
                                                    ));
                                                });
                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            flex_direction: FlexDirection::Column,
                                                            width: Val::Px(170.0),
                                                            row_gap: Val::Px(10.0),
                                                            align_items: AlignItems::Start,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("Text Busses 5-8"),
                                                ))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Bus 5: loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        TextBus5,
                                                        Name::new("TextBus5"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Bus 6: loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        TextBus6,
                                                        Name::new("TextBus6"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Bus 7: loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        TextBus7,
                                                        Name::new("TextBus7"),
                                                    ));
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Bus 8: loading...",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        TextBus8,
                                                        Name::new("TextBus8"),
                                                    ));
                                                });
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
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(136.0),
                                                        height: Val::Px(73.0),
                                                        flex_direction: FlexDirection::Column,
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::SpaceAround,
                                                        ..Default::default()
                                                    },
                                                    ..Default::default()
                                                },
                                                Name::new("Toggle Auto-Mine Node"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            width: Val::Px(66.0),
                                                            height: Val::Px(16.0),
                                                            align_items: AlignItems::Center,
                                                            justify_content: JustifyContent::Center,
                                                            ..Default::default()
                                                        },
                                                        ..Default::default()
                                                    },
                                                    Name::new("Text Auto-Mine Node"),
                                                )).with_children(|parent| {
                                                    parent.spawn((TextBundle::from_section(
                                                            "Mine",
                                                            TextStyle {
                                                                font: asset_server.load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                ..default()
                                                            },
                                                        ),
                                                    ));
                                                });
                                                parent.spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            width: Val::Px(49.0),
                                                            height: Val::Px(26.0),
                                                            ..Default::default()
                                                        },
                                                        ..Default::default()
                                                    },
                                                    Name::new("Toggle Auto-Mine"),
                                                )).with_children(|parent| {
                                                            parent.spawn((
                                                                ButtonBundle {
                                                                    style: Style {
                                                                        width: Val::Px(49.53),
                                                                        height: Val::Px(26.0),
                                                                        ..default()
                                                                    },
                                                                    image: UiImage::new(
                                                                        asset_server.load(TOGGLE_OFF),
                                                                    ),
                                                                    ..default()
                                                                },
                                                                ToggleAutoMine(false),
                                                                Name::new("ButtonStartMining"),
                                                            ));
                                                });
                                            });
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
                                    height: Val::Percent(47.0),
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
                                                    // flex_direction: FlexDirection::Row,
                                                    // align_items: AlignItems::Center,
                                                    // justify_content: JustifyContent::SpaceAround,
                                                    // width: Val::Percent(100.0),
                                                    flex_direction: FlexDirection::Row,
                                                    width: Val::Percent(100.0),
                                                    // padding: UiRect::left(Val::Px(20.0)),
                                                    // column_gap: Val::Px(30.0),
                                                    ..default()
                                                },
                                                background_color: Color::rgb(0.15, 0.15, 0.15)
                                                    .into(),
                                                ..default()
                                            },
                                            Name::new("Tx Status Title"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        height: Val::Px(20.0),
                                                        width: Val::Px(60.0),
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("Tx Type Node"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    TextBundle::from_section(
                                                        "Type",
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),
                                                    Label,
                                                ));
                                            });

                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        height: Val::Px(20.0),
                                                        width: Val::Px(80.0),
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("DateTime"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    TextBundle::from_section(
                                                        "Landed At",
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),
                                                    Label,
                                                ));
                                            });

                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        height: Val::Px(20.0),
                                                        width: Val::Px(134.0),
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("Signature"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    TextBundle::from_section(
                                                        "Signature",
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),
                                                    Label,
                                                ));
                                            });

                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        height: Val::Px(20.0),
                                                        width: Val::Px(80.0),
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("SendTime"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    TextBundle::from_section(
                                                        "Send Time",
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),
                                                    Label,
                                                ));
                                            });

                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        height: Val::Px(20.0),
                                                        width: Val::Px(150.0),
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("HashTime"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    TextBundle::from_section(
                                                        "Hash Time - Difficulty",
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),
                                                    Label,
                                                ));
                                            });

                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        height: Val::Px(20.0),
                                                        width: Val::Px(200.0),
                                                        margin: UiRect::left(Val::Px(10.0)),
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::Start,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("Status"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    TextBundle::from_section(
                                                        "Status",
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),
                                                    Label,
                                                ));
                                            });
                                            // parent.spawn((TextBundle::from_section(
                                            //     "Tx Type",
                                            //     TextStyle {
                                            //         font: asset_server.load(FONT_ROBOTO),
                                            //         font_size: FONT_SIZE_TITLE,
                                            //         ..default()
                                            //     },
                                            // ),));
                                            // parent.spawn((TextBundle::from_section(
                                            //     "DateTime",
                                            //     TextStyle {
                                            //         font: asset_server.load(FONT_ROBOTO),
                                            //         font_size: FONT_SIZE,
                                            //         ..default()
                                            //     },
                                            // ),));
                                            // parent.spawn((TextBundle::from_section(
                                            //     "Signature",
                                            //     TextStyle {
                                            //         font: asset_server.load(FONT_ROBOTO),
                                            //         font_size: FONT_SIZE,
                                            //         ..default()
                                            //     },
                                            // ),));
                                            // parent.spawn((TextBundle::from_section(
                                            //     "Send Time",
                                            //     TextStyle {
                                            //         font: asset_server.load(FONT_ROBOTO),
                                            //         font_size: FONT_SIZE,
                                            //         ..default()
                                            //     },
                                            // ),));
                                            // parent.spawn((TextBundle::from_section(
                                            //     "Hash Time - Difficulty",
                                            //     TextStyle {
                                            //         font: asset_server.load(FONT_ROBOTO),
                                            //         font_size: FONT_SIZE,
                                            //         ..default()
                                            //     },
                                            // ),));
                                            // parent.spawn((TextBundle::from_section(
                                            //     "Status",
                                            //     TextStyle {
                                            //         font: asset_server.load(FONT_ROBOTO),
                                            //         font_size: FONT_SIZE,
                                            //         ..default()
                                            //     },
                                            // ),));
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
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Px(100.0),
                                                    height: Val::Px(21.0),
                                                    align_self: AlignSelf::End,
                                                    justify_content: JustifyContent::SpaceBetween,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Auto Scroll Node"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((TextBundle::from_section(
                                                "Auto-Scroll",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO),
                                                    font_size: FONT_SIZE,
                                                    color: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.60 }
                                                },
                                            ),));
                                            parent.spawn((
                                                ButtonBundle {
                                                    background_color: Color::WHITE.into(),
                                                    style: Style {
                                                        width: Val::Px(21.0),
                                                        height: Val::Px(21.0),
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    image: UiImage::new(
                                                        asset_server.load(CHECKBOX),
                                                    ),
                                                    ..default()
                                                },
                                                ButtonAutoScroll(true),
                                                Name::new("ButtonAutoScroll"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    NodeBundle {
                                                        background_color: Color::WHITE.into(),
                                                        style: Style {
                                                            width: Val::Px(16.0),
                                                            height: Val::Px(11.0),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    UiImage::new(
                                                            asset_server.load(CHECK_ICON),
                                                        ),
                                                    AutoScrollCheckIcon,
                                                    Name::new("CheckIcon"),
                                                ));
                                            });
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
                                            height: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Column,
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
                                                    width: Val::Percent(100.0),
                                                    height: Val::Percent(100.0),
                                                    padding: UiRect::top(Val::Px(20.0)),
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
                                                            width: Val::Percent(40.0),
                                                            height: Val::Percent(100.0),
                                                            row_gap: Val::Px(10.0),
                                                            align_items: AlignItems::End,
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
                                                        format!("Total CPU Usage:"),
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO),
                                                            font_size: FONT_SIZE,
                                                            ..default()
                                                        },
                                                    ),));
                                                    parent.spawn((TextBundle::from_section(
                                                        format!("Total RAM Usage:"),
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
                                                            width: Val::Percent(60.0),
                                                            height: Val::Percent(100.0),
                                                            row_gap: Val::Px(10.0),
                                                            align_items: AlignItems::Start,
                                                            padding: UiRect::left(Val::Px(10.0)),
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
                                            height: Val::Percent(100.0),
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
                                                    height: Val::Px(40.0),
                                                    align_items: AlignItems::Start,
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
                                                    width: Val::Percent(100.0),
                                                    height: Val::Percent(70.0),
                                                    margin: UiRect {
                                                        top: Val::Px(10.0),
                                                        left: Val::Px(5.0),
                                                        right: Val::Px(5.0),
                                                        bottom: Val::Px(0.0),
                                                    },
                                                    row_gap: Val::Px(10.0),
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
                                                            "Last Claim At:",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextLastClaimAt"),
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Burn Amount: ",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("TextBurnAmount"),
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
                                                                    height: Val::Px(40.0),
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
                                                            " ",
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
                                                        Name::new("TextLastClaimAt"),
                                                        TextLastClaimAt,
                                                    ));
                                                    parent.spawn((
                                                        NodeBundle {
                                                            style: Style {
                                                                width: Val::Px(100.0),
                                                                height: Val::Px(40.0),
                                                                ..Default::default()
                                                            },
                                                            ..Default::default()
                                                        },
                                                        Name::new("BurnAmount Node"),
                                                    )).with_children(|parent| {
                                                        parent.spawn((
                                                            TextBundle::from_section(
                                                                "loading...",
                                                                TextStyle {
                                                                    font: asset_server
                                                                        .load(FONT_ROBOTO),
                                                                    font_size: FONT_SIZE,
                                                                    color: Color::RED.into(),
                                                                },
                                                            ),
                                                            Name::new("TextBurnAmount"),
                                                            TextBurnAmount,
                                                        ));

                                                        // parent.spawn((
                                                        //     NodeBundle {
                                                        //         background_color: Color::WHITE.into(),
                                                        //         style: Style {
                                                        //             width: Val::Px(24.0),
                                                        //             height: Val::Px(24.0),
                                                        //             ..Default::default()
                                                        //         },
                                                        //         ..Default::default()
                                                        //     },
                                                        //     UiImage::new(asset_server.load(FIRE_ICON)),
                                                        //     Name::new("FireIcon"),
                                                        // ));
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
                                                    height: Val::Percent(30.0),
                                                    justify_content: JustifyContent::SpaceAround,
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
                                                        width: Val::Px(129.0),
                                                        height: Val::Px(37.0),
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    image: UiImage::new(
                                                        asset_server.load(BUTTON_RED_MEDIUM),
                                                    ),
                                                    ..default()
                                                },
                                                ButtonClaimOreRewards,
                                                Name::new("ButtonClaimOreRewards"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    TextBundle::from_section(
                                                        "Burn & Claim",
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                                            font_size: FONT_SIZE,
                                                            color: Color::BLACK.into(),
                                                        },
                                                    ),
                                                    Name::new("TextBurnAndClaim"),
                                                ));
                                            });
                                            parent.spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        width: Val::Px(129.0),
                                                        height: Val::Px(37.0),
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    image: UiImage::new(
                                                        asset_server.load(BUTTON_GREEN_MEDIUM),
                                                    ),
                                                    ..default()
                                                },
                                                ButtonStakeOre,
                                                Name::new("ButtonStakeOre"),
                                            )).with_children(|parent| {
                                                parent.spawn((
                                                    TextBundle::from_section(
                                                        "Stake",
                                                        TextStyle {
                                                            font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                                            font_size: FONT_SIZE,
                                                            color: Color::BLACK.into(),
                                                        },
                                                    ),
                                                    Name::new("TextStake"),
                                                ));

                                            });
                                        });
                                });
                        });
                });
        });
}
