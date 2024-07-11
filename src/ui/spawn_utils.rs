use crate::utils::shorten_string;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};

use super::{components::*, styles::*};

pub struct UiListItem {
    pub id: String,
    pub landed_at: String,
    pub sig: String,
    pub tx_time: String,
    pub hash_time: String,
    pub status: String,
}

pub fn spawn_new_list_item(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scroll_panel_entity: Entity,
    item_data: UiListItem,
) {
    let sig = shorten_string(item_data.sig.clone(), 10);
    let new_result_item = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    // padding: UiRect::left(Val::Px(20.0)),
                    // column_gap: Val::Px(30.0),
                    // justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
            Name::new("TxResult Item"),
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
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
                        item_data.id,
                        TextStyle {
                            font: asset_server.load(FONT_REGULAR),
                            font_size: FONT_SIZE_MEDIUM,
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

                let pieces: Vec<&str> = item_data.landed_at.split(' ').collect();

                let value = if pieces.len() > 2 {
                    String::from(pieces[1])
                } else {
                    item_data.landed_at
                };

                parent.spawn((
                    TextBundle::from_section(
                            value,
                        TextStyle {
                            font: asset_server.load(FONT_REGULAR),
                            font_size: FONT_SIZE_MEDIUM,
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
                spawn_web_link_icon(parent, asset_server, item_data.sig.clone(), sig);
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
                        item_data.tx_time,
                        TextStyle {
                            font: asset_server.load(FONT_REGULAR),
                            font_size: FONT_SIZE_MEDIUM,
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
                        item_data.hash_time,
                        TextStyle {
                            font: asset_server.load(FONT_REGULAR),
                            font_size: FONT_SIZE_MEDIUM,
                            ..default()
                        },
                    ),
                    Label,
                ));
            });

            parent.spawn((
                NodeBundle {
                    style: Style {
                        min_height: Val::Px(20.0),
                        max_height: Val::Px(60.0),
                        width: Val::Px(400.0),
                        overflow: Overflow {
                            x: OverflowAxis::Clip,
                            y: OverflowAxis::Clip,
                        },
                        margin: UiRect::left(Val::Px(10.0)),
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Status"),
            )).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        item_data.status,
                        TextStyle {
                            font: asset_server.load(FONT_REGULAR),
                            font_size: FONT_SIZE_MEDIUM,
                            ..default()
                        },
                    ),
                    Label,
                ));
            });
        })
        .id();

    commands
        .entity(scroll_panel_entity)
        .add_child(new_result_item);
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
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    padding: UiRect {
                        top: Val::Px(0.0),
                        left: Val::Px(15.0),
                        right: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
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
                        font: asset_server.load(FONT_REGULAR),
                        font_size: FONT_SIZE_MEDIUM,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                Name::new("WalletPubkeyText"),
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(20.96),
                            height: Val::Px(20.96),
                            padding: UiRect::left(Val::Px(20.0)),
                            // border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            // justify_content: JustifyContent::Center,
                            // vertically center child text
                            // align_items: AlignItems::Center,
                            ..default()
                        },
                        image: UiImage::new(asset_server.load(BUTTON_COPY_TEXT)),
                        ..default()
                    },
                    ButtonCopyText,
                    Name::new("ButtonCopyText"),
                ));
        });
}

pub fn spawn_web_link_icon(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    copy_text: String,
    display_text: String,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    // padding: UiRect {
                    //     top: Val::Px(0.0),
                    //     left: Val::Px(15.0),
                    //     right: Val::Px(0.0),
                    //     bottom: Val::Px(0.0),
                    // },
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
                        font: asset_server.load(FONT_REGULAR),
                        font_size: FONT_SIZE_MEDIUM,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                Name::new("WalletPubkeyText"),
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(14.0),
                            height: Val::Px(14.0),
                            margin: UiRect::left(Val::Px(8.0)),
                            // border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            // justify_content: JustifyContent::Center,
                            // vertically center child text
                            // align_items: AlignItems::Center,
                            ..default()
                        },
                        image: UiImage::new(asset_server.load(SOLANA_ICON)),
                        ..default()
                    },
                    ButtonOpenWebTxExplorer,
                    Name::new("ButtonCopyText"),
                ));
        });
}

pub fn spawn_fps_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
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
        ))
        .id();
    // create our text
    let text_fps = commands
        .spawn((
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
                        },
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
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

pub fn spawn_app_screen_mining(parent: &mut ChildBuilder, asset_server: &AssetServer) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        Name::new("Mining App Screen"),
    )).with_children(|parent| {
        // Top Data Section
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(70.0),
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
                            "420.696969 ORE",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextTreasuryOreBalance"),
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
                            "0.000032389 ORE",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextBaseRewardRate"),
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
                            "2024-00-00 00:00:00 UTC",
                            TextStyle {
                                font: asset_server.load(FONT_REGULAR),
                                font_size: FONT_SIZE_MEDIUM,
                                color: hex_dark_mode_text_gray().into()
                            },
                        ),
                        Name::new("TextLastResetAt"),
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
                                "1: 0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus1"),
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
                                "2: 0.64323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus2"),
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
                                "3: 0.438873254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus3"),
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
                                "4: 0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus4"),
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
                                "5: 0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus5"),
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
                                "6: 0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus6"),
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
                                "7: 0.59323254732",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus7"),
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
                                "8: 0.650000000",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
                                    font_size: FONT_SIZE_MEDIUM,
                                    color: hex_dark_mode_text_gray().into()
                                },
                            ),
                            Name::new("TextBus8"),
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
                                height: Val::Percent(15.0),
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
                        ));
                    });

                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(15.0),
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
                        ));
                    });

                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(15.0),
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
                        ));
                    });

                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(15.0),
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
                                height: Val::Percent(15.0),
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
                        ));
                    });

                    // Last Hash At
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(15.0),
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
                        ));
                    });

                    // Total Hashes
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(15.0),
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
                        ));
                    });

                    // Staked
                    parent.spawn((
                        NodeBundle {
                            background_color: hex_dark_mode_nav_title().into(),
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(15.0),
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
                    height: Val::Percent(30.0),
                    ..default()
                },
                ..default()
            },
            Name::new("Mining App Screen Logs Section"),
        )).with_children(|parent| {
        });
    });
}
