pub mod plugin {
    use bevy::app::{Last, Plugin};

    use super::systems::exit_when_no_agents;

    pub struct AutoEndSimulationPlugin;

    impl Plugin for AutoEndSimulationPlugin {
        fn build(&self, app: &mut bevy::app::App) {
            app.add_systems(Last, exit_when_no_agents);
        }
    }
}

pub mod systems {
    use bevy::prelude::*;

    use crate::components::prelude::Agent;

    pub fn exit_when_no_agents(
        mut app_exit_events: EventWriter<AppExit>,
        agents: Query<Entity, With<Agent>>,
    ) {
        if agents.iter().len() == 0 {
            app_exit_events.write(AppExit::Success);
        }
    }
}
