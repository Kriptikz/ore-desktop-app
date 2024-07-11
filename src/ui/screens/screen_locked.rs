use bevy::prelude::*;

use crate::ui::{
    components::{
        BaseScreenNode, ButtonCaptureTextInput, ButtonUnlock, LockedScreenNode, TextCursor, TextInput, TextPasswordInput, TextPasswordLabel
    },
    styles::{FONT_SIZE_SMALL, NORMAL_BUTTON},
};

pub fn spawn_locked_screen(
    parent: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
) -> Option<Entity> {
    let mut password_capture_text_entity = None;
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("App Node"),
            LockedScreenNode,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        z_index: ZIndex::Global(10),
                        style: Style {
                            justify_content: JustifyContent::Center,
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
                                        font_size: FONT_SIZE_SMALL,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                TextPasswordLabel,
                            ));
                            password_capture_text_entity = Some(
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
                                                    font_size: FONT_SIZE_SMALL,
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
                                    })
                                    .id(),
                            );
                        });
                });
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Button Unlock Node"),
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
                                        bottom: Val::Px(0.0),
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
                            ButtonUnlock,
                            Name::new("ButtonUnlock"),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "UNLOCK",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: FONT_SIZE_SMALL,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });

    password_capture_text_entity
}

pub fn despawn_locked_screen(
    mut commands: Commands,
    query: Query<Entity, With<LockedScreenNode>>,
) {
    let screen_node = query.get_single().unwrap();
    commands.entity(screen_node).despawn_recursive();
}


