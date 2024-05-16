use std::sync::Arc;

use bevy::prelude::*;
use solana_sdk::signature::Keypair;

use crate::ui::{
    components::{
        BaseScreenNode, ButtonCaptureTextInput, ButtonGenerateWallet,
        ButtonSaveGeneratedWallet, InitialSetupScreenNode,
        TextCursor, TextGeneratedKeypair, TextInput, TextMnemonicLine1, TextMnemonicLine2,
        TextMnemonicLine3, TextPasswordInput, TextPasswordLabel, WalletSetupScreenNode,
    },
    styles::{
        BUTTON_GENERATE, BUTTON_SAVE_WALLET, FONT_ROBOTO, FONT_SIZE, NORMAL_BUTTON, TREASURY_BACKGROUND
    },
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
            WalletSetupScreenNode,
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
                                                        font: asset_server.load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                Name::new("Text Generated Pubkey Header"),
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "Click Generate to make a new key. OR Drag-&-Drop a `.json` key file to import it.",
                                                    TextStyle {
                                                        font: asset_server.load(FONT_ROBOTO),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                Name::new("Text Generated Pubkey Value"),
                                                TextGeneratedKeypair(Arc::new(Keypair::new())),
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
                                                    width: Val::Percent(100.0),
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
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            " ",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("Text Mnemonic Phrase Line 1"),
                                                        TextMnemonicLine1,
                                                    ));
                                                });
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
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            " ",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("Text Mnemonic Phrase Line 2"),
                                                        TextMnemonicLine2,
                                                    ));
                                                });
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
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            " ",
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load(FONT_ROBOTO),
                                                                font_size: FONT_SIZE,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ),
                                                        Name::new("Text Mnemonic Phrase Line 3"),
                                                        TextMnemonicLine3,
                                                    ));
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
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Password Input Field"),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Password: ",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: FONT_SIZE,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        TextPasswordLabel,
                                    ));
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(150.0),
                                                    height: Val::Px(50.0),
                                                    border: UiRect::all(Val::Px(2.5)),
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
                                            ButtonCaptureTextInput,
                                            Name::new("ButtonCaptureText"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: FONT_SIZE,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                TextInput {
                                                    hidden: true,
                                                    numbers_only: false,
                                                    text: "".to_string(),
                                                },
                                                TextPasswordInput,
                                            ));
                                            parent.spawn((
                                                NodeBundle {
                                                    visibility: Visibility::Hidden,
                                                    style: Style {
                                                        width: Val::Px(8.0),
                                                        height: Val::Px(18.0),
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
                        });
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
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(164.53),
                                        height: Val::Px(38.0),
                                        ..default()
                                    },
                                    image: UiImage::new(asset_server.load(BUTTON_SAVE_WALLET)),
                                    ..default()
                                },
                                ButtonSaveGeneratedWallet,
                                Name::new("ButtonSaveGeneratedWallet"),
                            ));
                        });
                });
        });
}
