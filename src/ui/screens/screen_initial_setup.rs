use bevy::prelude::*;

use crate::{ui::{
    components::{
        BaseScreenNode, ButtonCaptureTextInput, ButtonSaveConfig, InitialSetupScreenNode,
        TextConfigInputRpcFetchAccountsInterval, TextConfigInputRpcSendTxInterval,
        TextConfigInputRpcUrl, TextConfigInputThreads, TextCursor, TextInput,
    },
    styles::{
        BUTTON, BUTTON_SAVE_CONFIG, CURRENT_TX_STATUS_BACKGROUND, FONT_ROBOTO, FONT_ROBOTO_MEDIUM, FONT_SIZE, FONT_SIZE_LARGE, FONT_SIZE_TITLE, MENU_BACKGROUND, SCREEN_BACKGROUND_1, SETTINGS_ICON, TITLE_BACKGROUND, TREASURY_BACKGROUND
    },
}, AppConfig};

pub fn spawn_initial_setup_screen(mut commands: Commands, asset_server: Res<AssetServer>, config: AppConfig) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            Name::new("Screen Node"),
            BaseScreenNode,
            InitialSetupScreenNode,
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
            parent
                .spawn((
                    NodeBundle {
                        z_index: ZIndex::Global(10),
                        style: Style {
                            //justify_content: JustifyContent::Center,
                            width: Val::Percent(60.0),
                            height: Val::Percent(77.7),
                            align_self: AlignSelf::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect {
                                top: Val::Px(100.0),
                                left: Val::Px(0.0),
                                right: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                            },
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load(TREASURY_BACKGROUND)),
                    Name::new("Config Setup Node"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    width: Val::Px(186.5),
                                    height: Val::Px(80.0),
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(40.0),
                                    margin: UiRect {
                                        top: Val::Px(40.0),
                                        left: Val::Px(0.0),
                                        right: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                    },
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            UiImage::new(asset_server.load(CURRENT_TX_STATUS_BACKGROUND)),
                            Name::new("Config Title"),
                        ))
                        .with_children(|parent| {
                            // parent.spawn((
                            //     NodeBundle {
                            //         style: Style {
                            //             width: Val::Px(90.0),
                            //             height: Val::Px(60.0),
                            //             align_items: AlignItems::Center,
                            //             justify_content: JustifyContent::Center,
                            //             margin: UiRect {
                            //                 top: Val::Px(0.0),
                            //                 left: Val::Px(20.0),
                            //                 right: Val::Px(0.0),
                            //                 bottom: Val::Px(0.0),
                            //             },
                            //             ..default()
                            //         },
                            //         background_color: Color::WHITE.into(),
                            //         ..default()
                            //     },
                            //     UiImage::new(asset_server.load(SETTINGS_ICON)),
                            //     Name::new("Settings Icon"),
                            // ));
                            parent.spawn(TextBundle::from_section(
                                "Config Setup",
                                TextStyle {
                                    font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                    font_size: FONT_SIZE_TITLE,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(75.0),
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Row,
                                    // row_gap: Val::Px(30.0),
                                    ..default()
                                },
                                //background_color: Color::WHITE.into(),
                                ..default()
                            },
                            //UiImage::new(asset_server.load(MENU_BACKGROUND)),
                            Name::new("Config Input Node"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(30.0),
                                            height: Val::Percent(60.0),
                                            padding: UiRect::right(Val::Px(20.0)),
                                            // flex_direction: FlexDirection::Column,
                                            // align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        //background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    //UiImage::new(asset_server.load(TITLE_BACKGROUND)),
                                    Name::new("Config Input Field Headers Wrapper"),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    flex_direction: FlexDirection::Column,
                                                    height: Val::Percent(100.0),
                                                    width: Val::Percent(100.0),
                                                    justify_content: JustifyContent::SpaceBetween,
                                                    align_items: AlignItems::End,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            Name::new("Config Input Field Headers"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "RPC URL: ",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                                    font_size: FONT_SIZE_TITLE,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                            ));
                                            parent.spawn(TextBundle::from_section(
                                                "Threads: ",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                                    font_size: FONT_SIZE_TITLE,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                            ));
                                            parent.spawn(TextBundle::from_section(
                                                "UI Fetch Interval (ms): ",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                                    font_size: FONT_SIZE_TITLE,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                            ));
                                            parent.spawn(TextBundle::from_section(
                                                "Tx Send Interval (ms): ",
                                                TextStyle {
                                                    font: asset_server.load(FONT_ROBOTO_MEDIUM),
                                                    font_size: FONT_SIZE_TITLE,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                            ));
                                        });
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            height: Val::Px(268.0),
                                            width: Val::Px(351.0),
                                            align_items: AlignItems::Start,
                                            justify_content: JustifyContent::SpaceBetween,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Config Input Field Values"),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(300.0),
                                                    height: Val::Px(50.0),
                                                    justify_content: JustifyContent::Start,
                                                    align_items: AlignItems::Center,
                                                    overflow: Overflow {
                                                        x: OverflowAxis::Clip,
                                                        y: OverflowAxis::Clip,
                                                    },
                                                    padding: UiRect::left(Val::Px(10.0)),
                                                    ..default()
                                                },
                                                image: UiImage::new(
                                                    asset_server.load(CURRENT_TX_STATUS_BACKGROUND),
                                                ),
                                                ..default()
                                            },
                                            ButtonCaptureTextInput,
                                            Name::new("ButtonCaptureText RPC URL"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE_LARGE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextInput {
                                                    hidden: false,
                                                    numbers_only: false,
                                                    text: config.rpc_url,
                                                },
                                                TextConfigInputRpcUrl,
                                                Name::new("TextConfigRpcUrl"),
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    visibility: Visibility::Hidden,
                                                    style: Style {
                                                        width: Val::Px(10.0),
                                                        height: Val::Px(15.0),
                                                        ..default()
                                                    },
                                                    background_color: Color::WHITE.into(),
                                                    ..default()
                                                },
                                                TextCursor,
                                                Name::new("TextCursor"),
                                            ));
                                        });
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(60.0),
                                                    height: Val::Px(40.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                image: UiImage::new(
                                                    asset_server.load(CURRENT_TX_STATUS_BACKGROUND),
                                                ),
                                                ..default()
                                            },
                                            ButtonCaptureTextInput,
                                            Name::new("ButtonCaptureText Threads"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE_LARGE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextInput {
                                                    hidden: false,
                                                    numbers_only: true,
                                                    text: config.threads.to_string(),
                                                },
                                                TextConfigInputThreads,
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    visibility: Visibility::Hidden,
                                                    style: Style {
                                                        width: Val::Px(10.0),
                                                        height: Val::Px(15.0),
                                                        ..default()
                                                    },
                                                    background_color: Color::WHITE.into(),
                                                    ..default()
                                                },
                                                TextCursor,
                                                Name::new("TextCursor"),
                                            ));
                                        });
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(60.0),
                                                    height: Val::Px(40.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                image: UiImage::new(
                                                    asset_server.load(CURRENT_TX_STATUS_BACKGROUND),
                                                ),
                                                ..default()
                                            },
                                            ButtonCaptureTextInput,
                                            Name::new("ButtonCaptureText"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE_LARGE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextInput {
                                                    hidden: false,
                                                    numbers_only: true,
                                                    text: config.ui_fetch_interval.to_string(),
                                                },
                                                TextConfigInputRpcFetchAccountsInterval,
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    visibility: Visibility::Hidden,
                                                    style: Style {
                                                        width: Val::Px(10.0),
                                                        height: Val::Px(15.0),
                                                        ..default()
                                                    },
                                                    background_color: Color::WHITE.into(),
                                                    ..default()
                                                },
                                                TextCursor,
                                                Name::new("TextCursor"),
                                            ));
                                        });
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(60.0),
                                                    height: Val::Px(40.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                image: UiImage::new(
                                                    asset_server.load(CURRENT_TX_STATUS_BACKGROUND),
                                                ),
                                                ..default()
                                            },
                                            ButtonCaptureTextInput,
                                            Name::new("ButtonCaptureText"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE_LARGE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextInput {
                                                    hidden: false,
                                                    numbers_only: true,
                                                    text: config.tx_send_interval.to_string(),
                                                },
                                                TextConfigInputRpcSendTxInterval,
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    visibility: Visibility::Hidden,
                                                    style: Style {
                                                        width: Val::Px(10.0),
                                                        height: Val::Px(15.0),
                                                        ..default()
                                                    },
                                                    background_color: Color::WHITE.into(),
                                                    ..default()
                                                },
                                                TextCursor,
                                                Name::new("TextCursor"),
                                            ));
                                        });
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            position_type: PositionType::Absolute,
                                            justify_content: JustifyContent::Center,
                                            left: Val::Percent(30.0),
                                            width: Val::Percent(40.0),
                                            height: Val::Percent(10.0),
                                            align_items: AlignItems::Center,
                                            align_self: AlignSelf::End,
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        //background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    //UiImage::new(asset_server.load(TITLE_BACKGROUND)),
                                    Name::new("Config Input Section"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(150.0),
                                                height: Val::Px(52.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            image: UiImage::new(
                                                asset_server.load(BUTTON_SAVE_CONFIG),
                                            ),
                                            ..default()
                                        },
                                        ButtonSaveConfig,
                                        Name::new("ButtonSaveConfig"),
                                    ));
                                });
                        });
                });
        });
}
