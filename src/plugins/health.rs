use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, kill_when_no_more_health);
    }
}

fn kill_when_no_more_health(query: Query<(Entity, &Health)>, mut commands: Commands) {
    for (entity, health) in &query {
        if health.0 <= 0. {
            commands.entity(entity).despawn();
        }
    }
}
