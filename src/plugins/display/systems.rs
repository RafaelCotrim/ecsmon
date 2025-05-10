use bevy::prelude::*;

use crate::components::prelude::*;

use super::resources::DisplayConfiguration;

pub fn add_transform_for_positioned_components(
    display_configuration: Res<DisplayConfiguration>,
    mut commands: Commands, 
    query: Query<(Entity, &Position), (With<Position>, Without<Transform>)>
){
    for (entity, position) in query.iter() {

        let mut t = Transform::default();

        t.translation = position.value().extend(0.) * display_configuration.pixels_per_meter;

        commands.entity(entity).insert(t);
    }
}

pub fn add_mesh_for_shaped_components(
    config: Res<DisplayConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands, 
    query: Query<(Entity, &Shape), Without<Mesh2d>>
){
    for (entity, shape) in query.iter() {

        let mesh = match shape {
            Shape::Circle(radius) => meshes.add(Circle { radius:radius * config.pixels_per_meter}),
            Shape::Polygon(_) => todo!(),
        };

        commands.entity(entity).insert(Mesh2d(mesh));
    }
}

pub fn position_to_pixel(display_configuration: Res<DisplayConfiguration>, mut query: Query<(&mut Transform, &Position), Changed<Position>>){
    for (mut transform, position) in query.iter_mut() {
        transform.translation = position.value().extend(0.) * display_configuration.pixels_per_meter;
    }
}