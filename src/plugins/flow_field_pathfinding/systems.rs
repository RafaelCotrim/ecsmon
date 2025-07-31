use std::{collections::VecDeque, f32::consts::PI};

use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{components::prelude::*, plugins::{display::resources::DisplayConfiguration, simulation_area::resources::SimulationArea}};

use super::{components::Ordering, models::*, resources::*};

// #############
// Setup Systems
// #############

pub fn add_field_map<T>(simulation_area: Res<SimulationArea>, grid_cell_size: Res<GridCellSize>,  mut commands: Commands) where T: Clone + Copy + Default + Send + Sync + 'static{
    commands.insert_resource(
        Field::new(
            grid_cell_size.columns, 
            grid_cell_size.rows, 
            simulation_area.0, 
            T::default()
        )
    );
}

pub fn add_entity_multi_field<T>(simulation_area: Res<SimulationArea>, grid_cell_size: Res<GridCellSize>,  mut commands: Commands) where T: Default + Clone + Send + Sync + 'static{
    commands.insert_resource(
        EntityMultiField::new(
            grid_cell_size.columns, 
            grid_cell_size.rows, 
            simulation_area.0, 
            T::default()
        )
    );
}

pub fn add_field_for_objectives<T> (
    mut fields: ResMut<EntityMultiField<T>>,
    objectives: Query<Entity, Added<Objective>>) where T: Clone + Default + Send + Sync + 'static {
    for e in objectives.into_iter() {
        fields.ensure(e);
    }
}

pub fn remove_field_for_objectives(
    mut grid_multi_map: ResMut<EntityMultiField<TargetStatus>>,
    mut removed: RemovedComponents<Objective>){
    for e in removed.read() {
        grid_multi_map.remove(&e);
    }
}

// ###################
// Computation Systems
// ###################

pub fn compute_objective_colision_map(
    mut grid_multi_map: ResMut<EntityMultiField<TargetStatus>>, 
    objectives: Query<(Entity, &Position, &Shape), (With<Objective>, Changed<Position>)>
){
    for (e, position, shape) in objectives.into_iter() {
        let map = grid_multi_map.get_mut(&e).expect("Grid map not found in grid multi map");

        map.reset(TargetStatus::default());

        let center = position.value();
        let rect = shape.get_rectangle_with_center(center);

        let region = map.get_cells_within(rect);
        
        let region = match region {
            Some(v) => v,
            None => continue,
        };

        for x in region.min.x..region.max.x {
            for y in region.min.y..region.max.y {

                
                let cell = IVec2::new(x, y);

                if let Some(value) = map.get(&cell){

                    if value == &TargetStatus::IsTarget {
                        continue;
                    }
                }
                else {
                    continue;
                }

                let cell_center = map.get_coord(cell);

                if !point_in_shape(shape, center, cell_center) {
                    let _ = map.set(cell, TargetStatus::NotTarget);
                    continue;
                }
                
                let _ = map.set(cell, TargetStatus::IsTarget);
            }
        }
    }
}

pub fn compute_colision_map<T, U>(
    mut map: ResMut<Field<T>>, 
    targets: Query<(&Position, &Shape), (With<U>, Changed<Position>)> 
) where T: CellStatus + 'static, U: Component{
    
    if !targets.is_empty(){
        map.reset(T::default());
    }
    
    for (position, shape) in &targets {
        let center = position.value();
        let rect = shape.get_rectangle_with_center(center);

        let region = map.get_cells_within(rect);
        
        let region = match region {
            Some(v) => v,
            None => continue,
        };

        for x in region.min.x..region.max.x {
            for y in region.min.y..region.max.y {

                
                let cell = IVec2::new(x, y);

                if let Some(value) = map.get(&cell){

                    if value == &T::get_non_default_value(){
                        continue;
                    }
                }
                else {
                    continue;
                }

                let cell_center = map.get_coord(cell);

                if !point_in_shape(shape, center, cell_center) {
                    let _ = map.set(cell, T::default());
                    continue;
                }
                
                let _ = map.set(cell, T::get_non_default_value());
            }
        }

        
    }
}

