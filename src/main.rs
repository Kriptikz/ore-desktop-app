use std::fs::File;

use bevy::prelude::*;
use cocoon::Cocoon;
use solana_sdk::signature::{Keypair, Signer};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct WalletPubkeyText;

#[derive(Component)]
pub struct ButtonGenerateWallet;

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (
            Changed<Interaction>,
            With<ButtonGenerateWallet>,
        ),
    >,
    mut wallet_pubkey_text_query: Query<&mut Text, With<WalletPubkeyText>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        let mut wallet_pubkey_text = wallet_pubkey_text_query.get_single_mut().unwrap();

        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                let wallet = Keypair::new();

                wallet_pubkey_text.sections[0].value = wallet.pubkey().to_string();

                let wallet_bytes = wallet.to_bytes();

                let mut cocoon = Cocoon::new(b"secret password");
                let mut file = File::create("target/encrypted_data.data").unwrap();

                // Dump the serialized database into a file as an encrypted container.
                let container = cocoon.dump(wallet_bytes.to_vec(), &mut file).unwrap();

                // Let's look at how to decrypt the container and parse it back.
                // let mut file = File::open("target/encrypted_data.data").unwrap();
                // let encoded = cocoon.parse(&mut file).unwrap();
                // let decoded = Database::try_from_slice(&encoded).unwrap();

            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Generate A New Wallet...",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                WalletPubkeyText,
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(30.0),
                            border: UiRect::all(Val::Px(5.0)),
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
                    ButtonGenerateWallet,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Generate Wallet",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
