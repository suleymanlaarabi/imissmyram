use avian3d::prelude::{Collider, CollisionEventsEnabled, CollisionStart, RigidBody, Sensor};
use bevy::prelude::*;

use crate::plugins::{gun::Bullet, health::Health, player::Player};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct EnemyMovement(pub Vec<Vec3>);

#[derive(Resource)]
pub struct EnemyDieAudio(pub Handle<AudioSource>);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_enemy, spawn_enemy_die_audio))
            .add_systems(Update, (move_enemy, handle_enemy_hit));
    }
}
fn spawn_enemy_die_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(EnemyDieAudio(asset_server.load("die.ogg")));
}

const ENEMY_SPEED: f32 = 2.;

fn handle_enemy_hit(
    mut collision_start_event_reader: MessageReader<CollisionStart>,
    bullet_query: Query<Entity, With<Bullet>>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
    mut commands: Commands,
    mut player: Single<&mut Player>,
    audio: Res<EnemyDieAudio>,
) {
    for CollisionStart {
        collider1,
        collider2,
        body1: _,
        body2: _,
    } in collision_start_event_reader.read()
    {
        let target = if bullet_query.contains(collider1.entity()) {
            commands.entity(collider1.entity()).despawn();
            collider2
        } else if bullet_query.contains(collider2.entity()) {
            commands.entity(collider2.entity()).despawn();
            collider1
        } else {
            continue;
        };

        if let Ok(mut health) = enemy_query.get_mut(target.entity()) {
            health.0 -= 10.;
            if health.0 <= 0. {
                player.coins += 5;
                commands.spawn(AudioPlayer::new(audio.0.clone()));
            }
        }
    }
}

fn move_enemy(query: Query<(&mut Transform, &mut EnemyMovement), With<Enemy>>, time: Res<Time>) {
    for (mut transform, mut movement) in query {
        if let Some(target) = movement.last().cloned() {
            let distance = transform.translation.distance(target);

            if distance < 0.1 {
                movement.pop();
            }

            let direction = (target - transform.translation).normalize();

            transform.translation += direction * ENEMY_SPEED * time.delta_secs()
        } else {
            let direction = (Vec3::new(0., 0.5, 0.) - transform.translation).normalize();

            transform.translation += direction * ENEMY_SPEED * time.delta_secs()
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Enemy,
        Health(100.),
        EnemyMovement([Vec3::new(1., 0.5, 4.)].to_vec()),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 50, 50))),
        Transform::from_xyz(3.0, 0.5, 4.0),
        Sensor::default(),
        Collider::cuboid(1.0, 1.0, 1.0),
        CollisionEventsEnabled,
    ));
}