pub fn compute_proximity_map(
    mut proximity_multi_map: ResMut<EntityMultiField<TargetProximity>>, 
    obstacles_map: Res<Field<BlockedStatus>>, 
    target_multi_map: Res<EntityMultiField<TargetStatus>>,
    density_mutli_field: Res<EntityMultiField<AgentDensity>>){
    
    // if !density_map.is_changed() && !obstacles_map.is_changed() && !target_multi_map.is_changed(){
    //     return;
    // }

    for (target, proximity_map) in proximity_multi_map.iter_mut() {
        let target_map = target_multi_map.get(target).expect("Could not find related target colision map");
        //let density_map = density_mutli_field.get(target).expect("Could not find related density map");

        let mut open_list = VecDeque::new();

        proximity_map.reset(TargetProximity::NotComputed);
    
        for x in 0..proximity_map.get_columns() {
            for y in 0..proximity_map.get_rows() {
                let pos: IVec2 = IVec2::new(x as i32, y as i32);
    
                let proximity = match (obstacles_map.get(&pos), target_map.get(&pos)) {
                    (Some(BlockedStatus::Blocked), _) => TargetProximity::Obstacle,
                    (_, Some(TargetStatus::IsTarget)) => TargetProximity::Computed(0.),
                    (_, _) => TargetProximity::NotComputed
                };
    
                if let TargetProximity::Computed(_) = proximity{
                    open_list.push_back(pos);
                }
    
                proximity_map.set(pos, proximity).ok();
            }
        }
    
        while let Some(pivot_pos) = open_list.pop_front(){
    
            let any_invalid_coordinate = pivot_pos.adjacent()
                .iter()
                .any(|coord| proximity_map.get(coord) == Some(&TargetProximity::Obstacle));
    
            if any_invalid_coordinate {
                proximity_map.set(pivot_pos, TargetProximity::Buffer).unwrap();
            }
    
            let value_pivot_pos =  proximity_map.get(&pivot_pos).cloned();
    
            let value_pivot_pos = match value_pivot_pos {
                Some(TargetProximity::Computed(value)) => value,
                _ => continue,
            };
    
            for current_cell in pivot_pos.adjacent(){
                
                let value_at_cell = proximity_map.get(&current_cell);
    
                let delta = (current_cell - pivot_pos).as_vec2().length();
                // let density_delta = density_map.get(&current_cell);
                
                let densities = density_mutli_field.get_all(&current_cell);

                let same_target_density = densities.iter().find(|(k, _)| *k == target);

                let same_target_density = match same_target_density {
                    Some((_, density)) => density.map_or(0., |v| v.value()),
                    None => 0.,
                };

                let other_target_density : f32 = densities.
                iter()
                .filter(|(k, _)| *k != target)
                .filter_map(|(_, v)| v.to_owned())
                .map(|x| x.value()).sum();

                let base_repulsion = 250.;
                let ratio = 0.5;

                match value_at_cell {
                    Some(TargetProximity::NotComputed) => {
                        proximity_map.set(current_cell, TargetProximity::Computed(value_pivot_pos + delta + other_target_density * base_repulsion + same_target_density * base_repulsion * ratio)).unwrap();
                        open_list.push_back(current_cell);
                    },
                    Some(TargetProximity::Computed(value)) => {
                        let new_distance = value_pivot_pos + delta + other_target_density * base_repulsion + same_target_density * base_repulsion * ratio;
                        let distance = f32::min(*value, new_distance);
                        proximity_map.set(current_cell, TargetProximity::Computed(distance)).unwrap();
                    },
                    _ => {}
                };
            }
        }
    }


    
}

