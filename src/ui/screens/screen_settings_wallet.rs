use bevy::prelude::*;

use crate::ui::{
    components::
        SettingsWalletScreenNode
    ,
    styles::{
        hex_dark_mode_text_white_2, FONT_REGULAR, FONT_SIZE_LARGE
    },
};

pub fn spawn_settings_wallet_screen(
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
            Name::new("App Screen Node"),
            SettingsWalletScreenNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "WALLET SETTINGS",
                    TextStyle {
                        font: asset_server.load(FONT_REGULAR),
                        font_size: FONT_SIZE_LARGE,
                        color: hex_dark_mode_text_white_2().into()
                    },
                ),
                Name::new("TextSETTINGSWALLETAPPSCREENBACKGROUND"),
            ));
        });
}


pub fn despawn_settings_wallet_screen(
    mut commands: Commands,
    query: Query<Entity, With<SettingsWalletScreenNode>>,
) {
    let screen_node = query.get_single().unwrap();
    commands.entity(screen_node).despawn_recursive();
}
