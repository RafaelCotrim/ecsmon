use std::fmt::Display;

use bevy::{
    ecs::{entity::Entity, resource::Resource},
    math::Vec2,
};

pub struct DataEntry {
    pub entity: Entity,
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub frame: u32,
}

impl Display for DataEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.entity,
            self.frame,
            self.end_pos.x,
            self.end_pos.y,
            self.start_pos.x,
            self.start_pos.y
        )
    }
}

#[derive(Resource)]
pub struct DataEntryStore(Vec<DataEntry>);

impl DataEntryStore {
    pub fn new() -> Self {
        DataEntryStore(Vec::new())
    }

    pub fn add(&mut self, entry: DataEntry) {
        self.0.push(entry);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for DataEntryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.0 {
            writeln!(f, "{}", entry)?;
        }
        Ok(())
    }
}
