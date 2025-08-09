use bevy::{color::palettes::tailwind::GRAY_500, prelude::*};
use bevy_prototype_lyon::{path::ShapePath, prelude::{ShapeBuilder, ShapeBuilderBase}};

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
    query: Query<(Entity, &Shape), (Without<Mesh2d>, Without<bevy_prototype_lyon::entity::Shape>)>
){
    for (entity, shape) in query.iter() {

        if let Shape::Circle(radius) = shape {
            let mesh = meshes.add(Circle { radius:radius * config.pixels_per_meter});
            commands.entity(entity).insert(Mesh2d(mesh));
        }
        
        else if let Shape::Polygon(points) = shape {
            let mut up_path = ShapePath::new();

            for point in points.iter()  {
                up_path = up_path.line_to(*point * config.pixels_per_meter);
            }

            let path = up_path.close();

            let shape = ShapeBuilder::with(&path).fill(GRAY_500).build();

            commands.entity(entity).insert(shape);
        }
        
    }
}

pub fn position_to_pixel(display_configuration: Res<DisplayConfiguration>, mut query: Query<(&mut Transform, &Position), Changed<Position>>){
    for (mut transform, position) in query.iter_mut() {
        transform.translation = position.value().extend(0.) * display_configuration.pixels_per_meter;
    }
}