pub fn compute_vector_map(mut vector_multi_field: ResMut<EntityMultiField<Vec2>>, proximity_multi_map: ResMut<EntityMultiField<TargetProximity>>){
    
    if !proximity_multi_map.is_changed() {
        return;
    }

    for (entity, proximity_map) in proximity_multi_map.iter() {

        let vector_field = vector_multi_field.get_mut(entity).expect("Grid map pot found in grid multi map");

        for x_center in 0..proximity_map.get_columns(){
            for y_center in 0..proximity_map.get_rows(){
    
                let center: IVec2 = IVec2::new(x_center as i32, y_center as i32);
                let mut values = [Vec2::ZERO; 8];
                let mut i = 0;
    
                let invalid_coordinate = proximity_map.get(&center) == Some(&TargetProximity::Obstacle);
    
                if invalid_coordinate {
                    continue;
                }
    
                for current_pos in center.adjacent(){
    
                    values[i] = match proximity_map.get(&current_pos) {
                        Some(TargetProximity::Computed(value)) => 1./value * (current_pos - center).as_vec2(),
                        _ => Vec2::ZERO,
                    };
                
                    i += 1;
                }
    
                let final_vector = values
                .iter()
                .fold(Vec2::ZERO, |acc, &v| acc + v)
                .normalize();
    
                vector_field.set(center, final_vector).ok();
            }
        }
    }
}

pub fn compute_density_map(
    mut density_mutli_field: ResMut<EntityMultiField<AgentDensity>>, 
    agents: Query<(&Position, &Shape, &Destination), (With<Agent>, Changed<Position>)>){

    density_mutli_field.reset(0.0.into());

    let influence_radius = Shape::Circle(10.* 0.3);

    for (position, shape, destination) in agents.into_iter() {

        let density_map = match density_mutli_field.get_mut(&destination.0) {
            Some(value) => value,
            None => continue,
        };

        let agent_center = position.value();
        let rect = influence_radius.get_rectangle_with_center(agent_center);

        let region = density_map.get_cells_within(rect);
        
        let region = match region {
            Some(v) => v,
            None => continue,
        };

        for x in region.min.x..region.max.x {
            for y in region.min.y..region.max.y {

                let cell = IVec2::new(x, y);

                let cell_center = density_map.get_coord(cell);

                if !point_in_shape(&influence_radius, agent_center, cell_center) {
                    continue;
                }

                if let Some(value) = density_map.get(&cell){

                    let value = *value;

                    let distance: f32 = (cell_center - agent_center - 0.3).length();
                    let delta_density = density_kernel(distance, 10. * 0.3);
                    let new_density = value + delta_density.into();
                    density_map.set(cell, new_density).unwrap();
                }
            }
        }
    }
}

fn density_kernel(distance: f32,  radius: f32) -> f32{
    if distance >= radius {
        return 0.;
    }

    let volume = PI * radius.powi(4) / 6.;
    return (radius - distance).powi(2) / volume;
}

// ############
// Force Sytems
// ############



// #############
// Input Systems
// #############

pub fn handle_grid_state_inputs(grid_state: Res<State<ShowGridState>>, mut nex_grid_state: ResMut<NextState<ShowGridState>>, keys: Res<ButtonInput<KeyCode>>) {
    
    if keys.just_pressed(KeyCode::KeyG) {
        match grid_state.get() {
            ShowGridState::HideGrid => nex_grid_state.set(ShowGridState::ShowGrid),
            ShowGridState::ShowGrid => nex_grid_state.set(ShowGridState::HideGrid),
        }
    }
}

pub fn handle_overlay_inputs(state: Res<State<PathFindingOverlayState>>, mut next_state: ResMut<NextState<PathFindingOverlayState>>, keys: Res<ButtonInput<KeyCode>>) {
    
    let mut next = Option::None;

    if keys.just_pressed(KeyCode::KeyO) {
        next = Some(PathFindingOverlayState::ShowObstacles);
    }

    if keys.just_pressed(KeyCode::KeyP) {
        next = Some(PathFindingOverlayState::ShowProimity);
    }

    if keys.just_pressed(KeyCode::KeyT) {
        next = Some(PathFindingOverlayState::ShowTargets);
    }

    if keys.just_pressed(KeyCode::KeyV) {
        next = Some(PathFindingOverlayState::ShowVectorField);
    }

    if let Some(new_value) = next{
        if new_value == *state.get(){
            next_state.set(PathFindingOverlayState::ShowNone);
        } else {
            next_state.set(new_value);
        }
    }
}

