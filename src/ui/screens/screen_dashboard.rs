use bevy::{ecs::storage::Column, prelude::*};

use crate::ui::{
    components::{
        BaseScreenNode, ButtonCaptureTextInput, ButtonUnlock, DashboardProofUpdatesLogsList, DashboardScreenNode, LockedScreenNode, MovingScrollPanel, ScrollingList, ScrollingListNode, TextActiveMinersLastEpoch, TextActiveMinersThisEpoch, TextBus1, TextBus2, TextBus3, TextBus4, TextBus5, TextBus6, TextBus7, TextBus8, TextCrownStakeAmount, TextCursor, TextHighestDifficultySeen, TextInput, TextPasswordInput, TextPasswordLabel, TextTreasuryBalance, TextTreasuryRewardRate
    },
    styles::{hex_dark_mode_background, hex_dark_mode_nav_title, hex_dark_mode_text_gray, hex_dark_mode_text_white_2, CONTENT_BACKGROUND_MEDIUM, CONTENT_BACKGROUND_SMALL, FONT_REGULAR, FONT_SIZE_LARGE, FONT_SIZE_MEDIUM, FONT_SIZE_SMALL, NORMAL_BUTTON},
};

pub fn spawn_dashboard_screen(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
) {
    parent
        .spawn((
            NodeBundle {
                visibility: Visibility::Hidden,
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Name::new("App Node"),
            DashboardScreenNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(60.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Top Section"),
            )).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(70.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceEvenly,
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Top Left Section"),
                )).with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(40.0),
                                height: Val::Percent(30.0),
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
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
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Dashboard Screen Top Left Section"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Treasury",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTreasuryBalance"),
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
                            Name::new("TextTreasuryOreBalance"),
                            TextTreasuryBalance,
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(40.0),
                                height: Val::Percent(30.0),
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
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
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Dashboard Screen Top Left Section"),
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
                            Name::new("TextTreasuryBalance"),
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
                                width: Val::Percent(40.0),
                                height: Val::Percent(30.0),
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
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
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Dashboard Screen Top Left Section"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Highest Difficulty Seen",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextHighestDifficultySeen"),
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
                            Name::new("TextHighestDifficultySeen"),
                            TextHighestDifficultySeen,
                        ));
                    });
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(40.0),
                                height: Val::Percent(30.0),
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
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
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Dashboard Screen Top Left Section"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Crown",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleCrown"),
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
                            Name::new("TextCrownStakeAmount"),
                            TextCrownStakeAmount,
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "By:",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextTitleCrownBy"),
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
                            Name::new("TextCrownStakeAmount"),
                            TextCrownStakeAmount,
                        ));
                    });
                });
                parent.spawn((
                    NodeBundle {
                        background_color: hex_dark_mode_nav_title().into(),
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                    UiImage::new(
                            asset_server.load(CONTENT_BACKGROUND_MEDIUM),
                        ),
                    Name::new("Top Right Section"),
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
            });
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Bottom Section"),
            )).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(70.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Bottom Left Section"),
                )).with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("DashboardProofUpdatesLogs Section"),
                    )).with_children(|parent| {
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(95.0),
                                    height: Val::Percent(95.0),
                                    align_items: AlignItems::Start,
                                    justify_content: JustifyContent::Start,
                                    flex_direction: FlexDirection::Column,
                                    overflow: Overflow::clip_y(),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Scrolling List Node"),
                            ScrollingListNode(true),
                        )).with_children(|parent| {
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
                                MovingScrollPanel,
                                DashboardProofUpdatesLogsList,
                                Name::new("MovingScrollPanel"),
                            ));
                        });
                    });

                });

                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Bottom Right Section"),
                )).with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(40.0),
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Start,
                                flex_direction: FlexDirection::Column,
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
                                asset_server.load(CONTENT_BACKGROUND_SMALL),
                            ),
                        Name::new("Dashboard Screen Bottom Right Section"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Active Miners This Epoch:",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextActiveMinersTitle"),
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
                            Name::new("TextActiveMiners"),
                            TextActiveMinersThisEpoch,
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "Active Miners Last Epoch:",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextActiveMinersTitle"),
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
                            Name::new("TextActiveMinersLastEpoch"),
                            TextActiveMinersLastEpoch,
                        ));
                    });

                });

            });
        });
}

pub fn despawn_dashboard_screen(
    mut commands: Commands,
    query: Query<Entity, With<DashboardScreenNode>>,
) {
    let screen_node = query.get_single().unwrap();
    commands.entity(screen_node).despawn_recursive();
}


