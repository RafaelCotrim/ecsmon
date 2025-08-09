use bevy::{color::palettes::tailwind::{BLUE_500, YELLOW_100}, diagnostic::FrameCount, prelude::*};
use rand::Rng;

use crate::{
    components::{
        physics::{Position, Shape, Speed},
        prelude::{Agent, Destination},
    },
    plugins::{display::resources::DisplayConfiguration, spawner::components::*}, resources::configuration::SimulationConfiguration,
};

pub fn add_mesh_to_obstacles(
    mut commands: Commands,
    spawners: Query<(Entity, &SpawnerArea), With<Spawner>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<DisplayConfiguration>
) {
    for (entity, area) in spawners.iter() {
        let area = area.0 * config.pixels_per_meter;

        let material = materials.add(Color::from(YELLOW_100));
        let mesh = meshes.add(Rectangle::new(area.x * 2., area.y * 2.));

        commands.entity(entity).insert((
            MeshMaterial2d(material),
            Mesh2d(mesh),
        ));
    }
}

pub fn spawner(
    mut commands: Commands,
    frames: Res<FrameCount>,
    config: Res<SimulationConfiguration>,
    mut spawners: Query<(&Position, &SpawnerArea, &mut SpawnerSchedule, &SpawnerDestination), With<Spawner>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let now = frames.0 as f32 * config.simulation_time_step;
    let mut rng = rand::rng();

    for (position, area, mut schedule, destination) in spawners.iter_mut() {

        if now < schedule.start_time || now > schedule.end_time {
            continue;
        }

        if now - schedule.last_spawn < schedule.interval{
            continue;
        }

        schedule.last_spawn = now;

        let size = area.0;

        let x = position.value().x + rng.random_range(-size.x..=size.x);
        let y = position.value().y + rng.random_range(-size.y..=size.y);

        commands.spawn((
            Agent,
            Position::from(Vec2::new(x, y)),
            Shape::Circle(0.3),
            Speed::new(Vec2::new(0.0, 0.)),
            MeshMaterial2d(materials.add(Color::from(BLUE_500))),
            Destination(destination.0),
        ));
    }
}
