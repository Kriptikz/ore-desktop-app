
use bevy::
    prelude::*
;

use crate::ui::styles::{hex_dark_mode_background, hex_dark_mode_nav_title, hex_dark_mode_text_gray, hex_dark_mode_text_white, hex_dark_mode_text_white_2, DASHBOARD_ICON_WHITE, FONT_REGULAR, FONT_SIZE_MEDIUM, FONT_SIZE_SMALL, MINING_ICON_GRAY, NAV_ARROW_ICON, ORE_LOGO_WHITE};
use crate::{
    ui::
        styles::
            hex_black
        
    ,
    utils::shorten_string,
    AppConfig,
};

use crate::ui::
    components::
        BaseScreenNode
    
;

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
                            padding: UiRect::left(Val::Px(12.0)),
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
                        border_color: Color::MIDNIGHT_BLUE.into(),
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
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
                                width: Val::Percent(35.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Top Section Header App Screen Title"),
                    )).with_children(|parent| {});
                    // Mine Toggle
                    parent.spawn((
                        NodeBundle {
                            border_color: Color::GREEN.into(),
                            style: Style {
                                width: Val::Percent(25.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Top Section Header Mine Toggle"),
                    )).with_children(|parent| {});
                    // Wallet Info
                    parent.spawn((
                        NodeBundle {
                            border_color: Color::ORANGE.into(),
                            style: Style {
                                width: Val::Percent(40.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Top Section Header Wallet Info"),
                    )).with_children(|parent| {});
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
                                        height: Val::Px(38.0),
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
                                            height: Val::Px(28.0),
                                            flex_direction: FlexDirection::Row,
                                            padding: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(5.0), bottom: Val::Px(5.0) },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Nav Bar Top Half Menu Items Item"),
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
                                                    style: Style {
                                                        width: Val::Px(2.5),
                                                        height: Val::Px(FONT_SIZE_SMALL),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("Nav Item Selected"),
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(15.0),
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
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                UiImage::new(
                                                        asset_server.load(DASHBOARD_ICON_WHITE),
                                                    ),
                                                Name::new("Dashboard Icon"),
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Dashboard",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_REGULAR),
                                                        font_size: FONT_SIZE_SMALL,
                                                        color: hex_dark_mode_text_white_2().into()
                                                    },
                                                ),
                                                Name::new("TextDashboard"),
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
                                    Name::new("Nav Bar Top Half Menu Items Item"),
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
                                                    background_color: hex_dark_mode_background().into(),
                                                    style: Style {
                                                        width: Val::Px(2.5),
                                                        height: Val::Px(FONT_SIZE_SMALL),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Name::new("Nav Item Selected"),
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(15.0),
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
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                UiImage::new(
                                                        asset_server.load(MINING_ICON_GRAY),
                                                    ),
                                                Name::new("Mining Icon"),
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
                                                Name::new("TextDashboard"),
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
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Nav Bar Bottom Half"),
                    )).with_children(|parent| {
                        // Bottom Nav Children.
                    });
                });
                parent.spawn((
                    NodeBundle {
                        border_color: Color::BLUE.into(),
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("App Screen"),
                )).with_children(|parent| {
                    //App Screen Children
                    // Defaults to Dashboard/Mining
                });
            });
        });
}