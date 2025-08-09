use bevy::app::{Plugin, PreUpdate, Update};

use crate::plugins::spawner::systems::*;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .add_systems(PreUpdate, add_mesh_to_obstacles)
        .add_systems(Update, spawner);
    }
}
