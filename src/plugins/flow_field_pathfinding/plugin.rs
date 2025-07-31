use bevy::prelude::*;

use crate::{plugins::simulation_area::resources::SimulationArea, Obstacle};

use super::{models::{AgentDensity, BlockedStatus, TargetProximity, TargetStatus}, resources::*, systems::*};

pub struct FlowFieldPathfindingPlugin{
    pub cell_size: f32
}

impl Plugin for FlowFieldPathfindingPlugin {
    fn build(&self, app: &mut App) {

        let cell_size = self.cell_size;

        app
        .insert_state(PathFindingOverlayState::ShowNone)
        .insert_state(ShowGridState::HideGrid)
        .insert_resource(SelectedItem(0));

        register_multi_field::<TargetStatus>(app);
        register_multi_field::<TargetProximity>(app);
        register_multi_field::<Vec2>(app);
        register_multi_field::<AgentDensity>(app);

        app.add_systems(PreStartup, move |simulation_area: Res<SimulationArea>, commands: Commands| {
            add_configuration(simulation_area, commands, cell_size);
        })

        .add_systems(Startup, add_field_map::<BlockedStatus>)

        .add_systems(PreUpdate, handle_grid_state_inputs)
        .add_systems(PreUpdate, handle_overlay_inputs)
        .add_systems(PreUpdate, handle_selection_inputs)
        
        .add_systems(Update, 
            (
                (compute_colision_map::<BlockedStatus, Obstacle>, compute_objective_colision_map, compute_density_map),
                compute_proximity_map,
                compute_vector_map
            ).chain().in_set(FlowFieldSystemSet::ComputeFields)
        )
        
        .add_systems(PostUpdate, draw_grid.run_if(in_state(ShowGridState::ShowGrid)))

        .add_systems(PostUpdate, draw_obstacles.run_if(in_state(PathFindingOverlayState::ShowObstacles)))
        .add_systems(PostUpdate, draw_targets.run_if(in_state(PathFindingOverlayState::ShowTargets)))
        .add_systems(PostUpdate, draw_proximity.run_if(in_state(PathFindingOverlayState::ShowProimity)))
        .add_systems(PostUpdate, draw_vectors.run_if(in_state(PathFindingOverlayState::ShowVectorField)))
        .add_systems(PostUpdate, draw_density.run_if(in_state(PathFindingOverlayState::ShowDensityField)))
        
        .add_systems(Last, remove_field_for_objectives);
    }
}

fn add_configuration(simulation_area: Res<SimulationArea>, mut commands: Commands, cell_size: f32){
    let ratio: Vec2 = simulation_area.0.size() / cell_size * Vec2::ONE;
    let columns = ratio.x.ceil() as usize;
    let rows = ratio.y.ceil() as usize;
    
    commands.insert_resource(GridCellSize{ rows, columns});
}

fn register_multi_field<T>(app: &mut App) where T: Default + Clone + Send + Sync + 'static{
    app
    .add_systems(Startup, add_entity_multi_field::<T>)
    .add_systems(PreUpdate, add_field_for_objectives::<T>)
    .add_systems(Last, remove_field_for_objectives);
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FlowFieldSystemSet {
    ComputeFields,
}