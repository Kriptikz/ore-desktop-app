use bevy::{a11y::{accesskit::{NodeBuilder, Role}, AccessibilityNode}, prelude::*};
use crate::{spawn_copyable_text, AppWallet};
use solana_sdk::signature::Signer;

use super::{styles::*, components::*};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>, app_wallet: Res<AppWallet>) {
    let full_addr = app_wallet.wallet.pubkey().to_string();
    let wallet_str;
    let len = full_addr.len();
    if len > 10 {
        let prefix = &full_addr[0..5];

        let suffix = &full_addr[len - 5..len];

        wallet_str = format!("{}...{}", prefix, suffix);
    } else {
        wallet_str = full_addr;
    }
    let sol_balance_str = app_wallet.sol_balance.to_string();
    let ore_balance_str = app_wallet.ore_balance.to_string();
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("Screen Node"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(50.0),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Top Half"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(50.0),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Top Half Left"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(350.0),
                                            height: Val::Px(150.0),
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect {
                                                top: Val::Px(40.0),
                                                left: Val::Px(30.0),
                                                right: Val::Px(0.0),
                                                bottom: Val::Px(0.0),
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("TreasuryAccountNode"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Treasury",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE + 8.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTitleTreasury"),
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Balance: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryBalance"),
                                        TextTreasuryBalance,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Admin: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryAdmin"),
                                        TextTreasuryAdmin,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Difficulty: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryDifficulty"),
                                        TextTreasuryDifficulty,
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Last Reset At: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryLastResetAt"),
                                        TextTreasuryLastResetAt,
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Reward Rate: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryRewardRate"),
                                        TextTreasuryRewardRate,
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Total Claimed Rewards: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTreasuryTotalClaimedRewards"),
                                        TextTreasuryTotalClaimedRewards,
                                    ));
                                });
                        });

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(50.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::End,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Top Half Right"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::End,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("AppWallet Node"),
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
                                                            .load("fonts/FiraSans-Bold.ttf"),
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
                                                            .load("fonts/FiraSans-Bold.ttf"),
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
                                        style: Style {
                                            width: Val::Px(200.0),
                                            height: Val::Px(110.0),
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect {
                                                top: Val::Px(100.0),
                                                left: Val::Px(0.0),
                                                right: Val::Px(160.0),
                                                bottom: Val::Px(0.0),
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("ProofAccountNode"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Proof Account",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE + 8.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTitleProofAccount"),
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "current hash: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextCurrentHash"),
                                        TextCurrentHash,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "total hashes: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTotalHashes"),
                                        TextTotalHashes,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "total rewards: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextTotalRewards"),
                                        TextTotalRewards,
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            "claimable rewards: loading...",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        Name::new("TextClaimableRewards"),
                                        TextClaimableRewards,
                                    ));
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(300.0),
                                            height: Val::Px(30.0),
                                            margin: UiRect {
                                                top: Val::Px(0.0),
                                                right: Val::Px(100.0),
                                                left: Val::Px(0.0),
                                                bottom: Val::Px(0.0),
                                            },
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
                                    //ButtonUpdateSolOreBalances,
                                    Name::new("ButtonClaimOreRewards"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "CLAIM",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(300.0),
                                            height: Val::Px(30.0),
                                            margin: UiRect {
                                                top: Val::Px(0.0),
                                                right: Val::Px(100.0),
                                                left: Val::Px(0.0),
                                                bottom: Val::Px(0.0),
                                            },
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
                                    ButtonUpdateSolOreBalances,
                                    Name::new("ButtonUpdateSolOreBalances"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Update Sol and Ore balances",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });
                        });

                    // ore logo (flex center)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                margin: UiRect {
                                    top: Val::Px(40.0),
                                    left: Val::Percent(49.0),
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
                                            width: Val::Px(125.0),
                                            height: Val::Px(125.0),
                                            margin: UiRect::top(Val::VMin(5.)),
                                            ..default()
                                        },
                                        // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                                        background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    UiImage::new(asset_server.load("ore-icon.webp")),
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

                });
                parent
                    .spawn((NodeBundle {
                        z_index: ZIndex::Global(10),
                        style: Style {
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            margin: UiRect {
                                top: Val::Percent(24.0),
                                right: Val::Px(0.0),
                                left: Val::Percent(50.0),
                                bottom: Val::Px(0.0),
                            },
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("ButtonStartMining"),
                ))
                    .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
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
                    ButtonStartStopMining,
                    Name::new("ButtonStartStopMining"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "START MINING",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FONT_SIZE,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
                    });
            parent
                .spawn((
                    NodeBundle {
                        background_color: Color::DARK_GRAY.into(),
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(50.0),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Bottom Half"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(60.0),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Bottom Half Left"),
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
                                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                            ..default()
                                        },
                                        Name::new("Tx Status Title"),
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                "Signature",
                                                TextStyle {
                                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 25.,
                                                    ..default()
                                                },
                                            ),
                                        ));
                                        parent.spawn((
                                            TextBundle::from_section(
                                                "Tx Time",
                                                TextStyle {
                                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 25.,
                                                    ..default()
                                                },
                                            ),
                                        ));
                                        parent.spawn((
                                            TextBundle::from_section(
                                                "Hash Time",
                                                TextStyle {
                                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 25.,
                                                    ..default()
                                                },
                                            ),
                                        ));
                                        parent.spawn((
                                            TextBundle::from_section(
                                                "Status",
                                                TextStyle {
                                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 25.,
                                                    ..default()
                                                },
                                            ),
                                        ));
                                    });
                                    // List with hidden overflow
                                    // TODO: look into lazy loading items
                                    // TODO: add the scroll bar on the right
                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                flex_direction: FlexDirection::Column,
                                                align_self: AlignSelf::Stretch,
                                                min_height: Val::Percent(90.0),
                                                max_height: Val::Percent(90.0),
                                                overflow: Overflow::clip_y(),
                                                ..default()
                                            },
                                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                                            ..default()
                                        },
                                        Name::new("ScrollingList Node"),
                                    ))
                                        .with_children(|parent| {
                                            // Moving panel
                                            parent
                                                .spawn((
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
                                                ))
                                                .with_children(|parent| {
                                                    // List items
                                                    for i in 0..100 {
                                                        parent.spawn((
                                                            NodeBundle {
                                                                style: Style {
                                                                    flex_direction: FlexDirection::Row,
                                                                    width: Val::Percent(100.0),
                                                                    justify_content: JustifyContent::SpaceAround,
                                                                    ..default()
                                                                },
                                                                ..default()
                                                            },
                                                            Name::new("TxResult Item"),
                                                            AccessibilityNode(NodeBuilder::new(
                                                                Role::ListItem,
                                                            )),
                                                        ))
                                                        .with_children(|parent| {
                                                            parent.spawn((
                                                                TextBundle::from_section(
                                                                    format!("{i}."),
                                                                    TextStyle {
                                                                        font: asset_server.load(
                                                                            "fonts/FiraSans-Bold.ttf",
                                                                        ),
                                                                        font_size: 20.,
                                                                        ..default()
                                                                    },
                                                                ),
                                                                Label,
                                                            ));

                                                            parent.spawn((
                                                                TextBundle::from_section(
                                                                    format!("TxnS...s8cs   COPY"),
                                                                    TextStyle {
                                                                        font: asset_server.load(
                                                                            "fonts/FiraSans-Bold.ttf",
                                                                        ),
                                                                        font_size: 20.,
                                                                        ..default()
                                                                    },
                                                                ),
                                                            ));

                                                            parent.spawn((
                                                                TextBundle::from_section(
                                                                    format!("23s"),
                                                                    TextStyle {
                                                                        font: asset_server.load(
                                                                            "fonts/FiraSans-Bold.ttf",
                                                                        ),
                                                                        font_size: 20.,
                                                                        ..default()
                                                                    },
                                                                ),
                                                            ));

                                                            parent.spawn((
                                                                TextBundle::from_section(
                                                                    format!("40s"),
                                                                    TextStyle {
                                                                        font: asset_server.load(
                                                                            "fonts/FiraSans-Bold.ttf",
                                                                        ),
                                                                        font_size: 20.,
                                                                        ..default()
                                                                    },
                                                                ),
                                                            ));

                                                            parent.spawn((
                                                                TextBundle::from_section(
                                                                    format!("SUCCESS"),
                                                                    TextStyle {
                                                                        font: asset_server.load(
                                                                            "fonts/FiraSans-Bold.ttf",
                                                                        ),
                                                                        font_size: 20.,
                                                                        ..default()
                                                                    },
                                                                ),
                                                            ));
                                                        });
                                                    }
                                                });
                                        });
                                });

                        });
                    parent
                        .spawn((
                            NodeBundle {
                                background_color: Color::DARK_GRAY.into(),
                                style: Style {
                                    width: Val::Percent(40.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::SpaceAround,
                                    align_items: AlignItems::End,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Bottom Half Right"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            row_gap: Val::Px(20.0),
                                            align_items: AlignItems::Start,
                                            margin: UiRect {
                                                top: Val::Px(0.0),
                                                right: Val::Px(10.0),
                                                left: Val::Px(0.0),
                                                bottom: Val::Px(0.0),
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Mining Status Node"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Miner Status: STOPPED"),
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/FiraSans-Bold.ttf",
                                                ),
                                                font_size: 20.,
                                                ..default()
                                            },
                                        ),
                                        TextMinerStatusStatus,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("CPU Usage: 2%"),
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/FiraSans-Bold.ttf",
                                                ),
                                                font_size: 20.,
                                                ..default()
                                            },
                                        ),
                                        TextMinerStatusCpuUsage,
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("RAM Usage: 0.2 GB / 6.0 GB"),
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/FiraSans-Bold.ttf",
                                                ),
                                                font_size: 20.,
                                                ..default()
                                            },
                                        ),
                                        TextMinerStatusRamUsage,
                                    ));
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        border_color: Color::BLACK.into(),
                                        style: Style {
                                            border: UiRect {
                                                top: Val::Px(5.0),
                                                right: Val::Px(5.0),
                                                left: Val::Px(5.0),
                                                bottom: Val::Px(5.0),
                                            },
                                            flex_direction: FlexDirection::Column,
                                            row_gap: Val::Px(20.0),
                                            width: Val::Px(250.0),
                                            padding: UiRect::all(Val::Px(5.0)),
                                            margin: UiRect {
                                                top: Val::Px(0.0),
                                                right: Val::Px(10.0),
                                                left: Val::Px(0.0),
                                                bottom: Val::Px(0.0),
                                            },
                                            align_items: AlignItems::Start,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Current Tx Status Node"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Current Transaction"),
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/FiraSans-Bold.ttf",
                                                ),
                                                font_size: 26.,
                                                ..default()
                                            },
                                        ),
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Signature:   COPY"),
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/FiraSans-Bold.ttf",
                                                ),
                                                font_size: 20.,
                                                ..default()
                                            },
                                        ),
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Status:"),
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/FiraSans-Bold.ttf",
                                                ),
                                                font_size: 20.,
                                                ..default()
                                            },
                                        ),
                                    ));
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Elapsed:"),
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/FiraSans-Bold.ttf",
                                                ),
                                                font_size: 20.,
                                                ..default()
                                            },
                                        ),
                                    ));
                                });
                        });
                });
        });

}

pub fn spawn_new_list_item(commands: &mut Commands, asset_server: &Res<AssetServer>, scroll_panel_entity: Entity) {
        let new_result_item = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    ..default()
                },
                Name::new("TxResult Item"),
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        format!("NEW."),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.,
                            ..default()
                        },
                    ),
                    Label,
                ));

                parent.spawn((TextBundle::from_section(
                    format!("TxnS...s8cs   COPY"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        ..default()
                    },
                ),));

                parent.spawn((TextBundle::from_section(
                    format!("23s"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        ..default()
                    },
                ),));

                parent.spawn(TextBundle::from_section(
                    format!("40s"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        ..default()
                    },
                ));

                parent.spawn(TextBundle::from_section(
                    format!("SUCCESS"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        ..default()
                    },
                ));
            }).id();

        commands.entity(scroll_panel_entity).add_child(new_result_item);
}