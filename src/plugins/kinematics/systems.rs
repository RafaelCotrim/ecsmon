use bevy::prelude::*;

use crate::{components::prelude::*, resources::configuration::SimulationConfiguration};

pub fn apply_velocity(simulation_configuration: Res<SimulationConfiguration>, mut query: Query<(&mut Position, &Speed)>){
    query.iter_mut().for_each(|(mut pos, velocity)| {
        *pos += (velocity.value() * simulation_configuration.simulation_time_step).into()
    });
}