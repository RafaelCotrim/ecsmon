use crate::components::prelude::*;
use crate::plugins::start_time::resources::StartTime;
use bevy::diagnostic::FrameCount;
use bevy::prelude::*;
use chrono::{DateTime, Local};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;

use super::components::PreviousPosition;
use super::configuration::ExportOptions;
use super::resources::{DataEntry, DataEntryStore};

pub fn track_agents(
    mut store: ResMut<DataEntryStore>,
    frame: Res<FrameCount>,
    agents: Query<(Entity, &Position, &PreviousPosition), With<Agent>>,
) {
    for (entity, position, previous) in agents.iter() {
        let entry = DataEntry {
            entity,
            start_pos: previous.value(),
            end_pos: position.value(),
            frame: frame.0,
        };

        store.add(entry);
    }
}

pub fn export_data(
    start_time: Res<StartTime>,
    config: Res<ExportOptions>,
    mut store: ResMut<DataEntryStore>,
) {
    let time = start_time.0.format("%Y-%m-%d-%H-%M-%S").to_string();

    let path = config.out.replace("{time}", &time);
    write_data_and_clear_store(&path, &mut store);
}

pub fn export_data_on_close(
    exit_events: EventReader<AppExit>,
    start_time: Res<StartTime>,
    config: Res<ExportOptions>,
    mut store: ResMut<DataEntryStore>,
) {
    if exit_events.len() > 0 {
        let time = start_time.0.format("%Y-%m-%d-%H-%M-%S").to_string();

        let path = config.out.replace("{time}", &time);
        write_data_and_clear_store(&path, &mut store);
    }
}

pub fn record_previous_speed(
    mut agents: Query<(&Position, &mut PreviousPosition), (With<Agent>, Changed<Position>)>,
) {
    for (position, mut previous) in agents.iter_mut() {
        previous.set_value(position.value());
    }
}

pub fn add_previous_position_component_to_agents(
    mut commands: Commands,
    agents: Query<(Entity, &Position), (Added<Agent>, Without<PreviousPosition>)>,
) {
    for (entity, position) in agents.iter() {
        commands
            .entity(entity)
            .insert(PreviousPosition::new(position.value()));
    }
}

pub fn run_every_n_frames(frames: Res<FrameCount>, config: Res<ExportOptions>) -> bool {
    frames.0 != 0 && frames.0 % config.export_interval == 0
}

fn write_data_and_clear_store(path: &str, store: &mut DataEntryStore) {

    if store.len() == 0 {
        return;
    }

    let path = Path::new(&path);

    if let Some(parent) = path.parent() {
        create_dir_all(parent).unwrap();
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    let data = format!("{}", store);
    file.write_all(data.as_bytes()).unwrap();

    store.clear();
}
