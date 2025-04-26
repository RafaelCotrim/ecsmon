mod components;
mod plugins;
mod resources;

use std::ops::AddAssign;

use bevy::{color::palettes::tailwind::*, prelude::*};
use components::prelude::*;
use plugins::{default::plugin::ECSMosDefaultPlugins, kinematics::plugin::KinematicsPlugin};
use resources::configuration::*;

fn main() {
    let mut app = App::new();
    app
    .add_plugins(ECSMosDefaultPlugins)
    .add_plugins(KinematicsPlugin)
    .add_systems(Startup, setup)

    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){

    commands.insert_resource(SimulationConfiguration::default());
    commands.insert_resource(SocialForcesModelConfiguration::default());

    let objective = commands.spawn((
        Objective,
        Shape::Circle(1.),
        Mesh2d(meshes.add(Circle { radius: 1. * 10. })),
        MeshMaterial2d(materials.add(Color::from(RED_500))),
        Position::from(Vec2::new(10., 0.0)),
        Transform::default()
    )).id();


    commands.spawn((
        Agent,
        Shape::Circle(0.3),
        Destination( objective ),
        Speed::new(Vec2::new(0.8, 0.)),
        Mesh2d(meshes.add(Circle { radius: 0.3 * 10.})),
        MeshMaterial2d(materials.add(Color::from(BLUE_500))),
        Position::from(Vec2::new(0., 0.)),
        Transform::default()
    ));

}