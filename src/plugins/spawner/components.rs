use bevy::{ecs::{component::Component, entity::Entity}, math::Vec2};

#[derive(Component, Clone, Copy)]
pub struct Spawner;

#[derive(Component, Clone, Copy)]
pub struct SpawnerArea(pub Vec2);

#[derive(Component, Clone, Copy)]
pub struct SpawnerSchedule{
    pub interval: f32,
    pub last_spawn: f32,
    pub start_time: f32,
    pub end_time: f32,
}

#[derive(Component, Clone, Copy)]
pub struct SpawnerDestination(pub Entity);