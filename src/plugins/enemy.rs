use std::process::exit;

use avian3d::prelude::{Collider, CollisionEventsEnabled, CollisionStart, Sensor};
use bevy::prelude::*;

use crate::{
    Cpu, DespawnOnFinish,
    plugins::{gun::Bullet, health::Health, player::Player, ui::CpuHealthBar},
};

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct EnemyMovement(pub Vec<Vec3>);

#[derive(Resource)]
pub struct EnemyDieAudio(pub Handle<AudioSource>);

#[derive(Component)]
pub struct EnemySpawner {
    pub position: Vec3,
    pub timer: Timer,
    pub path: Vec<Vec3>,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_enemy_spawner, spawn_enemy_die_audio))
            .add_systems(Update, (move_enemy, handle_enemy_hit, handle_enemy_spawn));
    }
}

fn spawn_enemy_die_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(EnemyDieAudio(asset_server.load("die.ogg")));
}

const ENEMY_SPEED: f32 = 2.;

const BASE_BULLET_DAMAGE: f32 = 10.;
const DAMAGE_PER_LEVEL: f32 = 5.;

fn handle_enemy_hit(
    mut collision_start_event_reader: MessageReader<CollisionStart>,
    bullet_query: Query<Entity, With<Bullet>>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
    mut commands: Commands,
    mut player: Single<&mut Player>,
    audio: Res<EnemyDieAudio>,
) {
    let damage = BASE_BULLET_DAMAGE + player.damage_level as f32 * DAMAGE_PER_LEVEL;

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
            health.0 -= damage;
            if health.0 <= 0. {
                player.coins += 5;
                commands.spawn((AudioPlayer::new(audio.0.clone()), DespawnOnFinish));
            }
        }
    }
}

fn move_enemy(
    query: Query<(Entity, &mut Transform, &mut EnemyMovement), With<Enemy>>,
    mut commands: Commands,
    mut cpu: Single<&mut Health, With<Cpu>>,
    time: Res<Time>,
    mut health_bar: Single<&mut Node, With<CpuHealthBar>>,
) {
    for (entity, mut transform, mut movement) in query {
        if let Some(target) = movement.last().cloned() {
            let distance = transform.translation.distance(target);

            if distance < 0.1 {
                movement.pop();
            }

            let direction = (target - transform.translation).normalize();

            transform.translation += direction * ENEMY_SPEED * time.delta_secs()
        } else {
            let direction = (Vec3::new(0., 0.5, 0.) - transform.translation).normalize();

            transform.translation += direction * ENEMY_SPEED * time.delta_secs();

            if transform.translation.distance(Vec3::new(0., 0.5, 0.)) <= 0.2 {
                commands.entity(entity).despawn();
                cpu.0 -= 10.;
                health_bar.width = percent(cpu.0);
                if cpu.0 <= 0. {
                    exit(0);
                }
            }
        }
    }
}

fn spawn_enemy_spawner(mut commands: Commands) {
    commands.spawn(EnemySpawner {
        timer: Timer::from_seconds(3., TimerMode::Repeating),
        position: Vec3::new(0., 0.5, -14.),
        path: vec![
            Vec3::new(-1., 0.5, -2.),
            Vec3::new(1.5, 0.5, -5.),
            Vec3::new(-2., 0.5, -9.),
        ],
    });

    commands.spawn(EnemySpawner {
        timer: Timer::from_seconds(12., TimerMode::Repeating),
        position: Vec3::new(0., 0.5, 14.),
        path: vec![
            Vec3::new(1., 0.5, 2.),
            Vec3::new(-1.5, 0.5, 5.),
            Vec3::new(2., 0.5, 9.),
        ],
    });

    commands.spawn(EnemySpawner {
        timer: Timer::from_seconds(17., TimerMode::Repeating),
        position: Vec3::new(14., 0.5, 0.),
        path: vec![
            Vec3::new(2., 0.5, 1.),
            Vec3::new(6., 0.5, -1.5),
            Vec3::new(10., 0.5, 1.),
        ],
    });

    commands.spawn(EnemySpawner {
        timer: Timer::from_seconds(25., TimerMode::Repeating),
        position: Vec3::new(-14., 0.5, 0.),
        path: vec![
            Vec3::new(-2., 0.5, -1.),
            Vec3::new(-6., 0.5, 1.5),
            Vec3::new(-10., 0.5, -1.),
        ],
    });
}

fn handle_enemy_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut EnemySpawner>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for mut spawner in &mut query {
        spawner.timer.tick(delta);

        if spawner.timer.just_finished() {
            commands.spawn((
                Enemy,
                Health(100.),
                EnemyMovement(spawner.path.clone()),
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::srgb_u8(255, 50, 50))),
                Transform::from_xyz(spawner.position.x, spawner.position.y, spawner.position.z),
                Sensor::default(),
                Collider::cuboid(1.0, 1.0, 1.0),
                CollisionEventsEnabled,
            ));
            spawner.timer.reset();
        }
    }
}
