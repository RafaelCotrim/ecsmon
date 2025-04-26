pub mod plugin{
    use bevy::{app::{App, Plugin, Update}, ecs::schedule::IntoSystemConfigs};

    use crate::plugins::kinematics::plugin::KinematicsSet;

    use super::systems::*;

    pub struct SimpleObjective;
    
    impl Plugin for SimpleObjective {
        fn build(&self, app: &mut App) {
    
            app.add_systems(Update, agent_araived_at_destination_system.after(KinematicsSet::ApplyVelocity));
        }
    }
}

pub mod systems{
    use bevy::prelude::*;

    use crate::components::prelude::*;

    pub fn agent_araived_at_destination_system(
        mut commands: Commands,
        agents: Query<(Entity, &Position), With<Agent>>,
        destinations: Query<(&Position, &Shape), With<Objective>>,
    ) {
        for (agent, agent_pos) in &agents {
            let agent_position = agent_pos.value();
    
            for (dest_pos, dest_colider) in &destinations {
                let destination_pos = dest_pos.value();
    
                match dest_colider {
                    Shape::Circle(radius) => {
                        let distance = (destination_pos - agent_position).length() - radius;
                        if distance <= 0. {
                            commands.entity(agent).despawn();
                        }
                    }
                    _ => todo!(),
                }
            }
        }
    }
}