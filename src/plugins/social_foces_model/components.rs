use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Default)]
pub struct MotivationForce(pub Vec2);

#[derive(Component, Default)]
pub struct ObstacleForce(pub Vec2);

#[derive(Component, Default)]
pub struct RepulsiveForce(pub Vec2);