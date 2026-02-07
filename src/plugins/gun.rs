use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::DespawnOnFinish;

#[derive(Message)]
pub struct GunShootEvent {
    pub target: Vec3,
    pub source: Vec3,
}

#[derive(Component)]
pub struct Bullet {
    pub target: Vec3,
}

#[derive(Resource)]
pub struct BulletHitAudio(pub Handle<AudioSource>);

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GunShootEvent>()
            .add_systems(Startup, (insert_bullet_model, spawn_bullet_hit_audio))
            .add_systems(Update, handle_bullet_move)
            .add_systems(Update, handle_shoot);
    }
}

fn spawn_bullet_hit_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(BulletHitAudio(asset_server.load("impact.ogg")));
}

const BULLET_SPEED: f32 = 8.;

#[derive(Resource)]
struct BulletModel {
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
}

fn insert_bullet_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let model = BulletModel {
        mesh: Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        material: MeshMaterial3d(materials.add(Color::srgb_u8(255, 222, 33))),
    };
    commands.insert_resource(model);
}

fn handle_bullet_move(
    mut query: Query<(Entity, &mut Transform, &Bullet)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, bullet) in &mut query {
        let direction = (bullet.target - transform.translation).normalize();

        let distance = transform.translation.distance(bullet.target);

        if distance < 0.2 {
            commands.entity(entity).despawn();
        }
        transform.translation += direction * BULLET_SPEED * time.delta_secs()
    }
}

fn handle_shoot(
    mut commands: Commands,
    mut shoots: MessageReader<GunShootEvent>,
    bullet_model: Res<BulletModel>,
    bullet_audio: Res<BulletHitAudio>,
) {
    for shoot in shoots.read() {
        commands.spawn((
            bullet_model.mesh.clone(),
            bullet_model.material.clone(),
            Transform::from_xyz(shoot.source.x, shoot.source.y, shoot.source.z),
            Bullet {
                target: shoot.target.clone(),
            },
            Collider::cuboid(0.5, 0.5, 0.5),
        ));
        commands.spawn((AudioPlayer::new(bullet_audio.0.clone()), DespawnOnFinish));
    }
}
