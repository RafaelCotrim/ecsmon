use bevy::prelude::*;


use super::{configuration::ExportOptions, resources::DataEntryStore, systems::*};

pub struct TrackingPlugin {
    pub export_interval: u32,
    pub out: String,
}

impl Default for TrackingPlugin {
    fn default() -> Self {
        Self {
            export_interval: 512,
            out: "./out/{time}.txt".to_string()
        }
    }
}

impl Plugin for TrackingPlugin {
    fn build(&self, app: &mut App) {

        app.insert_resource(ExportOptions {
            export_interval: self.export_interval,
            out: self.out.to_string(),
        });

        app.configure_sets(
            Update,
            TrackingSet::Track.after(add_previous_position_component_to_agents),
        )
        .configure_sets(
            PostUpdate,
            TrackingSet::Export
                .after(TrackingSet::Track)
                .run_if(run_every_n_frames),
        );

        app.insert_resource(DataEntryStore::new())
            .add_systems(PreUpdate, add_previous_position_component_to_agents)
            .add_systems(PreUpdate, record_previous_speed.in_set(TrackingSet::Track))
            .add_systems(PostUpdate, track_agents.in_set(TrackingSet::Track))
            .add_systems(PostUpdate, export_data.in_set(TrackingSet::Export))
            .add_systems(Last, export_data_on_close);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrackingSet {
    Track,
    Export,
}