pub fn handle_selection_inputs(mut state: ResMut<SelectedItem>, keys: Res<ButtonInput<KeyCode>>){
    if keys.just_pressed(KeyCode::Numpad0) {
        state.0 = 0;
    }

    if keys.just_pressed(KeyCode::Numpad1) {
        state.0 = 1;
    }
}

// ###############
// Drawing Systems
// ###############

pub fn draw_grid(config: Res<DisplayConfiguration>, mut gizmos: Gizmos, map: Res<Field<BlockedStatus>>){
    
    gizmos
        .grid_2d(
            map.get_area().center(),
            UVec2::new(map.get_columns() as u32, map.get_rows() as u32),
            map.get_cell_dimentions() * config.pixels_per_meter,
            LinearRgba::gray(0.05),
        )
        .outer_edges();


}

pub fn draw_obstacles(config: Res<DisplayConfiguration>, mut gizmos: Gizmos, map: Res<Field<BlockedStatus>>){

    let global_offset = Vec2::new(map.get_columns() as f32, map.get_rows() as f32) / 2.;
    let color = Color::from(RED_500);

    for x in 0..map.get_columns() {
        for y in 0..map.get_rows() {
            
            if let Some(BlockedStatus::Empty) = map.get(&IVec2::new(x as i32, y as i32)){
                continue;
            }

        
            let cell_top_left = map.get_area().center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.get_cell_dimentions();

            gizmos.line_2d(cell_top_left * config.pixels_per_meter, (cell_top_left + map.get_cell_dimentions()) * config.pixels_per_meter, color);
            gizmos.line_2d((cell_top_left + map.get_cell_dimentions().with_x(0.)) * config.pixels_per_meter, (cell_top_left + map.get_cell_dimentions().with_y(0.)) * config.pixels_per_meter, color);
        }
    }

}

pub fn draw_targets(config: Res<DisplayConfiguration>, mut gizmos: Gizmos, fields: Res<EntityMultiField<TargetStatus>>, selected: Res<SelectedItem>, objectives: Query<(Entity, &Ordering)>){

    let selected_entity = objectives.iter().filter(|x| x.1.0 == selected.0).last();

    let entity = match selected_entity {
        Some((e, _)) => e,
        None => {
            println!("No Entities with orderring {:?}", selected.0);
            return;
        },
    };

    let map = fields.get(&entity).unwrap();

    let global_offset = Vec2::new(map.get_columns() as f32, map.get_rows() as f32) / 2.;
    let color = Color::from(GREEN_500);

    for x in 0..map.get_columns() {
        for y in 0..map.get_rows() {
            
            if let Some(TargetStatus::NotTarget) = map.get(&IVec2::new(x as i32, y as i32)){
                continue;
            }

        
            let cell_top_left = map.get_area().center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.get_cell_dimentions();

            gizmos.line_2d(cell_top_left * config.pixels_per_meter, (cell_top_left + map.get_cell_dimentions()) * config.pixels_per_meter, color);
            gizmos.line_2d((cell_top_left + map.get_cell_dimentions().with_x(0.)) * config.pixels_per_meter, (cell_top_left + map.get_cell_dimentions().with_y(0.)) * config.pixels_per_meter, color);
        }
    }

}

