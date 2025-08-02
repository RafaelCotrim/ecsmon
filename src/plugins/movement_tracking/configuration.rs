use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct ExportOptions{
    pub export_interval: u32,
    pub out: String,
}