use bevy::prelude::*;
use bevy_pancam::PanCam;

use crate::resources::configuration::DisplayConfiguration;

use super::systems::*;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DisplayConfiguration::default())
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, position_to_pixel);
        
    }
}

fn setup(
    mut commands: Commands,
){
    commands.spawn((Camera2d::default(), PanCam::default()));
}