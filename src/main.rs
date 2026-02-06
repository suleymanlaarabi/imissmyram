use avian3d::PhysicsPlugins;
use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*};

use crate::plugins::{
    enemy::EnemyPlugin, gun::GunPlugin, health::HealthPlugin, player::PlayerPlugin,
    ui::GameUiPlugin,
};

mod plugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            EnemyPlugin,
            GunPlugin,
            HealthPlugin,
            GameUiPlugin,
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(),
            FpsOverlayPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // commands.spawn((
    //     SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("Chara.glb"))),
    //     Transform::from_scale(Vec3::splat(3.)).with_translation(Vec3::ZERO),
    // ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 40., 0.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
