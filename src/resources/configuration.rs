use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct SimulationConfiguration{
    /// Number of seconds the simulation is advance by for each iteration (s)
    pub simulation_time_step: f32,
}

impl Default for SimulationConfiguration {
    fn default() -> Self {
        Self { simulation_time_step: 0.2 }
    }
}