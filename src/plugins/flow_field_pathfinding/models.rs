use std::fmt;

use derive_more::{Add, From, Into, Mul};

pub trait CellStatus : Default + Send + Sync + Copy + PartialEq + 'static{
    fn get_non_default_value() -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BlockedStatus {
    #[default]
    Empty,
    Blocked
}

impl CellStatus for BlockedStatus {
    fn get_non_default_value() -> Self {
        Self::Blocked
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum TargetStatus {
    #[default]
    NotTarget,
    IsTarget,
}

impl CellStatus for TargetStatus {
    fn get_non_default_value() -> Self {
        Self::IsTarget
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum TargetProximity {
    #[default]
    NotComputed,
    Obstacle,
    Buffer,
    Computed(f32)
}

#[derive(Clone, Copy, Debug, PartialEq, Default, From, Into, Add, Mul)]
pub struct AgentDensity(f32);

impl AgentDensity {
    pub fn value(&self) -> f32 {
        return self.0;
    }
}

impl fmt::Display for AgentDensity
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}