use bevy::{color::palettes::{css::WHITE, tailwind::{BLUE_500, RED_500}}, math::VectorSpace, prelude::*};
use bevy_pancam::PanCam;


use crate::plugins::simulation_area::resources::SimulationArea;

use super::{resources::DisplayConfiguration, systems::*};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DisplayConfiguration::default())
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, add_transform_for_positioned_components)
        .add_systems(PreUpdate, add_mesh_for_shaped_components)
        .add_systems(PostUpdate, position_to_pixel);
        
    }
}

fn setup(
    simulation_area: Res<SimulationArea>,
    display_configuration: Res<DisplayConfiguration>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    commands.spawn((Camera2d::default(), PanCam::default()));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(simulation_area.0.size() * display_configuration.pixels_per_meter))),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform{
            translation: Vec3::ZERO.with_z(-0.1),
            ..Default::default()
        }
    ));
}