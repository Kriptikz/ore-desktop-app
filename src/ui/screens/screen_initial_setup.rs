use bevy::prelude::*;

use crate::ui::{components::{
    BaseScreenNode, ButtonCaptureTextInput,
    InitialSetupScreenNode, TextInput,
}, styles::{FONT_SIZE, NORMAL_BUTTON}};


pub fn spawn_initial_setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("Screen Node"),
            BaseScreenNode,
            InitialSetupScreenNode,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        z_index: ZIndex::Global(10),
                        style: Style {
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Button Capture Text Node"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(100.0),
                                    height: Val::Px(50.0),
                                    border: UiRect::all(Val::Px(5.0)),
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
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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
