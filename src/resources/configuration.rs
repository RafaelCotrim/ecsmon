use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct SimulationConfiguration{
    /// Number of seconds the simulation is advance by for each iteration (s)
    pub simulation_time_step: f32,
}

impl Default for SimulationConfiguration {
    fn default() -> Self {
        Self { simulation_time_step: 1. }
    }
}

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

#[derive(Resource)]
pub struct SocialForcesModelConfiguration{

    // Agent data
    pub agent_desired_speed: f32, // m/s
    pub agent_mass: f32, // Kg 
    //pub agent_radius: f32 ,// m

    // Constants
    pub a: f32,
    pub b: f32,
    pub k: f32,
    pub kappa: f32,
}

impl Default for SocialForcesModelConfiguration {
    fn default() -> Self {
        Self { 
            agent_desired_speed: 0.8, 
            agent_mass: 80., 
            //agent_radius: 0.3, 
            a: 2000.,       // N
            b: 0.08,        // m
            k: 120000.,     // kg/s²
            kappa: 240000.  // kg/m*s
        }
    }
}