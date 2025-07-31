use bevy::{math::vec2, prelude::*};

use crate::{components::prelude::*, plugins::flow_field_pathfinding::resources::{EntityMultiField, Grid2D}};

use super::{components::*, configuration::*};

// Setup

pub fn add_force_to_agents<T> (
    mut commands: Commands, 
    query: Query<Entity, (With<Agent>, Without<T>)>
) where T: Default + Component{
    for entity in query.iter() {
        commands.entity(entity).insert(T::default());
    }
}

// Update

pub fn compute_obstacle_force(
    config: Res<SocialForcesModelConfiguration>,
    mut agents: Query<(&mut ObstacleForce, &Position, &Shape), With<Agent>>,
    obstacles: Query<(&Position, &Shape), With<Obstacle>>,
) {

    for (mut force, _, _)in &mut agents{
        force.0 = vec2(0., 0.)
    }
    
    for (mut obstacle_force, agent_pos, shape) in &mut agents {
        for (obstacle_pos, obstacle_shape) in &obstacles {
            
            let (n, dist) = signed_distance_and_normal_to_sahpe(
                obstacle_shape, 
                obstacle_pos.value(), 
                agent_pos.value()
            );
        
            let effective_distance = dist - shape.get_rectangle_with_center(Vec2::ZERO).width() / 2.;

            let g = 0.;

            let n = n.normalize();
            let t = Vec2::new(-n.y, n.x);

            let repulsive_factor = config.a * (-effective_distance / config.b).exp();
            let contact_factor = config.k * g * effective_distance;

            let pushing_force = (repulsive_factor + contact_factor) * n;
            let sliding_force = config.kappa * g * effective_distance * t;

            let final_force = pushing_force + sliding_force;

            obstacle_force.0 += Vec2::new(final_force.x, final_force.y);
        }
    }
}

pub fn apply_social_foces(
    config: Res<SocialForcesModelConfiguration>,
    mut agents: Query<(&mut Speed, &ObstacleForce, &MotivationForce, &RepulsiveForce), With<Agent>>,
) {
    for (mut agent_speed, obstacle_force, motivation_force, repulsive_force) in &mut agents {
        let previous_speed = agent_speed.value().clone();

        agent_speed.set_value(previous_speed + motivation_force.0 + (obstacle_force.0 + repulsive_force.0) / config.agent_mass);

        *agent_speed += (obstacle_force.0 + (obstacle_force.0 + repulsive_force.0) / config.agent_mass).into();
    }
}

pub fn compute_motivation_force_via_floor_field(
    config: Res<SocialForcesModelConfiguration>,
    vector_multi_field: ResMut<EntityMultiField<Vec2>>, 
    mut agents: Query<(&mut MotivationForce, &Position, &Speed, &Destination), With<Agent>>
){
    
    for (mut motivation_force, position, agent_speed, &destination) in &mut agents {
        let pos = position.value();
        let vector_field = vector_multi_field.get(&destination.0).expect("Grid map not found in grid multi map");

        let cell = match vector_field.get_cell(&pos) {
            Some(v) => v,
            None => continue,
        };

        let base_vector = cell
        .adjacent_inclusive()
        .iter()
        .map(|c| (c, vector_field.get(c)))
        .filter_map(|(c, v)| 
            if let Some(value) = v {
            Some((c, value))
            } else {
                None
            }
        ).map(|(c, v)| v/(vector_field.get_coord(*c) - pos).length_squared())
        .fold(Vec2::ZERO, |acc, v| acc + v).normalize() * config.agent_desired_speed;

        if base_vector.is_nan(){
            continue;
        }
        
        let final_force = base_vector - agent_speed.value();

        motivation_force.0 = final_force;
    }
}

pub fn compute_motivation_force_via_absolute_direction(
    config: Res<SocialForcesModelConfiguration>,
    mut agents: Query<(&mut MotivationForce, &Position, &Speed, &Destination), With<Agent>>,
    objectives: Query<(&Position), With<Objective>>
){
    for (mut motivation_force, agent_position, agent_speed, destination) in agents.iter_mut() {
        if let Ok(objective_position) = objectives.get(destination.0){
            let base_vector = (objective_position.value() - agent_position.value()).normalize() * config.agent_desired_speed;

            if base_vector.is_nan() || base_vector.length() < f32::EPSILON{
                continue;
            }

            let final_force = base_vector - agent_speed.value();

            motivation_force.0 = final_force;
        }
    }
}

pub fn compute_repulsive_forces(
    config: Res<SocialForcesModelConfiguration>,
    mut agents: Query<(&mut RepulsiveForce, &Position, &Shape), With<Agent>>
) {
    
    for (mut force, _, _)in &mut agents{
        force.0 = vec2(0., 0.)
    }
    
    let mut combinations = agents.iter_combinations_mut();

    let g = 0.;

    while let Some([(mut force_1, position_1, shape_1), (mut force_2, position_2, shape_2)]) = combinations.fetch_next() {

        let combined_radius = match (shape_1, shape_2) {
            (Shape::Circle(r1), Shape::Circle(r2)) => r1 + r2,
            (_, _) => todo!()
        };

        let effective_distance = (position_2.value() - position_1.value()).length() - combined_radius;


        let n = (position_1.value() - position_2.value())
        .normalize();

        let t = Vec2::new(-n.y, n.x);

        let repulsive_factor = config.a * (-effective_distance / config.b).exp();
        let contact_factor = config.k * g * effective_distance;

        let pushing_force = (repulsive_factor + contact_factor) * n;
        let sliding_force = config.kappa * g * effective_distance * t;

        let final_force = pushing_force + sliding_force;

        force_1.0 += Vec2::new(final_force.x, final_force.y);
        force_2.0 += -Vec2::new(final_force.x, final_force.y);
    }
}

pub fn agent_max_speed(config: Res<SocialForcesModelConfiguration>, mut agents: Query<&mut Speed, With<Agent>>) {
    for mut speed in &mut agents {
        let mut new_speed = speed.value().clamp_length_max(config.agent_desired_speed);
        
        if new_speed.is_nan() {
            new_speed = Vec2::ZERO;
        }

        speed.set_value(new_speed);
    }
}