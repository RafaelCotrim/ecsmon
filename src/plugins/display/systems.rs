use bevy::prelude::*;

use crate::{components::prelude::*, resources::configuration::DisplayConfiguration};

pub fn position_to_pixel(display_configuration: Res<DisplayConfiguration>, mut query: Query<(&mut Transform, &Position), Changed<Position>>){
    for (mut transform, position) in query.iter_mut() {
        transform.translation = position.value().extend(0.) * display_configuration.pixels_per_meter;
    }
}