use bevy::ecs::system::Resource;

#[derive(Resource, Clone, Copy)]
pub struct SocialForcesModelConfiguration {
    // Agent data
    pub agent_desired_speed: f32,   // m/s
    pub agent_mass: f32,            // Kg
    //pub agent_radius: f32 ,       // m

    // Constants
    pub a: f32,
    pub b: f32,
    pub k: f32,
    pub kappa: f32,

    pub forces: ForceConfiguration,
}

impl Default for SocialForcesModelConfiguration {
    fn default() -> Self {
        Self {
            agent_desired_speed: 0.8,
            agent_mass: 80.,
            //agent_radius: 0.3,
            a: 2000.,   // N
            b: 0.08,    // m
            k: 120000., // kg/s²
            kappa: 240000.,
            forces: ForceConfiguration::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ForceConfiguration {
    pub motivation_force: MotivationForceComputationStrategy,
    pub repulsion_force: RepulsionForceComputationStrategy,
    pub obstacle_force: ObstacleForceComputationStrategy,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum MotivationForceComputationStrategy {
    None,
    Direct,
    #[default]
    FlowFieldPathFinding,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum RepulsionForceComputationStrategy {
    None,
    #[default]
    Direct,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum ObstacleForceComputationStrategy {
    None,
    #[default]
    Direct,
}
