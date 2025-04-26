use bevy::prelude::*;

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Objective;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component, Copy, Clone)]
pub struct Destination(pub Entity);