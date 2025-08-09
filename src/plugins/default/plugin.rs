use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::{app::{PluginGroup, PluginGroupBuilder}, window::{Window, WindowPlugin}, DefaultPlugins};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_prototype_lyon::prelude::*;

use crate::plugins::display::plugin::DisplayPlugin;

pub struct ECSMosDefaultPlugins;

impl PluginGroup for ECSMosDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()

        // Bevy Base
        .add_group(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            }
        ))

        // External
        .add(FpsCounterPlugin)
        .add(PanCamPlugin::default())
        .add(LogDiagnosticsPlugin::default())
        .add(ShapePlugin)

        // Internal
        .add_after::<PanCamPlugin>(DisplayPlugin)
    }
}