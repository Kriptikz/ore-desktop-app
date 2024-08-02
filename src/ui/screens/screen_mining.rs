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
            AutoScrollCheckIcon, ButtonAutoScroll, ButtonCooldownSpinner, ButtonRequestAirdrop, ButtonStakeOre, MiningScreenNode, MiningScreenTxResultList, SpinnerIcon, TextBurnAmount, TextBus1, TextBus2, TextBus3, TextBus4, TextBus5, TextBus6, TextBus7, TextBus8, TextHashrate, TextLastClaimAt, TextLastHashAt, TextMinerStatusThreads, TxPopUpArea
        },
        spawn_utils::spawn_copyable_text,
        styles::{
            hex_black, hex_dark_mode_app_screen_background, hex_dark_mode_background, hex_dark_mode_nav_title, hex_dark_mode_text_gray, BUTTON_CLAIM, BUTTON_GREEN_MEDIUM, BUTTON_RED_MEDIUM, BUTTON_STAKE, CHECKBOX, CHECK_ICON, CONTENT_BACKGROUND_MEDIUM, CONTENT_BACKGROUND_SMALL, FONT_REGULAR, FONT_SIZE_LARGE, FONT_SIZE_MEDIUM, LOG_ITEMS_BACKGROUND, MINE_TOGGLE_BUTTON, PROOF_ACCOUNT_BACKGROUND, SPINNER_ICON, SYSTEM_OVERVIEW_BACKGROUND, TOGGLE_OFF, TREASURY_BACKGROUND, TX_RESULTS_BACKGROUND
        },
    }, utils::shorten_string, AppConfig, AppWallet
};

use crate::ui::{
    components::{
        BaseScreenNode, ButtonClaimOreRewards, MovingScrollPanel, ScrollingList,
        TextCurrentChallenge, TextCurrentStake, TextMinerStatusCpuUsage, TextMinerStatusRamUsage,
        TextMinerStatusStatus, TextMinerStatusTime, TextTotalHashes, TextTreasuryAdmin, TextTreasuryBalance, TextTreasuryLastResetAt,
        TextTreasuryRewardRate, TextWalletOreBalance, TextWalletSolBalance, ToggleAutoMine,
    },
    styles::FONT_SIZE_SMALL as FONT_SIZE,
};

