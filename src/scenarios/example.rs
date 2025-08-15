// use bevy::{app::{prelude::*, PluginGroupBuilder}, math::{Rect, Vec2}};

// use crate::plugins::{default::plugin::ECSMosDefaultPlugins, kinematics::plugin::KinematicsPlugin, simple_objective::plugin::SimpleObjective, simulation_area::plugin::SimulationAreaPlugin, social_foces_model::{configuration::SocialForcesModelConfiguration, plugin::SocialForcesPlugin}};

// pub struct ExampleScenario;

// impl PluginGroup for ExampleScenario {
//     fn build(self) -> PluginGroupBuilder {
//         PluginGroupBuilder::start::<Self>()

//         .add_group(ECSMosDefaultPlugins)
//         .add(KinematicsPlugin)
//         .add(SimpleObjective)
//         .add((SimulationAreaPlugin {
//             simulation_area: Rect::from_center_size(Vec2::ZERO, Vec2::new(120., 60.)),
//         },))
//         .add(SocialForcesPlugin {
//             configuration: SocialForcesModelConfiguration {
//                 forces: ForceConfiguration {
//                     motivation_force: MotivationForceComputationStrategy::FlowFieldPathFinding,
//                     obstacle_force: ObstacleForceComputationStrategy::Direct,
//                     repulsion_force: RepulsionForceComputationStrategy::Direct,
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .add_plugins((FlowFieldPathfindingPlugin {
//             cell_size: 2. * 0.3,
//             ..Default::default()
//         },))
//         .add_plugins(TrackingPlugin::default())
//         .add_plugins(StartTimePluging)
//         // .add_plugins(AutoEndSimulationPlugin)
//         .add_plugins(SpawnerPlugin)
//         .add_systems(Startup, setup)
//     }
// }