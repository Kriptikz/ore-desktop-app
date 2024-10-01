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
    use_light_background: bool,
) {
    let sig = shorten_string(item_data.sig.clone(), 10);
    let background_color = if use_light_background {
        hex_dark_mode_app_screen_background()
    } else {
        hex_dark_mode_nav_title()
    };

    let new_result_item = commands
        .spawn((
            NodeBundle {
                background_color: background_color.into(),
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(99.0),
                    height: Val::Px(25.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    // padding: UiRect::left(Val::Px(20.0)),
                    // column_gap: Val::Px(30.0),
                    // justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
            Name::new("TxResult Item"),
            UiImage::new(
                    asset_server.load(LOG_ITEMS_BACKGROUND),
                ),
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
                        item_data.hash_time,
                        TextStyle {
                            font: asset_server.load(FONT_REGULAR),
                            font_size: FONT_SIZE_MEDIUM,
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
                            color: hex_dark_mode_text_gray().into(),
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
                        color: hex_dark_mode_text_gray().into(),
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
