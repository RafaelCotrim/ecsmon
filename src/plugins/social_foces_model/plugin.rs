use bevy::{
    app::prelude::*,
    ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet},
};

use crate::plugins::{
    flow_field_pathfinding::plugin::FlowFieldSystemSet, kinematics::plugin::KinematicsSet,
};

use super::{components::*, configuration::*, system::*};

#[derive(Default)]
pub struct SocialForcesPlugin {
    pub configuration: SocialForcesModelConfiguration,
}

impl Plugin for SocialForcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.configuration);

        app.configure_sets(
            Update,
            SocialForcesSystemSet::ComputeForces.before(KinematicsSet::ApplyVelocity),
        );
        app.configure_sets(
            Update,
            (
                SocialForcesSystemSet::ComputeForces,
                SocialForcesSystemSet::ApplyForces,
                SocialForcesSystemSet::ApplyConstraints,
                KinematicsSet::ApplyVelocity,
            )
                .chain(),
        );

        app.add_systems(PreUpdate, add_force_to_agents::<MotivationForce>)
            .add_systems(PreUpdate, add_force_to_agents::<ObstacleForce>)
            .add_systems(PreUpdate, add_force_to_agents::<RepulsiveForce>);

        match self.configuration.forces.motivation_force {
            MotivationForceComputationStrategy::None => (),
            MotivationForceComputationStrategy::Direct => {
                app.add_systems(
                    Update,
                    compute_motivation_force_via_absolute_direction
                        .in_set(SocialForcesSystemSet::ComputeForces),
                );
            }
            MotivationForceComputationStrategy::FlowFieldPathFinding => {
                app.add_systems(
                    Update,
                    compute_motivation_force_via_floor_field
                        .in_set(SocialForcesSystemSet::ComputeForces)
                        .after(FlowFieldSystemSet::ComputeFields),
                );
            }
        }

        match self.configuration.forces.repulsion_force {
            RepulsionForceComputationStrategy::None => (),
            RepulsionForceComputationStrategy::Direct => {
                app.add_systems(
                    Update,
                    compute_repulsive_forces.in_set(SocialForcesSystemSet::ComputeForces),
                );
            }
        }

        match self.configuration.forces.obstacle_force {
            ObstacleForceComputationStrategy::None => (),
            ObstacleForceComputationStrategy::Direct => {
                app.add_systems(
                    Update,
                    compute_obstacle_force.in_set(SocialForcesSystemSet::ComputeForces),
                );
            }
        }

        app.add_systems(
            Update,
            apply_social_foces.in_set(SocialForcesSystemSet::ApplyForces),
        )
        .add_systems(
            Update,
            agent_max_speed.in_set(SocialForcesSystemSet::ApplyConstraints),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SocialForcesSystemSet {
    ComputeForces,
    ApplyForces,
    ApplyConstraints,
}
