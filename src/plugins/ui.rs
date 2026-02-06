use bevy::prelude::*;

use crate::plugins::player::Player;

pub struct GameUiPlugin;

#[derive(Component)]
struct PlayerCoinText;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(FixedUpdate, update_player_coin_ui);
    }
}

fn update_player_coin_ui(
    mut text: Single<&mut Text, With<PlayerCoinText>>,
    player: Query<&Player, Changed<Player>>,
) {
    if let Ok(player) = player.single() {
        text.0 = format!("coins: {}", player.coins);
    }
}

fn spawn_ui(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: px(110),
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(px(10)),
            ..default()
        },
        children![
            (Node {
                width: px(100),
                height: px(100),
                ..default()
            }),
            (
                Text::new("hey"),
                PlayerCoinText,
                Node {
                    width: px(100),
                    height: px(100),
                    ..default()
                }
            ),
            (
                Node {
                    width: px(200),
                    height: px(200),
                    border_radius: BorderRadius::all(percent(100)),

                    margin: UiRect::top(px(150)).with_right(px(20)),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(50, 50, 50)),
                children![
                    (
                        Node {
                            width: percent(33),
                            height: percent(100),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        children![(
                            ImageNode::new(asset_server.load("Y.png")),
                            Node {
                                width: px(40),
                                height: px(40),
                                margin: UiRect::left(px(-60)),
                                ..default()
                            }
                        )]
                    ),
                    (
                        Node {
                            width: percent(33),
                            height: percent(100),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        children![
                            (
                                ImageNode::new(asset_server.load("B.png")),
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    margin: UiRect::top(px(-17)),
                                    ..default()
                                }
                            ),
                            (
                                ImageNode::new(asset_server.load("B.png")),
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    margin: UiRect::bottom(px(-17)),
                                    ..default()
                                }
                            ),
                        ]
                    ),
                    (
                        Node {
                            width: percent(33),
                            height: percent(100),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        children![(
                            ImageNode::new(asset_server.load("A.png")),
                            Node {
                                width: px(40),
                                height: px(40),
                                margin: UiRect::left(px(60)),
                                ..default()
                            }
                        )]
                    )
                ]
            )
        ],
    ));
}
