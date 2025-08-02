mod components;
mod plugins;
mod resources;
mod utils;

use bevy::{color::palettes::tailwind::*, prelude::*};
use components::prelude::*;
use plugins::{
    default::plugin::ECSMosDefaultPlugins,
    flow_field_pathfinding::{components::Ordering, plugin::FlowFieldPathfindingPlugin},
    kinematics::plugin::KinematicsPlugin,
    movement_tracking::plugin::TrackingPlugin,
    simple_objective::plugin::SimpleObjective,
    simulation_area::plugin::SimulationAreaPlugin,
    social_foces_model::{
        configuration::{
            ForceConfiguration, MotivationForceComputationStrategy,
            ObstacleForceComputationStrategy, RepulsionForceComputationStrategy,
            SocialForcesModelConfiguration,
        },
        plugin::SocialForcesPlugin,
    }, start_time::plugin::StartTimePluging,
};
use resources::configuration::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(ECSMosDefaultPlugins)
        .add_plugins(KinematicsPlugin)
        .add_plugins(SimpleObjective)
        .add_plugins((SimulationAreaPlugin {
            simulation_area: Rect::from_center_size(Vec2::ZERO, Vec2::new(120., 60.)),
        },))
        .add_plugins(SocialForcesPlugin {
            configuration: SocialForcesModelConfiguration {
                forces: ForceConfiguration {
                    motivation_force: MotivationForceComputationStrategy::FlowFieldPathFinding,
                    obstacle_force: ObstacleForceComputationStrategy::Direct,
                    repulsion_force: RepulsionForceComputationStrategy::Direct,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins((FlowFieldPathfindingPlugin {
            cell_size: 2. * 0.3,
            ..Default::default()
        },))
        .add_plugins(TrackingPlugin::default())
        .add_plugins(StartTimePluging)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(SimulationConfiguration::default());

    let objective = commands
        .spawn((
            Objective,
            Ordering(0),
            Shape::Circle(2.),
            MeshMaterial2d(materials.add(Color::from(RED_500))),
            Position::from(Vec2::new(30., 0.0)),
        ))
        .id();

    commands.spawn((
        Obstacle,
        Shape::Circle(10.),
        MeshMaterial2d(materials.add(Color::from(GRAY_400))),
        Position::from(Vec2::new(15., 0.)),
    ));

    for x in 0..10 {
        for y in 0..10 {
            commands.spawn((
                Agent,
                Shape::Circle(0.3),
                Destination(objective),
                Speed::new(Vec2::new(0.0, 0.)),
                MeshMaterial2d(materials.add(Color::from(BLUE_500))),
                Position::from(Vec2::new(-x as f32, y as f32)),
            ));
        }
    }
}
