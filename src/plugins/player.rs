use bevy::prelude::*;

use crate::plugins::{gun::GunShootEvent, health::Health};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub coins: u32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, handle_player_move);
    }
}

const PLAYER_SPEED: f32 = 2.;

fn handle_player_move(
    time: Res<Time>,
    gamepads: Query<&Gamepad, Without<Player>>,
    mut player: Single<&mut Transform, With<Player>>,
    mut shoots: MessageWriter<GunShootEvent>,
) {
    for gamepad in &gamepads {
        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        if left_stick_x.abs() > 0.01 {
            player.translation.z -= left_stick_x * PLAYER_SPEED * time.delta_secs();
        }
        let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        if left_stick_y.abs() > 0.01 {
            player.translation.x -= left_stick_y * PLAYER_SPEED * time.delta_secs();
        }

        let x = gamepad.get(GamepadAxis::RightStickX).unwrap();
        let y = gamepad.get(GamepadAxis::RightStickY).unwrap();

        let dead_zone = 0.1;

        if x.abs() > dead_zone || y.abs() > dead_zone {
            let angle = (-x).atan2(y);
            player.rotation = Quat::from_rotation_y(angle);
        }

        if gamepad.just_pressed(GamepadButton::RightTrigger2) {
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

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player { coins: 0 },
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
