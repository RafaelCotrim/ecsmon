use bevy::{math::vec2, prelude::*};

use crate::components::prelude::*;

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

pub fn obstacle_force(
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

// Helpers

fn signed_distance_and_normal_to_sahpe(shape: &Shape, shape_position: Vec2, point: Vec2) -> (Vec2, f32) {
    match shape {
        Shape::Circle(radius) => ( point - shape_position, (point - shape_position).length() - radius),
        Shape::Polygon(polygon_points) => {
        
            let point = point - shape_position;

            let num_points = polygon_points.len();
            let mut min_dist: f32 = f32::INFINITY;
            let mut segment = (Vec2::ZERO, Vec2::ZERO);
            let mut t_of_min = 0.;

            for i in 0..num_points{

                let a = polygon_points[i];
                let b = polygon_points[(i + 1) % num_points];

                let t = (point - a).dot(b - a) / (b - a).dot( b - a);

                let closes_point = a.lerp(b, t.clamp(0., 1.));
                
                let distance = (point - closes_point).length();

                if distance < min_dist{
                    min_dist = distance;
                    segment = (a, b);
                    t_of_min = t;
                }
            }

            let ab = segment.1 - segment.0;
            // Vec2::new(ab.y, -ab.x)
            let normal = match t_of_min {
                x if x < 0. => point - segment.0,
                x if x > 1. => point - segment.1,
                _ => Vec2::new(ab.y, -ab.x)
            };

            return (normal, min_dist);
        }
    }
}