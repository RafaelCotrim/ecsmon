use bevy::{app::prelude::*, ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet}};

use crate::plugins::{flow_field_pathfinding::plugin::FlowFieldSystemSet, kinematics::plugin::KinematicsSet};

use super::{components::*, configuration::*, system::*};

#[derive(Default)]
pub struct SocialForcesPlugin{
    pub configuration: SocialForcesModelConfiguration,
}

impl Plugin for SocialForcesPlugin {
    fn build(&self, app: &mut App) {

        app.insert_resource(self.configuration);

        app.configure_sets(Update, SocialForcesSystemSet::ComputeForces.before(KinematicsSet::ApplyVelocity));
        app.configure_sets(
            Update, 
            SocialForcesSystemSet::ApplyForces
            .after(SocialForcesSystemSet::ComputeForces)
            .before(KinematicsSet::ApplyVelocity)
        );

        app
        .add_systems(First, add_force_to_agents::<MotivationForce>)
        .add_systems(First, add_force_to_agents::<ObstacleForce>)
        .add_systems(First, add_force_to_agents::<RepulsiveForce>)
        .add_systems(Update, obstacle_force.in_set(SocialForcesSystemSet::ComputeForces))
        .add_systems(Update, compute_motivation_force.in_set(SocialForcesSystemSet::ComputeForces).after(FlowFieldSystemSet::ComputeFields))
        //.add_systems(Update, compute_repulsive_forces.in_set(SocialForcesSystemSet::ComputeForces))
        .add_systems(Update, apply_social_foces.in_set(SocialForcesSystemSet::ApplyForces))
        .add_systems(Update, agent_max_speed.in_set(SocialForcesSystemSet::ApplyForces));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SocialForcesSystemSet {
    ComputeForces,
    ApplyForces,
}