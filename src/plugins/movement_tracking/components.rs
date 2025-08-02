use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct PreviousPosition(Vec2);

impl PreviousPosition {
    pub fn new(data: Vec2) -> Self {
        PreviousPosition(data)
    }

    pub fn value(&self) -> Vec2 {
        self.0
    }

    pub fn set_value(&mut self, value: Vec2) {
        self.0 = value;
    }
}

impl From<Vec2> for PreviousPosition {
    fn from(value: Vec2) -> Self {
        PreviousPosition::new(value)
    }
}
