use bevy::prelude::*;

use crate::{components::prelude::*, resources::configuration::SimulationConfiguration};

pub fn apply_velocity(
    simulation_configuration: Res<SimulationConfiguration>,
    mut query: Query<(&mut Position, &Speed)>,
) {
    query.iter_mut().for_each(|(mut pos, velocity)| {
        *pos += (velocity.value() * simulation_configuration.simulation_time_step).into()
    });
}

// #######
// Testing
// #######

#[test]
fn check_general_movement() {

    // Setup

    let mut app = App::new();

    app.insert_resource(SimulationConfiguration {
        simulation_time_step: 1.,
    });

    app.add_systems(Update, apply_velocity);

    let world = app.world_mut();


    let horizontal_agent =
        world.spawn((Speed::new(Vec2::new(1., 0.)), Position::from(Vec2::ZERO)))
        .id();

    let vertical_agent =
        world.spawn((Speed::new(Vec2::new(0., 1.)), Position::from(Vec2::ZERO)))
        .id();

    let diagonal_agent =
        world.spawn((Speed::new(Vec2::new(1., 1.)), Position::from(Vec2::ZERO)))
        .id();

    // Act
    
    app.update();

    // Assert

    assert!(app.world().get::<Position>(horizontal_agent).unwrap().eq(&Position::from(Vec2::new(1., 0.))));
    assert!(app.world().get::<Position>(vertical_agent).unwrap().eq(&Position::from(Vec2::new(0., 1.))));
    assert!(app.world().get::<Position>(diagonal_agent).unwrap().eq(&Position::from(Vec2::new(1., 1.))));
}


#[test]
fn check_simulation_speed() {

    let time_step = 3.5;
    let starting_speed_vec = Vec2::new(1.5324, -367.);
    // Setup

    let mut app = App::new();

    app.insert_resource(SimulationConfiguration {
        simulation_time_step: time_step,
    });

    app.add_systems(Update, apply_velocity);

    let world = app.world_mut();

    let agent =
        world.spawn((Speed::new(starting_speed_vec), Position::from(Vec2::ZERO)))
        .id();

    // Act

    app.update();

    // Assert

    assert!(app.world().get::<Position>(agent).unwrap().eq(&Position::from(starting_speed_vec * time_step)));
}