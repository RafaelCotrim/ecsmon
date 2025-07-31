use bevy::{app::{App, Plugin, Update}, ecs::schedule::IntoSystemConfigs};

use crate::plugins::kinematics::plugin::KinematicsSet;

use super::systems::*;

pub struct SimpleObjective;

impl Plugin for SimpleObjective {
    fn build(&self, app: &mut App) {

        app.add_systems(Update, check_if_agent_araived_at_destination.after(KinematicsSet::ApplyVelocity));
    }
}
