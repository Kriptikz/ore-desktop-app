
use bevy::
    prelude::*
;

use crate::ui::styles::{hex_dark_mode_app_screen_background, hex_dark_mode_background, hex_dark_mode_header_border, hex_dark_mode_nav_title, hex_dark_mode_text_gray, hex_dark_mode_text_white, hex_dark_mode_text_white_2, DASHBOARD_ICON_WHITE, FONT_REGULAR, FONT_SIZE_LARGE, FONT_SIZE_MEDIUM, FONT_SIZE_SMALL, MINE_TOGGLE_BACKGROUND, MINE_TOGGLE_BUTTON, MINE_TOGGLE_OFF, MINING_ICON, NAV_ARROW_ICON, ORE_LOGO_WHITE};
use crate::NavItemScreen;
use crate::{
    ui::
        styles::
            hex_black
        
    ,
    utils::shorten_string,
    AppConfig,
};

use crate::ui::
    components::{AppScreenParent, BaseScreenNode, NavItem, NavItemArrow, NavItemIcon, NavItemText, NavItemWhiteSelectedBar, TextWalletOreBalance, TextWalletPubkey, TextWalletSolBalance, ToggleAutoMine, ToggleAutoMineParent, TxPopUpArea}
    
;

use super::screen_dashboard::spawn_dashboard_screen;
use super::screen_locked::spawn_locked_screen;
use super::screen_mining::spawn_app_screen_mining;