pub fn spawn_app_screen_mining(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        Name::new("Mining App Screen"),
        MiningScreenNode
    )).with_children(|parent| {
        // Top Data Section
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(55.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("Mining App Screen Top Data Section"),
        )).with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Mining App Screen Top Data Section Top"),
            )).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        background_color: hex_dark_mode_nav_title().into(),
                        style: Style {
                            width: Val::Percent(25.0),
                            height: Val::Percent(90.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Start,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect {
                                top: Val::Px(5.0),
                                bottom: Val::Px(5.0),
                                left: Val::Px(8.0),
                                right: Val::Px(0.0),
                            },
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::new(
                            asset_server.load(CONTENT_BACKGROUND_SMALL),
                        ),
                    Name::new("Mining App Screen Top Section Top"),
                )).with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Base Reward Rate",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextBaseRewardRate"),
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "loading...",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextBaseRewardRate"),
                        TextTreasuryRewardRate,
                    ));
                });
                parent.spawn((
                    NodeBundle {
                        background_color: hex_dark_mode_nav_title().into(),
                        style: Style {
                            width: Val::Percent(25.0),
                            height: Val::Percent(90.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Start,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect {
                                top: Val::Px(5.0),
                                bottom: Val::Px(5.0),
                                left: Val::Px(8.0),
                                right: Val::Px(0.0),
                            },
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::new(
                            asset_server.load(CONTENT_BACKGROUND_SMALL),
                        ),
                    Name::new("Mining App Screen Top Section Top"),
                )).with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Last Reset At",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextLastResetAt"),
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            "loading...",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextLastResetAt"),
                        TextTreasuryLastResetAt,
                    ));
                });
            });
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(80.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Mining App Screen Top Data Section Bottom"),
            )).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        background_color: hex_dark_mode_background().into(),
                        style: Style {
                            width: Val::Percent(25.0),
                            height: Val::Percent(90.0),
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceAround,
                            padding: UiRect {
                                top: Val::Px(12.0),
                                bottom: Val::Px(5.0),
                                left: Val::Px(8.0),
                                right: Val::Px(0.0),
                            },
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::new(
                            asset_server.load(CONTENT_BACKGROUND_MEDIUM),
                        ),
                    Name::new("Mining App Screen Top Section Bottom"),
                )).with_children(|parent| {
                    // Title with tooltip icon
                    parent.spawn((
                        TextBundle::from_section(
                            "Busses",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextBussesTitle"),
                    ));

                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(10.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Busses Node Bus 1"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "1: ",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus1"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus1"),
                            TextBus1,
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_background().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(10.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Busses Node Bus 2"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "2: ",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus2"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "0.64323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus2"),
                            TextBus2
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(10.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Busses Node Bus 3"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "3: ",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus3"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "0.438873254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus3"),
                            TextBus3,
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_background().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(10.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Busses Node Bus 4"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "4: ",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus4"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus4"),
                            TextBus4,
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(10.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Busses Node Bus 5"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "5: ",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus5"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus5"),
                            TextBus5,
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_background().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(10.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Busses Node Bus 6"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "6: ",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus6"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus6"),
                            TextBus6,
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(10.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Busses Node Bus 7"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "7: ",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus7"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus7"),
                            TextBus7
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_background().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(10.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Busses Node Bus 8"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "8: ",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus8"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "0.650000000",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus8"),
                            TextBus8,
                        ));
                    });

                });
                parent.spawn((
                    NodeBundle {
                        background_color: hex_dark_mode_background().into(),
                        style: Style {
                            width: Val::Percent(25.0),
                            height: Val::Percent(90.0),
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceAround,
                            padding: UiRect {
                                top: Val::Px(12.0),
                                bottom: Val::Px(5.0),
                                left: Val::Px(8.0),
                                right: Val::Px(0.0),
                            },
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::new(
                            asset_server.load(CONTENT_BACKGROUND_MEDIUM),
                        ),
                    Name::new("Mining App Screen Top Section Bottom"),
                )).with_children(|parent| {
                    // Title with tooltip icon
                    parent.spawn((
                        TextBundle::from_section(
                            "Status",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextTitleStatus"),
                    ));

                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Status Current Time Node"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Current Time UTC",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleCurrentTime"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "2024-00-00 00:00:00 UTC",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextCurrentTime"),
                            TextMinerStatusTime,
                        ));
                    });

                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Status Threads Node"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Threads",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleThreads"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "1",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextThreads"),
                            TextMinerStatusThreads,
                        ));
                    });

                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Status CPU Usage Node"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "CPU Usage",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleCpuUsage"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "144.34 % / 400 %",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTotalCpuUsage"),
                            TextMinerStatusCpuUsage
                        ));
                    });

                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Status RAM Usage Node"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "RAM Usage",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleRamUsage"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "43 % / 100 %",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextRamUsage"),
                            TextMinerStatusRamUsage
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Status Hashrate"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Hashrate",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleHashrate"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "calculating...",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextHashrate"),
                            TextHashrate
                        ));
                    });
                });
                parent.spawn((
                    NodeBundle {
                        background_color: hex_dark_mode_background().into(),
                        style: Style {
                            width: Val::Percent(25.0),
                            height: Val::Percent(90.0),
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceAround,
                            padding: UiRect {
                                top: Val::Px(12.0),
                                bottom: Val::Px(5.0),
                                left: Val::Px(8.0),
                                right: Val::Px(0.0),
                            },
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::new(
                            asset_server.load(CONTENT_BACKGROUND_MEDIUM),
                        ),
                    Name::new("Mining App Screen Top Section Bottom"),
                )).with_children(|parent| {
                    // Title with tooltip icon
                    parent.spawn((
                        TextBundle::from_section(
                            "Proof Account",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextTreasuryBalance"),
                    ));

                    // Current Challenge
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Proof Account Current Challenge Node"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Current Challenge",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleCurrentChallenge"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "Adsakjlfdkslajdcliusdkcjlsccc",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextCurrentChallenge"),
                            TextCurrentChallenge,
                        ));
                    });

                    // Last Hash At
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Proof Account Last Hash At"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Last Hash At",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleLastHashAt"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "2024-00-00 00:00:00 UTC",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextLastHashAt"),
                            TextLastHashAt
                        ));
                    });

                    // Total Hashes
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Proof Account Total Hashes"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Total Hashes",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleTotalHashes"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "69",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTotalHashes"),
                            TextTotalHashes
                        ));
                    });

                    // Staked
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(8.0),
                                    right: Val::Px(0.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Proof Account Staked"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Staked",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleStaked"),
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "9001.60420",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextStaked"),
                            TextCurrentStake
                        ));
                    });
                });
            });
        });

        // Bottom Logs Section
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(45.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("Mining App Screen Logs Section"),
        )).with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        background_color: hex_dark_mode_nav_title().into(),
                        // border_color: Color::ORANGE.into(),
                        style: Style {
                            width: Val::Percent(95.0),
                            height: Val::Percent(98.0),
                            border: UiRect::all(Val::Px(1.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::new(asset_server.load(LOG_ITEMS_BACKGROUND)),
                    Name::new("Left Bottom Node"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                background_color: hex_dark_mode_background().into(),
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    height: Val::Percent(88.0),
                                    width: Val::Percent(95.0),
                                    ..default()
                                },
                                ..default()
                            },
                            UiImage::new(asset_server.load(LOG_ITEMS_BACKGROUND)),
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
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE,
                                                    color: hex_dark_mode_text_gray().into(),
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
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE,
                                                    color: hex_dark_mode_text_gray().into(),
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
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE,
                                                    color: hex_dark_mode_text_gray().into(),
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
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE,
                                                    color: hex_dark_mode_text_gray().into(),
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
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE,
                                                    color: hex_dark_mode_text_gray().into(),
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
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE,
                                                    color: hex_dark_mode_text_gray().into(),
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
                                        MiningScreenTxResultList,
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
                                            padding: UiRect::all(Val::Px(2.0)),
                                            justify_content: JustifyContent::SpaceBetween,
                                            align_items: AlignItems::Center,
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
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE,
                                            color: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.60 }
                                        },
                                    ),));
                                    parent.spawn((
                                        ButtonBundle {
                                            background_color: hex_dark_mode_app_screen_background().into(),
                                            style: Style {
                                                width: Val::Px(14.0),
                                                height: Val::Px(12.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            image: UiImage::new(
                                                asset_server.load(MINE_TOGGLE_BUTTON),
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
    });
}

pub fn despawn_mining_screen(
    mut commands: Commands,
    query: Query<Entity, With<MiningScreenNode>>,
) {
    if let Ok(screen_node) = query.get_single() {
        commands.entity(screen_node).despawn_recursive();
    }
}
