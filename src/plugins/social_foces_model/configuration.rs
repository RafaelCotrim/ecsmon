use bevy::ecs::system::Resource;

#[derive(Resource, Clone, Copy)]
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