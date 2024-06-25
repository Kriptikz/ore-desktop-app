
use bevy::
    prelude::*
;

use crate::ui::styles::{hex_dark_mode_background, ORE_LOGO_WHITE};
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
                    border: UiRect::all(Val::Px(5.0)),
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
                        border: UiRect::all(Val::Px(5.0)),
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
                            border: UiRect::all(Val::Px(5.0)),
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
                                border: UiRect::all(Val::Px(5.0)),
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
                            border: UiRect::all(Val::Px(5.0)),
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
                                border: UiRect::all(Val::Px(5.0)),
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
                                border: UiRect::all(Val::Px(5.0)),
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
                                border: UiRect::all(Val::Px(5.0)),
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
                        border: UiRect::all(Val::Px(5.0)),
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
                            border: UiRect::all(Val::Px(5.0)),
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
                                border: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Nav Bar Top Half"),
                    )).with_children(|parent| {
                        // Top Nav Children.
                    });
                    parent.spawn((
                        NodeBundle {
                            border_color: Color::RED.into(),
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(50.0),
                                flex_direction: FlexDirection::Column,
                                border: UiRect::all(Val::Px(5.0)),
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
                            border: UiRect::all(Val::Px(5.0)),
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