pub fn draw_proximity(config: Res<DisplayConfiguration>, mut gizmos: Gizmos, fields: Res<EntityMultiField<TargetProximity>>, selected: Res<SelectedItem>, objectives: Query<(Entity, &Ordering)>){

    let selected_entity = objectives.iter().filter(|x| x.1.0 == selected.0).last();

    let entity = match selected_entity {
        Some((e, _)) => e,
        None => {
            println!("No Entities with orderring {:?}", selected.0);
            return;
        },
    };

    let map = fields.get(&entity).unwrap();

    let global_offset = Vec2::new(map.get_columns() as f32, map.get_rows() as f32) / 2.;
    
    for x in 0..map.get_columns() {
        for y in 0..map.get_rows() {
            
            if let Some(TargetProximity::Obstacle) = map.get(&IVec2::new(x as i32, y as i32)){
                continue;
            }

            let color = match map.get(&IVec2::new(x as i32, y as i32)) {
                Some(TargetProximity::NotComputed) => Color::from(PURPLE_500),
                Some(TargetProximity::Computed(value)) => Color::from(GREEN_500).with_alpha(1./(value + 1.)),
                _ => continue,
            };
        
            let cell_size = map.get_cell_dimentions() * config.pixels_per_meter;
            let cell_top_left = (map.get_area().center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.get_cell_dimentions()) * config.pixels_per_meter;

            gizmos.line_2d(cell_top_left, cell_top_left + cell_size, color);
            gizmos.line_2d(cell_top_left + cell_size.with_x(0.), cell_top_left + cell_size.with_y(0.), color);
        }
    }

}

pub fn draw_vectors(config: Res<DisplayConfiguration>, mut gizmos: Gizmos, fields: Res<EntityMultiField<Vec2>>, selected: Res<SelectedItem>, objectives: Query<(Entity, &Ordering)>){

    let selected_entity = objectives.iter().filter(|x| x.1.0 == selected.0).last();

    let entity = match selected_entity {
        Some((e, _)) => e,
        None => {
            println!("No Entities with orderring {:?}", selected.0);
            return;
        },
    };

    let map = fields.get(&entity).unwrap();

    let global_offset = Vec2::new(map.get_columns() as f32, map.get_rows() as f32) / 2.;
    

    for x in 0..map.get_columns() {
        for y in 0..map.get_rows() {
            
            if let Some(value) = map.get(&IVec2::new(x as i32,  y as i32)){
                
                let cell_top_left = (map.get_area().center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.get_cell_dimentions()) * config.pixels_per_meter;
                let cell_center = cell_top_left + Vec2::new(0.5, 0.5) * map.get_cell_dimentions() * config.pixels_per_meter;
                gizmos.arrow_2d(cell_center, cell_center + value * config.pixels_per_meter, PURPLE_500);

            }
        
            
        }
    }

}

pub fn draw_density(
    config: Res<DisplayConfiguration>, 
    mut gizmos: Gizmos, 
    fields: Res<EntityMultiField<AgentDensity>>, 
    selected: Res<SelectedItem>, 
    objectives: Query<(Entity, &Ordering)>
){

    let selected_entity = objectives.iter().filter(|x| x.1.0 == selected.0).last();

    let entity = match selected_entity {
        Some((e, _)) => e,
        None => {
            println!("No Entities with orderring {:?}", selected.0);
            return;
        },
    };

    let map = fields.get(&entity).unwrap();

    let global_offset = Vec2::new(map.get_columns() as f32, map.get_rows() as f32) / 2.;
    
    let max = map.get_grid().as_vec().iter().map(|x| x.value()).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

    for x in 0..map.get_columns() {
        for y in 0..map.get_rows() {

            let color = match map.get(&IVec2::new(x as i32, y as i32)) {
                Some(value) => {
                    let alpha = value.value() / max;
                    Color::from(ORANGE_500).with_alpha(alpha)
                },
                _ => continue,
            };

            let cell_dimentions = map.get_cell_dimentions() * config.pixels_per_meter;
            let cell_top_left = (map.get_area().center() + (Vec2::new(x as f32,  y as f32) - global_offset) * map.get_cell_dimentions()) * config.pixels_per_meter;

            gizmos.line_2d(cell_top_left, cell_top_left + cell_dimentions, color);
            gizmos.line_2d(cell_top_left + cell_dimentions.with_x(0.), cell_top_left + cell_dimentions.with_y(0.), color);
        }
    }

}