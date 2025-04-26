use bevy::{math::VectorSpace, prelude::*};


use crate::components::{physics::{Position, Shape}, prelude::Agent};

use super::resources::SimulationArea;


pub fn clamp_agent_position(
    simulation_area: Res<SimulationArea>, 
    mut agents: Query<(&mut Position, &Shape), With<Agent>>) {

    for (mut position, shape) in &mut agents {
        let agent_radius = shape.get_rectangle_with_center(Vec2::ZERO).size() / 2.;
        
        let new_pos = position.value().clamp(simulation_area.0.min + agent_radius, simulation_area.0.max - agent_radius);
        position.set_value(new_pos);
    }
}

pub fn remove_out_of_bounds_agents_on_creation(mut commands: Commands, simulation_area: Res<SimulationArea>, agents: Query<(Entity, &Position), Added<Agent>>){
    for (entity, position ) in &agents {

        if !simulation_area.0.contains(position.value()){
            commands.entity(entity).despawn();
        }
    }
}