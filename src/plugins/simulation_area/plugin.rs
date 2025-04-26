use bevy::{app::prelude::*, ecs::schedule::IntoSystemConfigs, math::Rect};

use crate::plugins::kinematics::plugin::KinematicsSet;

use super::{resources::*, systems::*};

pub struct SimulationAreaPlugin{
    pub simulation_area: Rect
}

impl Plugin for SimulationAreaPlugin {
    fn build(&self, app: &mut App) {

        app.insert_resource(SimulationArea(self.simulation_area));

        app.add_systems(Update, clamp_agent_position.after(KinematicsSet::ApplyVelocity))
        .add_systems(First, remove_out_of_bounds_agents_on_creation)
        ;
    }
}