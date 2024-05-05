use bevy::prelude::*;

use crate::ui::{
    components::{
        BaseScreenNode, ButtonCaptureTextInput, ButtonTest, InitialSetupScreenNode,
        TextInput,
    },
    styles::{FONT_REGULAR, FONT_SIZE, MENU_BACKGROUND, SCREEN_BACKGROUND_1},
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
                            justify_content: JustifyContent::Center,
                            width: Val::Percent(50.0),
                            height: Val::Percent(80.0),
                            align_items: AlignItems::Center,
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
                            ButtonBundle {
                                //background_color: Color::WHITE.into(),
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(50.0),
                                    margin: UiRect {
                                        top: Val::Percent(0.0),
                                        right: Val::Px(0.0),
                                        left: Val::Px(0.0),
                                        bottom: Val::Px(200.0),
                                    },
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("button.png")),
                                //border_color: BorderColor(Color::BLACK),
                                ..default()
                            },
                            ButtonCaptureTextInput,
                            ButtonTest,
                            Name::new("ButtonTest"),
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
                                    text: "Click to Edit".to_string(),
                                },
                            ));
                        });
                });
        });
}
