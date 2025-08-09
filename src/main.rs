mod components;
mod plugins;
mod resources;
mod utils;

use bevy::{color::palettes::{css::{BLACK, RED}, tailwind::*}, math::VectorSpace, prelude::*};
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, path::ShapePath, prelude::{tess::path::traits::PathBuilder, ShapeBuilder, ShapeBuilderBase}};
use components::prelude::*;
use plugins::{
    auto_end_simulation::plugin::AutoEndSimulationPlugin,
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
    },
    start_time::plugin::StartTimePluging,
};
use resources::configuration::*;

use crate::plugins::spawner::{
    components::{Spawner, SpawnerArea, SpawnerDestination, SpawnerSchedule},
    plugin::SpawnerPlugin,
};

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
        // .add_plugins(AutoEndSimulationPlugin)
        .add_plugins(SpawnerPlugin)
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
            Position::from(Vec2::new(50., 0.0)),
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

    // commands.spawn((
    //     Spawner,
    //     Position::from(Vec2::new(-50., 0.)),
    //     SpawnerArea(Vec2::new(5., 20.)),
    //     SpawnerSchedule{
    //         start_time: 10.,
    //         end_time: 2000000.,
    //         interval: 2.,
    //         last_spawn: 0.,
    //     },
    //     SpawnerDestination(objective)
    // ));

    let points = vec![
        Vec2::new(100., 148.),
        Vec2::new(302.286, 300.),
        Vec2::new(700., 300.),
        Vec2::new(900., 148.),
        Vec2::new(1000., 0.),
        Vec2::new(0., 0.),
        Vec2::new(0., 148.),
        Vec2::new(100., 148.)
    ];

    let mut points: Vec<Vec2> = points.iter().map(|p| p - Vec2::new(500., 0.)).collect();
    points.reverse();

    let points = points.iter().map(|p| p /10.).collect();
    commands.spawn((
        Obstacle,
        Shape::Polygon(points),
        Position::from(Vec2::new(0., 0.))
    ));

}