pub fn spawn_base_screen(
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
                background_color: hex_dark_mode_background().into(),
                border_color: Color::PINK.into(),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("Base Screen Node"),
            BaseScreenNode,
        )).with_children(|parent| {
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
            parent.spawn((
                NodeBundle {
                    border_color: Color::PURPLE.into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(15.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Top Section"),
            )).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        border_color: Color::RED.into(),
                        style: Style {
                            width: Val::Percent(20.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Center,
                            padding: UiRect::left(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Ore Logo Section"),
                )).with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            background_color: Color::WHITE.into(),
                            border_color: Color::ORANGE.into(),
                            style: Style {
                                width: Val::Px(95.21),
                                height: Val::Px(50.0),
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(
                                asset_server.load(ORE_LOGO_WHITE),
                            ),
                        Name::new("ORE LOGO"),
                    ));
                });

                parent.spawn((
                    NodeBundle {
                        border_color: hex_dark_mode_header_border().into(),
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            border: UiRect {
                                top: Val::Px(0.5),
                                bottom: Val::Px(0.5),
                                left: Val::Px(0.6),
                                right: Val::Px(0.5),
                            },
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Top Section Header"),
                )).with_children(|parent| {
                    // Title
                    parent.spawn((
                        NodeBundle {
                            border_color: Color::PINK.into(),
                            style: Style {
                                width: Val::Percent(32.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Start,
                                padding: UiRect::left(Val::Px(5.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Top Section Header App Screen Title"),
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Mining",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_LARGE,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextAppScreenTitle"),
                        ));
                    });

                    // Mine Toggle
                    parent.spawn((
                        NodeBundle {
                            visibility: Visibility::Hidden,
                            style: Style {
                                width: Val::Percent(28.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Top Section Header Mine Toggle Section"),
                        ToggleAutoMineParent,
                    )).with_children(|parent| {
                        parent.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(292.0),
                                    height: Val::Px(80.0),
                                    ..default()
                                },
                                image: UiImage::new(
                                    asset_server.load(MINE_TOGGLE_OFF),
                                ),
                                ..default()
                            },
                            Name::new("Top Section Header Mine Toggle"),
                            ToggleAutoMine(false),
                        ));
                    });

                    // Wallet Info
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(45.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Top Section Header Wallet Section"),
                    )).with_children(|parent| {
                        parent.spawn((
                            NodeBundle {
                                background_color: hex_dark_mode_nav_title().into(),
                                style: Style {
                                    width: Val::Percent(98.0),
                                    height: Val::Percent(90.0),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            UiImage::new(
                                    asset_server.load(MINE_TOGGLE_BACKGROUND),
                                ),
                            Name::new("Top Section Header Wallet"),
                        )).with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(45.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::SpaceBetween,
                                        align_items: AlignItems::Start,
                                        flex_direction: FlexDirection::Column,
                                        padding: UiRect {
                                            top: Val::Px(10.0),
                                            bottom: Val::Px(10.0),
                                            left: Val::Px(8.0),
                                            right: Val::Px(0.0),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Top Section Header Wallet Left"),
                            )).with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        "Wallet",
                                        TextStyle {
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE_MEDIUM,
                                            color: hex_dark_mode_text_gray().into()
                                        },
                                    ),
                                    Name::new("TextWallet"),
                                ));

                                parent.spawn((
                                    TextBundle::from_section(
                                        "0.2343532 SOL",
                                        TextStyle {
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE_MEDIUM,
                                            color: hex_dark_mode_text_gray().into()
                                        },
                                    ),
                                    Name::new("TextSolBalance"),
                                    TextWalletSolBalance
                                ));
                            });
                            parent.spawn((
                                NodeBundle {
                                    background_color: hex_dark_mode_background().into(),
                                    style: Style {
                                        width: Val::Percent(53.0),
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
                                        asset_server.load(MINE_TOGGLE_BUTTON),
                                    ),
                                Name::new("Top Section Header Wallet Right"),
                            )).with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        "dfsadjkl...sdioa",
                                        TextStyle {
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE_MEDIUM,
                                            color: hex_dark_mode_text_gray().into()
                                        },
                                    ),
                                    Name::new("TextWalletPubkey"),
                                    TextWalletPubkey,
                                ));
                                parent.spawn((
                                    TextBundle::from_section(
                                        "420.696969 ORE",
                                        TextStyle {
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE_MEDIUM,
                                            color: hex_dark_mode_text_gray().into()
                                        },
                                    ),
                                    Name::new("TextOreBalance"),
                                    TextWalletOreBalance
                                ));
                            });
                        });
                    });
                });
            });

            parent.spawn((
                NodeBundle {
                    border_color: Color::GREEN.into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(85.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                },
                Name::new("App Section"),
            )).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        border_color: Color::YELLOW.into(),
                        style: Style {
                            width: Val::Percent(20.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Nav Bar"),
                )).with_children(|parent| {
                    //Nav children.
                    parent.spawn((
                        NodeBundle {
                            border_color: Color::GREEN.into(),
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(50.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Nav Bar Top Half"),
                    )).with_children(|parent| {
                        // Top Nav Children.
                        parent.spawn((
                            NodeBundle {
                                border_color: Color::BLUE.into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(40.0),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Nav Bar Top Half Menu"),
                        )).with_children(|parent| {
                            // Menu Nav Children
                            parent.spawn((
                                NodeBundle {
                                    background_color: hex_dark_mode_nav_title().into(),
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(30.0),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect::left(Val::Px(15.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Nav Bar Top Half Menu Text"),
                            )).with_children(|parent| {
                                // Menu Nav Text
                                parent.spawn((
                                    TextBundle::from_section(
                                        "MENU",
                                        TextStyle {
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE_MEDIUM,
                                            color: hex_dark_mode_text_white().into()
                                        },
                                    ),
                                    Name::new("TextMenu"),
                                ));
                            });

                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(40.0),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Nav Bar Top Half Menu Items"),
                            )).with_children(|parent| {
                                // Menu Nav Items
                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(35.0),
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Nav Bar Top Half Menu Items Item"),
                                    Interaction::default(),
                                    NavItem(NavItemScreen::Dashboard),
                                )).with_children(|parent| {
                                    // Menu Nav Item
                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::SpaceBetween,
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Name::new("Nav Item"),
                                    )).with_children(|parent| {
                                        // Menu Nav Item
                                        parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(80.0),
                                                    height: Val::Percent(100.0),
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Left"),
                                        )).with_children(|parent| {
                                            parent.spawn((
                                                NodeBundle {
                                                    background_color: Color::WHITE.into(),
                                                    visibility: Visibility::Hidden,
                                                    style: Style {
                                                        width: Val::Px(2.5),
                                                        height: Val::Px(FONT_SIZE_SMALL),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("Nav Item Selected"),
                                                NavItemWhiteSelectedBar(NavItemScreen::Dashboard),
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(10.0),
                                                        height: Val::Px(FONT_SIZE_SMALL),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("Nav Item Selected Margin"),
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    background_color: Color::GRAY.into(),
                                                    style: Style {
                                                        width: Val::Px(15.0),
                                                        height: Val::Px(15.0),
                                                        margin: UiRect::right(Val::Px(5.0)),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                UiImage::new(
                                                        asset_server.load(DASHBOARD_ICON_WHITE),
                                                    ),
                                                Name::new("Dashboard Icon"),
                                                NavItemIcon(NavItemScreen::Dashboard),
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Dashboard",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_REGULAR),
                                                        font_size: FONT_SIZE_SMALL,
                                                        color: hex_dark_mode_text_gray().into()
                                                    },
                                                ),
                                                Name::new("TextDashboard"),
                                                NavItemText(NavItemScreen::Dashboard),
                                            ));
                                        });

                                        parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(20.0),
                                                    height: Val::Percent(100.0),
                                                    justify_content: JustifyContent::End,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Right"),
                                        )).with_children(|parent| {
                                            parent.spawn((
                                                NodeBundle {
                                                    background_color: Color::GRAY.into(),
                                                    style: Style {
                                                        width: Val::Px(20.0),
                                                        height: Val::Px(20.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                UiImage::new(
                                                        asset_server.load(NAV_ARROW_ICON),
                                                    ),
                                                Name::new("Nav Arrow Icon"),
                                                NavItemArrow(NavItemScreen::Dashboard),
                                            ));
                                        });
                                    });
                                });

                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(35.0),
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Nav Bar Top Half Menu Items Item"),
                                    Interaction::default(),
                                    NavItem(NavItemScreen::Mining),
                                )).with_children(|parent| {
                                    // Menu Nav Item
                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::SpaceBetween,
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Name::new("Nav Item"),
                                    )).with_children(|parent| {
                                        // Menu Nav Item
                                        parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(80.0),
                                                    height: Val::Percent(100.0),
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Left"),
                                        )).with_children(|parent| {
                                            parent.spawn((
                                                NodeBundle {
                                                    background_color: Color::WHITE.into(),
                                                    visibility: Visibility::Hidden,
                                                    style: Style {
                                                        width: Val::Px(2.5),
                                                        height: Val::Px(FONT_SIZE_SMALL),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("Nav Item Selected"),
                                                NavItemWhiteSelectedBar(NavItemScreen::Mining),
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(10.0),
                                                        height: Val::Px(FONT_SIZE_SMALL),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("Nav Item Selected Margin"),
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    background_color: Color::WHITE.into(),
                                                    style: Style {
                                                        width: Val::Px(15.0),
                                                        height: Val::Px(15.0),
                                                        margin: UiRect::right(Val::Px(5.0)),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                UiImage::new(
                                                        asset_server.load(MINING_ICON),
                                                    ),
                                                Name::new("Mining Icon"),
                                                NavItemIcon(NavItemScreen::Mining),
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Mining",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_REGULAR),
                                                        font_size: FONT_SIZE_SMALL,
                                                        color: hex_dark_mode_text_white_2().into()
                                                    },
                                                ),
                                                Name::new("TextMining"),
                                                NavItemText(NavItemScreen::Mining)
                                            ));
                                        });

                                        parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(20.0),
                                                    height: Val::Percent(100.0),
                                                    justify_content: JustifyContent::End,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Right"),
                                        )).with_children(|parent| {
                                            parent.spawn((
                                                NodeBundle {
                                                    background_color: Color::WHITE.into(),
                                                    style: Style {
                                                        width: Val::Px(20.0),
                                                        height: Val::Px(20.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                UiImage::new(
                                                        asset_server.load(NAV_ARROW_ICON),
                                                    ),
                                                Name::new("Nav Arrow Icon"),
                                                NavItemArrow(NavItemScreen::Mining),
                                            ));
                                        });
                                    });
                                });

                            });
                        });

                        parent.spawn((
                            NodeBundle {
                                border_color: Color::ORANGE.into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(60.0),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Nav Bar Top Half Apps"),
                        )).with_children(|parent| {
                            // Apps Nav Children
                        });
                    });
                    parent.spawn((
                        NodeBundle {
                            border_color: Color::RED.into(),
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(50.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::End,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Nav Bar Bottom Half"),
                    )).with_children(|parent| {
                        // Bottom Nav Children.
                        parent.spawn((
                            NodeBundle {
                                background_color: hex_dark_mode_nav_title().into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(30.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::left(Val::Px(15.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Nav Bar Bottom Half Settings Text"),
                        )).with_children(|parent| {
                            // Menu Nav Text
                            parent.spawn((
                                TextBundle::from_section(
                                    "SETTINGS",
                                    TextStyle {
                                        font: asset_server.load(FONT_REGULAR),
                                        font_size: FONT_SIZE_MEDIUM,
                                        color: hex_dark_mode_text_white().into()
                                    },
                                ),
                                Name::new("TextSettings"),
                            ));
                        });

                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(30.0),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Nav Bar Bottom Half Settings Items"),
                        )).with_children(|parent| {
                            // Settings Nav Items
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(28.0),
                                        flex_direction: FlexDirection::Row,
                                        padding: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(5.0), bottom: Val::Px(5.0) },
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Nav Bar Bottom Half Settings Items Item"),
                                Interaction::default(),
                                NavItem(NavItemScreen::SettingsConfig),
                            )).with_children(|parent| {
                                // Settings Nav Item
                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::SpaceBetween,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Nav Item"),
                                )).with_children(|parent| {
                                    // Menu Nav Item
                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(80.0),
                                                height: Val::Percent(100.0),
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Name::new("Nav Item Left"),
                                    )).with_children(|parent| {
                                        parent.spawn((
                                            NodeBundle {
                                                background_color: Color::WHITE.into(),
                                                visibility: Visibility::Hidden,
                                                style: Style {
                                                    width: Val::Px(2.5),
                                                    height: Val::Px(FONT_SIZE_SMALL),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Selected"),
                                            NavItemWhiteSelectedBar(NavItemScreen::SettingsConfig),
                                        ));
                                        parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Px(10.0),
                                                    height: Val::Px(FONT_SIZE_SMALL),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Selected Margin"),
                                        ));
                                        parent.spawn((
                                            NodeBundle {
                                                background_color: Color::GRAY.into(),
                                                style: Style {
                                                    width: Val::Px(15.0),
                                                    height: Val::Px(15.0),
                                                    margin: UiRect::right(Val::Px(5.0)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            UiImage::new(
                                                    asset_server.load(DASHBOARD_ICON_WHITE),
                                                ),
                                            Name::new("Config Icon"),
                                            NavItemIcon(NavItemScreen::SettingsConfig),
                                        ));
                                        parent.spawn((
                                            TextBundle::from_section(
                                                "Config",
                                                TextStyle {
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE_SMALL,
                                                    color: hex_dark_mode_text_gray().into()
                                                },
                                            ),
                                            Name::new("TextConfig"),
                                            NavItemText(NavItemScreen::SettingsConfig),
                                        ));
                                    });

                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(20.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::End,
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Name::new("Nav Item Right"),
                                    )).with_children(|parent| {
                                        parent.spawn((
                                            NodeBundle {
                                                background_color: Color::GRAY.into(),
                                                style: Style {
                                                    width: Val::Px(20.0),
                                                    height: Val::Px(20.0),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            UiImage::new(
                                                    asset_server.load(NAV_ARROW_ICON),
                                                ),
                                            Name::new("Nav Arrow Icon"),
                                            NavItemArrow(NavItemScreen::SettingsConfig),
                                        ));
                                    });
                                });
                            });

                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(28.0),
                                        flex_direction: FlexDirection::Row,
                                        padding: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(5.0), bottom: Val::Px(5.0) },
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Nav Bar Bottom Half Settings Items Item"),
                                Interaction::default(),
                                NavItem(NavItemScreen::SettingsWallet),
                            )).with_children(|parent| {
                                // Menu Nav Item
                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::SpaceBetween,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Nav Item"),
                                )).with_children(|parent| {
                                    // Menu Nav Item
                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(80.0),
                                                height: Val::Percent(100.0),
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Name::new("Nav Item Left"),
                                    )).with_children(|parent| {
                                        parent.spawn((
                                            NodeBundle {
                                                background_color: Color::WHITE.into(),
                                                visibility: Visibility::Hidden,
                                                style: Style {
                                                    width: Val::Px(2.5),
                                                    height: Val::Px(FONT_SIZE_SMALL),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Selected"),
                                            NavItemWhiteSelectedBar(NavItemScreen::SettingsWallet)
                                        ));
                                        parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Px(10.0),
                                                    height: Val::Px(FONT_SIZE_SMALL),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Selected Margin"),
                                        ));
                                        parent.spawn((
                                            NodeBundle {
                                                background_color: Color::GRAY.into(),
                                                style: Style {
                                                    width: Val::Px(15.0),
                                                    height: Val::Px(15.0),
                                                    margin: UiRect::right(Val::Px(5.0)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            UiImage::new(
                                                    asset_server.load(MINING_ICON),
                                                ),
                                            Name::new("Mining Icon"),
                                            NavItemIcon(NavItemScreen::SettingsWallet),
                                        ));
                                        parent.spawn((
                                            TextBundle::from_section(
                                                "Wallet",
                                                TextStyle {
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE_SMALL,
                                                    color: hex_dark_mode_text_gray().into()
                                                },
                                            ),
                                            Name::new("TextWallet"),
                                            NavItemText(NavItemScreen::SettingsWallet),
                                        ));
                                    });

                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(20.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::End,
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Name::new("Nav Item Right"),
                                    )).with_children(|parent| {
                                        parent.spawn((
                                            NodeBundle {
                                                background_color: Color::GRAY.into(),
                                                style: Style {
                                                    width: Val::Px(20.0),
                                                    height: Val::Px(20.0),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            UiImage::new(
                                                    asset_server.load(NAV_ARROW_ICON),
                                                ),
                                            Name::new("Nav Arrow Icon"),
                                            NavItemArrow(NavItemScreen::SettingsWallet),
                                        ));
                                    });
                                });
                            });

                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(28.0),
                                        flex_direction: FlexDirection::Row,
                                        padding: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(5.0), bottom: Val::Px(5.0) },
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Nav Bar Bottom Half Menu Items Item"),
                                Interaction::default(),
                                NavItem(NavItemScreen::SettingsGeneral),
                            )).with_children(|parent| {
                                // Menu Nav Item
                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::SpaceBetween,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Nav Item"),
                                )).with_children(|parent| {
                                    // Menu Nav Item
                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(80.0),
                                                height: Val::Percent(100.0),
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Name::new("Nav Item Left"),
                                    )).with_children(|parent| {
                                        parent.spawn((
                                            NodeBundle {
                                                background_color: Color::WHITE.into(),
                                                visibility: Visibility::Hidden,
                                                style: Style {
                                                    width: Val::Px(2.5),
                                                    height: Val::Px(FONT_SIZE_SMALL),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Selected"),
                                            NavItemWhiteSelectedBar(NavItemScreen::SettingsGeneral),
                                        ));
                                        parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Px(10.0),
                                                    height: Val::Px(FONT_SIZE_SMALL),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Nav Item Selected Margin"),
                                        ));
                                        parent.spawn((
                                            NodeBundle {
                                                background_color: Color::GRAY.into(),
                                                style: Style {
                                                    width: Val::Px(15.0),
                                                    height: Val::Px(15.0),
                                                    margin: UiRect::right(Val::Px(5.0)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            UiImage::new(
                                                    asset_server.load(MINING_ICON),
                                                ),
                                            Name::new("General Settings Icon"),
                                            NavItemIcon(NavItemScreen::SettingsGeneral),
                                        ));
                                        parent.spawn((
                                            TextBundle::from_section(
                                                "General",
                                                TextStyle {
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE_SMALL,
                                                    color: hex_dark_mode_text_gray().into()
                                                },
                                            ),
                                            Name::new("TextGeneral"),
                                            NavItemText(NavItemScreen::SettingsGeneral),
                                        ));
                                    });

                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                width: Val::Percent(20.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::End,
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Name::new("Nav Item Right"),
                                    )).with_children(|parent| {
                                        parent.spawn((
                                            NodeBundle {
                                                background_color: Color::GRAY.into(),
                                                style: Style {
                                                    width: Val::Px(20.0),
                                                    height: Val::Px(20.0),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            UiImage::new(
                                                    asset_server.load(NAV_ARROW_ICON),
                                                ),
                                            Name::new("Nav Arrow Icon"),
                                            NavItemText(NavItemScreen::SettingsGeneral),
                                        ));
                                    });
                                });
                            });

                        });
                    });
                });
                parent.spawn((
                    NodeBundle {
                        background_color: hex_dark_mode_app_screen_background().into(),
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("App Screen Parent"),
                    AppScreenParent,
                )).with_children(|parent| {
                    spawn_dashboard_screen(parent, &asset_server)
                });
            });
        });
}
