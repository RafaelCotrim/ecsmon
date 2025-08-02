use bevy::{ecs::resource::Resource, math::Rect};

#[derive(Resource)]
pub struct SimulationArea(pub Rect);