use bevy::prelude::*;

use crate::ui::{
    components::{
        BaseScreenNode, ButtonCaptureTextInput, ButtonSaveConfig, InitialSetupScreenNode, TextConfigInputRpcFetchAccountsInterval, TextConfigInputRpcSendTxInterval, TextConfigInputRpcUrl, TextConfigInputThreads, TextCursor, TextInput
    },
    styles::{
        BUTTON, FONT_REGULAR, FONT_SIZE, FONT_SIZE_TITLE, MENU_BACKGROUND, SCREEN_BACKGROUND_1, SETTINGS_ICON, TITLE_BACKGROUND
    },
};

pub fn spawn_initial_setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            Name::new("Screen Node"),
            BaseScreenNode,
            InitialSetupScreenNode,
            UiImage::new(asset_server.load(SCREEN_BACKGROUND_1)),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        z_index: ZIndex::Global(10),
                        style: Style {
                            //justify_content: JustifyContent::Center,
                            width: Val::Percent(70.0),
                            height: Val::Percent(90.0),
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load(MENU_BACKGROUND)),
                    Name::new("Config Setup Node"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    width: Val::Px(315.0),
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
                            UiImage::new(asset_server.load(TITLE_BACKGROUND)),
                            Name::new("Config Title"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(90.0),
                                        height: Val::Px(60.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        margin: UiRect {
                                            top: Val::Px(0.0),
                                            left: Val::Px(20.0),
                                            right: Val::Px(0.0),
                                            bottom: Val::Px(0.0),
                                        },
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    ..default()
                                },
                                UiImage::new(asset_server.load(SETTINGS_ICON)),
                                Name::new("Settings Icon"),
                            ));
                            parent.spawn(TextBundle::from_section(
                                "Config Setup",
                                TextStyle {
                                    font: asset_server.load(FONT_REGULAR),
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
                                    width: Val::Percent(75.0),
                                    height: Val::Percent(75.0),
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(40.0),
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
                                            justify_content: JustifyContent::Center,
                                            width: Val::Percent(60.0),
                                            height: Val::Percent(10.0),
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Row,
                                            row_gap: Val::Px(10.0),
                                            ..default()
                                        },
                                        //background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    //UiImage::new(asset_server.load(TITLE_BACKGROUND)),
                                    Name::new("Config Input Section"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((TextBundle::from_section(
                                        "RPC URL: ",
                                        TextStyle {
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ).with_style(Style {
                                        width: Val::Px(200.0),
                                        ..Default::default()
                                    }),));
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(500.0),
                                                    height: Val::Px(60.0),
                                                    justify_content: JustifyContent::Start,
                                                    align_items: AlignItems::Center,
                                                    overflow: Overflow {
                                                        x: OverflowAxis::Clip,
                                                        y: OverflowAxis::Clip
                                                    },
                                                    padding: UiRect::left(Val::Px(10.0)),
                                                    ..default()
                                                },
                                                image: UiImage::new(asset_server.load(BUTTON)),
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
                                                        font: asset_server.load(FONT_REGULAR),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextInput {
                                                    hidden: false,
                                                    numbers_only: false,
                                                    text: "www.rpcpool.com".to_string(),
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
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            justify_content: JustifyContent::Center,
                                            width: Val::Percent(60.0),
                                            height: Val::Percent(10.0),
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Row,
                                            row_gap: Val::Px(10.0),
                                            ..default()
                                        },
                                        //background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    //UiImage::new(asset_server.load(TITLE_BACKGROUND)),
                                    Name::new("Config Input Section"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((TextBundle::from_section(
                                        "Threads: ",
                                        TextStyle {
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ),));
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(100.0),
                                                    height: Val::Px(60.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                image: UiImage::new(asset_server.load(BUTTON)),
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
                                                        font: asset_server.load(FONT_REGULAR),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextInput {
                                                    hidden: false,
                                                    numbers_only: true,
                                                    text: "1".to_string(),
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
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Row,
                                            row_gap: Val::Px(10.0),
                                            ..default()
                                        },
                                        //background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    //UiImage::new(asset_server.load(TITLE_BACKGROUND)),
                                    Name::new("Config Input Section"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((TextBundle::from_section(
                                        "RPC Fetch Accounts Interval (ms): ",
                                        TextStyle {
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ).with_style(Style {
                                        width: Val::Px(400.0),
                                        ..Default::default()
                                    }),));
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(100.0),
                                                    height: Val::Px(60.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                image: UiImage::new(asset_server.load(BUTTON)),
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
                                                        font: asset_server.load(FONT_REGULAR),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextInput {
                                                    hidden: false,
                                                    numbers_only: true,
                                                    text: "3000".to_string(),
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
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Row,
                                            row_gap: Val::Px(10.0),
                                            ..default()
                                        },
                                        //background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    //UiImage::new(asset_server.load(TITLE_BACKGROUND)),
                                    Name::new("Config Input Section"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((TextBundle::from_section(
                                        "RPC Send Interval (ms): ",
                                        TextStyle {
                                            font: asset_server.load(FONT_REGULAR),
                                            font_size: FONT_SIZE,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ).with_style(Style {
                                        width: Val::Px(400.0),
                                        ..Default::default()
                                    }),));
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(100.0),
                                                    height: Val::Px(60.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                image: UiImage::new(asset_server.load(BUTTON)),
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
                                                        font: asset_server.load(FONT_REGULAR),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextInput {
                                                    hidden: false,
                                                    numbers_only: true,
                                                    text: "2000".to_string(),
                                                },
                                                TextConfigInputRpcSendTxInterval
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
                                            justify_content: JustifyContent::Center,
                                            width: Val::Percent(40.0),
                                            height: Val::Percent(10.0),
                                            align_items: AlignItems::Center,
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
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(200.0),
                                                    height: Val::Px(50.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                image: UiImage::new(asset_server.load(BUTTON)),
                                                ..default()
                                            },
                                            ButtonSaveConfig,
                                            Name::new("ButtonSaveConfig"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((TextBundle::from_section(
                                                "Save Config",
                                                TextStyle {
                                                    font: asset_server.load(FONT_REGULAR),
                                                    font_size: FONT_SIZE,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                            ),));
                                        });
                                });
                        });
                });
        });
}
