use bevy::prelude::*;

use crate::components::prelude::*;

pub fn check_if_agent_araived_at_destination(
    mut commands: Commands,
    agents: Query<(Entity, &Position, &Destination)>,
    destinations: Query<(&Position, &Shape), With<Objective>>,
) {
    for (agent, agent_pos, agent_destination) in &agents {
        let agent_position = agent_pos.value();

        let (destination_pos, destination_shape) = match destinations.get(agent_destination.0) {
            Ok(e) => e,
            Err(_) => continue,
        };

        match destination_shape {
            Shape::Circle(radius) => {
                let distance = (destination_pos.value().length() - agent_position).length() - radius;
                if distance <= 0. {
                    commands.entity(agent).despawn();
                }
            }
            _ => todo!(),
        }
    }
}

// #######
// Testing
// #######

#[test]
fn test_contact() {
    // Setup

    let mut app = App::new();

    app.add_systems(Update, check_if_agent_araived_at_destination);

    let world = app.world_mut();

    let objective = world
        .spawn((
            Objective,
            Shape::Circle(2.),
            Position::from(Vec2::new(0., 0.)),
        ))
        .id();

    let inside_agent = world
        .spawn((
            Agent,
            Position::from(Vec2::ZERO),
            //Shape::Circle(2.),
            Destination(objective),
        ))
        .id();

    let touching_agent = world
        .spawn((
            Agent,
            Position::from(Vec2::new(2., 0.)),
            //Shape::Circle(2.),
            Destination(objective),
        ))
        .id();

    let outside_agent = world
        .spawn((
            Agent,
            Position::from(Vec2::new(3., 0.)),
            Shape::Circle(2.),
            Destination(objective),
        ))
        .id();

    // Act

    app.update();

    // Assert

    assert!(app.world().get::<Position>(inside_agent).is_none());
    assert!(app.world().get::<Position>(touching_agent).is_none());
    assert!(app.world().get::<Position>(outside_agent).is_some());
}

#[test]
fn test_diferent_destination() {
    // Setup

    let mut app = App::new();

    app.add_systems(Update, check_if_agent_araived_at_destination);

    let world = app.world_mut();

    let objective1 = world
        .spawn((
            Objective,
            Shape::Circle(2.),
            Position::from(Vec2::new(0., 0.)),
        ))
        .id();

    let objective2 = world
        .spawn((
            Objective,
            Shape::Circle(2.),
            Position::from(Vec2::new(0., 0.)),
        ))
        .id();

    let objective3 = world
        .spawn((
            Objective,
            Shape::Circle(2.),
            Position::from(Vec2::new(10., 10.)),
        ))
        .id();

    let agent_1 = world
        .spawn((Agent, Position::from(Vec2::ZERO), Destination(objective1)))
        .id();

    let agent_2 = world
        .spawn((
            Agent,
            Position::from(Vec2::new(10., 0.)),
            Destination(objective2),
        ))
        .id();

    let agent_3 = world
        .spawn((
            Agent,
            Position::from(Vec2::new(0., 0.)),
            Destination(objective3),
        ))
        .id();

    // Act

    app.update();

    // Assert

    assert!(app.world().get::<Position>(agent_1).is_none());
    assert!(app.world().get::<Position>(agent_2).is_some());
    assert!(app.world().get::<Position>(agent_3).is_some());
}
