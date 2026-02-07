use bevy::prelude::*;

use crate::plugins::player::Player;

pub struct GameUiPlugin;

#[derive(Component)]
struct PlayerCoinText;

#[derive(Component)]
pub struct CpuHealthBar;

#[derive(Component)]
struct FireRateLevelText;

#[derive(Component)]
struct DamageLevelText;

#[derive(Component)]
struct SpeedLevelText;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(FixedUpdate, (update_player_coin_ui, update_upgrade_levels));
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

fn update_upgrade_levels(
    player: Query<&Player, Changed<Player>>,
    mut fire_rate_text: Single<&mut Text, With<FireRateLevelText>>,
    mut damage_text: Single<&mut Text, (With<DamageLevelText>, Without<FireRateLevelText>)>,
    mut speed_text: Single<
        &mut Text,
        (
            With<SpeedLevelText>,
            Without<FireRateLevelText>,
            Without<DamageLevelText>,
        ),
    >,
) {
    if let Ok(player) = player.single() {
        fire_rate_text.0 = format!("Fire Rate Lv{}", player.fire_rate_level);
        damage_text.0 = format!("Damage Lv{}", player.damage_level);
        speed_text.0 = format!("Speed Lv{}", player.speed_level);
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
            (
                Node {
                    width: px(250),
                    height: px(40),
                    border: UiRect::all(px(4)),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(50, 50, 50)),
                BorderColor::all(Color::srgb_u8(240, 240, 240)),
                children![(
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    BackgroundColor(Color::srgb_u8(0, 200, 0)),
                    CpuHealthBar
                )]
            ),
            (
                Text::new("coins: 0"),
                PlayerCoinText,
                TextFont {
                    font_size: 20.,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    width: px(150),
                    height: px(40),
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
                        children![
                            (
                                ImageNode::new(asset_server.load("Y.png")),
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    margin: UiRect::left(px(-60)),
                                    ..default()
                                }
                            ),
                            (
                                Text::new("Fire Rate Lv0"),
                                TextFont {
                                    font_size: 10.,
                                    ..default()
                                },
                                Node {
                                    margin: UiRect::left(px(-60)),
                                    ..default()
                                },
                                FireRateLevelText
                            )
                        ]
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
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(px(-17)),
                                    ..default()
                                },
                                children![
                                    (
                                        ImageNode::new(asset_server.load("X.png")),
                                        Node {
                                            width: px(40),
                                            height: px(40),
                                            ..default()
                                        }
                                    ),
                                    (
                                        Text::new("Damage Lv0"),
                                        TextFont {
                                            font_size: 10.,
                                            ..default()
                                        },
                                        DamageLevelText
                                    )
                                ]
                            ),
                            (
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::bottom(px(-17)),
                                    ..default()
                                },
                                children![
                                    (
                                        ImageNode::new(asset_server.load("B.png")),
                                        Node {
                                            width: px(40),
                                            height: px(40),
                                            ..default()
                                        }
                                    ),
                                    (
                                        Text::new("Speed Lv0"),
                                        TextFont {
                                            font_size: 10.,
                                            ..default()
                                        },
                                        SpeedLevelText
                                    )
                                ]
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
                        children![
                            (
                                ImageNode::new(asset_server.load("A.png")),
                                Node {
                                    width: px(40),
                                    height: px(40),
                                    margin: UiRect::left(px(60)),
                                    ..default()
                                }
                            ),
                            (
                                Text::new("Heal CPU"),
                                TextFont {
                                    font_size: 10.,
                                    ..default()
                                },
                                Node {
                                    margin: UiRect::left(px(60)),
                                    ..default()
                                }
                            )
                        ]
                    )
                ]
            )
        ],
    ));
}
