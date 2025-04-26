use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct DisplayConfiguration{
    /// Number of pixels for each meter on the screen (px/m)
    pub pixels_per_meter: f32,
}

impl Default for DisplayConfiguration {
    fn default() -> Self {
        Self { pixels_per_meter: 10. }
    }
}