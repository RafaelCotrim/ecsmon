pub mod plugin {
    use bevy::app::Plugin;
    use chrono::Utc;

    use crate::plugins::start_time::resources::StartTime;

    pub struct StartTimePluging;

    impl Plugin for StartTimePluging {
        fn build(&self, app: &mut bevy::app::App) {
            let startup_time = Utc::now();

            app.insert_resource(StartTime(startup_time));
        }
    }
}


pub mod resources {
    use bevy::ecs::resource::Resource;
    use chrono::{DateTime, Utc};

    #[derive(Resource)]
    pub struct StartTime(pub DateTime<Utc>);
}
