use bevy::prelude::*;

use crate::ui::{
    components::{
        BaseScreenNode, ButtonCaptureTextInput, ButtonGenerateWallet, ButtonSaveConfig, InitialSetupScreenNode, TextConfigInputRpcFetchAccountsInterval, TextConfigInputRpcSendTxInterval, TextConfigInputRpcUrl, TextConfigInputThreads, TextCursor, TextGeneratedPubkey, TextInput
    }, spawn_utils::spawn_copyable_text, styles::{
        BUTTON, BUTTON_GENERATE, BUTTON_SAVE_CONFIG, CURRENT_TX_STATUS_BACKGROUND, FONT_ROBOTO, FONT_ROBOTO_MEDIUM, FONT_SIZE, FONT_SIZE_LARGE, FONT_SIZE_TITLE, MENU_BACKGROUND, SCREEN_BACKGROUND_1, SETTINGS_ICON, TITLE_BACKGROUND, TREASURY_BACKGROUND
    }
};

pub fn spawn_wallet_setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        width: Val::Percent(100.0),
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
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(75.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            // row_gap: Val::Px(30.0),
                            ..default()
                        },
                        //background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load(TREASURY_BACKGROUND)),
                    Name::new("Wallet Input Node"),
                ))
                .with_children(|parent| {
                    // wallet setup fields
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(40.0),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    // row_gap: Val::Px(30.0),
                                    ..default()
                                },
                                //background_color: Color::WHITE.into(),
                                ..default()
                            },
                            Name::new("Wallet Generator Node"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(30.0),
                                            height: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            // row_gap: Val::Px(30.0),
                                            ..default()
                                        },
                                        //background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    Name::new("Generator Left Side"),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(30.0),
                                                    height: Val::Percent(100.0),
                                                    flex_direction: FlexDirection::Column,
                                                    align_items: AlignItems::Center,
                                                    justify_content: JustifyContent::Center,
                                                    // row_gap: Val::Px(30.0),
                                                    ..default()
                                                },
                                                //background_color: Color::WHITE.into(),
                                                ..default()
                                            },
                                            Name::new("Generate Button Node"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        width: Val::Px(164.53),
                                                        height: Val::Px(38.0),
                                                        ..default()
                                                    },
                                                    image: UiImage::new(
                                                        asset_server.load(BUTTON_GENERATE),
                                                    ),
                                                    ..default()
                                                },
                                                ButtonGenerateWallet,
                                                Name::new("ButtonGenerateWallet"),
                                            ));
                                        });
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(30.0),
                                                    height: Val::Percent(100.0),
                                                    flex_direction: FlexDirection::Row,
                                                    align_items: AlignItems::Center,
                                                    justify_content: JustifyContent::Center,
                                                    // row_gap: Val::Px(30.0),
                                                    ..default()
                                                },
                                                //background_color: Color::WHITE.into(),
                                                ..default()
                                            },
                                            Name::new("Generated Pubkey Node"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Pubkey: ",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                Name::new("Text Generated Pubkey Header"),
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Click Generate to make a new key.",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                Name::new("Text Generated Pubkey Value"),
                                                TextGeneratedPubkey
                                            ));
                                            // spawn_copyable_text(
                                            //     parent,
                                            //     &asset_server,
                                            //     " ".to_string(),
                                            //     " ".to_string(),
                                            // );
                                        });
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(60.0),
                                            height: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            // row_gap: Val::Px(30.0),
                                            ..default()
                                        },
                                        //background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                    Name::new("Generator Right Side"),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(30.0),
                                                    height: Val::Percent(100.0),
                                                    flex_direction: FlexDirection::Row,
                                                    align_items: AlignItems::Center,
                                                    justify_content: JustifyContent::Center,
                                                    // row_gap: Val::Px(30.0),
                                                    ..default()
                                                },
                                                //background_color: Color::WHITE.into(),
                                                ..default()
                                            },
                                            Name::new("Generated Seed Phrase Node"),
                                        ))
                                        .with_children(|parent| {
                                            parent
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            width: Val::Percent(100.0),
                                                            height: Val::Percent(20.0),
                                                            flex_direction: FlexDirection::Row,
                                                            align_items: AlignItems::Center,
                                                            justify_content: JustifyContent::Center,
                                                            // row_gap: Val::Px(30.0),
                                                            ..default()
                                                        },
                                                        //background_color: Color::WHITE.into(),
                                                        ..default()
                                                    },
                                                    Name::new("Seed Phrase Line"),
                                                ))
                                                .with_children(|parent| {
                                                });
                                        });

                                });
                        });
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(30.0),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    // row_gap: Val::Px(30.0),
                                    ..default()
                                },
                                //background_color: Color::WHITE.into(),
                                ..default()
                            },
                            Name::new("Password Inputs Node"),
                        ))
                        .with_children(|parent| {});
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(20.0),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    // row_gap: Val::Px(30.0),
                                    ..default()
                                },
                                //background_color: Color::WHITE.into(),
                                ..default()
                            },
                            Name::new("Save Button Node"),
                        ))
                        .with_children(|parent| {});
                });
        });
}
