use bevy::{ecs::storage::Column, prelude::*};

use crate::ui::{
    components::{
        BaseScreenNode, ButtonCaptureTextInput, ButtonUnlock, DashboardProofUpdatesLogsList, DashboardScreenNode, LockedScreenNode, MovingScrollPanel, ScrollingList, ScrollingListNode, TextCursor, TextInput, TextPasswordInput, TextPasswordLabel
    },
    styles::{hex_dark_mode_text_white_2, FONT_REGULAR, FONT_SIZE_LARGE, FONT_SIZE_SMALL, NORMAL_BUTTON},
};

pub fn spawn_dashboard_screen(
    parent: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("App Node"),
            DashboardScreenNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                Name::new("DashboardProofUpdatesLogs Section"),
            )).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(95.0),
                            height: Val::Percent(40.0),
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::Start,
                            flex_direction: FlexDirection::Column,
                            overflow: Overflow::clip_y(),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Scrolling List Node"),
                    ScrollingListNode(true),
                )).with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScrollingList::default(),
                        MovingScrollPanel,
                        DashboardProofUpdatesLogsList,
                        Name::new("MovingScrollPanel"),
                    ));
                });
            });
        });
}

pub fn despawn_dashboard_screen(
    mut commands: Commands,
    query: Query<Entity, With<DashboardScreenNode>>,
) {
    let screen_node = query.get_single().unwrap();
    commands.entity(screen_node).despawn_recursive();
}


