use bevy::prelude::*;

use crate::{
    Cpu, DespawnOnFinish,
    plugins::{gun::GunShootEvent, health::Health, ui::CpuHealthBar},
};

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct PlayerLevelUpAudio(pub Handle<AudioSource>);

#[derive(Component)]
pub struct Player {
    pub coins: u32,
    pub fire_rate_level: u32,
    pub damage_level: u32,
    pub speed_level: u32,
}

pub const UPGRADE_COST: u32 = 10;

#[derive(Component)]
pub struct ShootCooldown(pub Timer);

const BASE_SHOOT_COOLDOWN: f32 = 0.5;
const COOLDOWN_REDUCTION_PER_LEVEL: f32 = 0.08;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_player_audio))
            .add_systems(Update, (handle_player_move, handle_upgrades));
    }
}

fn spawn_player_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(PlayerLevelUpAudio(asset_server.load("level.ogg")));
}

const PLAYER_BASE_SPEED: f32 = 2.;
const PLAYER_SPEED_PER_LEVEL: f32 = 0.5;

fn handle_player_move(
    time: Res<Time>,
    gamepads: Query<&Gamepad, Without<Player>>,
    mut player_query: Query<(&Player, &mut Transform, &mut ShootCooldown)>,
    mut shoots: MessageWriter<GunShootEvent>,
) {
    let Ok((player_data, mut player, mut cooldown)) = player_query.single_mut() else {
        return;
    };
    let speed = PLAYER_BASE_SPEED + player_data.speed_level as f32 * PLAYER_SPEED_PER_LEVEL;

    cooldown.0.tick(time.delta());

    for gamepad in &gamepads {
        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        if left_stick_x.abs() > 0.01 {
            player.translation.z -= left_stick_x * speed * time.delta_secs();
        }
        let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        if left_stick_y.abs() > 0.01 {
            player.translation.x -= left_stick_y * speed * time.delta_secs();
        }

        let x = gamepad.get(GamepadAxis::RightStickX).unwrap();
        let y = gamepad.get(GamepadAxis::RightStickY).unwrap();

        let dead_zone = 0.1;

        if x.abs() > dead_zone || y.abs() > dead_zone {
            let angle = (-x).atan2(y);
            player.rotation = Quat::from_rotation_y(angle);
        }

        if gamepad.just_pressed(GamepadButton::RightTrigger2) && cooldown.0.is_finished() {
            cooldown.0.reset();

            let distance = 20.0;

            let origin = player.translation;

            let rotation_offset = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
            let corrected_rotation = player.rotation * rotation_offset;

            let mut direction = corrected_rotation * Vec3::NEG_Z;
            direction.y = 0.0;
            let direction = direction.normalize();

            let target = origin + direction * distance;

            shoots.write(GunShootEvent {
                source: origin,
                target,
            });
        }
    }
}

fn handle_upgrades(
    gamepads: Query<&Gamepad, Without<Player>>,
    mut player_query: Query<(&mut Player, &mut ShootCooldown)>,
    mut cpu_health: Single<&mut Health, With<Cpu>>,
    mut health_bar: Single<&mut Node, With<CpuHealthBar>>,
    leve_audio: Res<PlayerLevelUpAudio>,
    mut commands: Commands,
) {
    let Ok((mut player, mut cooldown)) = player_query.single_mut() else {
        return;
    };

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::West) && player.coins >= UPGRADE_COST {
            player.coins -= UPGRADE_COST;
            player.fire_rate_level += 1;
            let new_cooldown = (BASE_SHOOT_COOLDOWN
                - player.fire_rate_level as f32 * COOLDOWN_REDUCTION_PER_LEVEL)
                .max(0.05);
            cooldown.0 = Timer::from_seconds(new_cooldown, TimerMode::Once);
            commands.spawn((AudioPlayer::new(leve_audio.0.clone()), DespawnOnFinish));
        }
        if gamepad.just_pressed(GamepadButton::North) && player.coins >= UPGRADE_COST {
            player.coins -= UPGRADE_COST;
            player.damage_level += 1;
            commands.spawn((AudioPlayer::new(leve_audio.0.clone()), DespawnOnFinish));
        }
        if gamepad.just_pressed(GamepadButton::East) && player.coins >= UPGRADE_COST {
            player.coins -= UPGRADE_COST;
            cpu_health.0 = (cpu_health.0 + 10.).min(100.);
            health_bar.width = percent(cpu_health.0);
            commands.spawn((AudioPlayer::new(leve_audio.0.clone()), DespawnOnFinish));
        }
        if gamepad.just_pressed(GamepadButton::South) && player.coins >= UPGRADE_COST {
            player.coins -= UPGRADE_COST;
            player.speed_level += 1;
            commands.spawn((AudioPlayer::new(leve_audio.0.clone()), DespawnOnFinish));
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player {
            coins: 0,
            fire_rate_level: 0,
            damage_level: 0,
            speed_level: 0,
        },
        ShootCooldown(Timer::from_seconds(BASE_SHOOT_COOLDOWN, TimerMode::Once)),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Health(100.),
        children![(
            Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 0.5))),
            MeshMaterial3d(materials.add(Color::srgb_u8(50, 255, 255))),
            Transform::from_xyz(-0.5, 0.0, 0.0),
        )],
    ));
}
