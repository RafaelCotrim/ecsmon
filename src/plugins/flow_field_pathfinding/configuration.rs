use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct GridCellSize{
    pub rows: usize,
    pub columns: usize,
}

#[derive(Resource, Clone, Copy)]
pub struct FlowFieldConstants{
    pub influence_radius_multiplier: f32,
    pub kernel_radius_overflow: f32,
}

impl Default for FlowFieldConstants {
    fn default() -> Self {
        Self { influence_radius_multiplier: 100., kernel_radius_overflow:10. }
    }
}