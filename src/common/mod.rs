pub mod components;
pub mod resources;
pub mod types;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<resources::PausePhysics>()
            .add_event::<components::CameraControl>();
    }
}
