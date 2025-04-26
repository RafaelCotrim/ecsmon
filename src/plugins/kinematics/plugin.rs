use bevy::{app::{Plugin, Update}, ecs::schedule::{IntoSystemConfigs, SystemSet}};

use super::systems::*;
pub struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, apply_velocity.in_set(KinematicsSet::ApplyVelocity));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum KinematicsSet {
    ApplyVelocity
}