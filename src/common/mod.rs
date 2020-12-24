pub mod components;
pub mod events;
pub mod resources;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<resources::PausePhysics>()
            .add_event::<events::CameraControl>();
    }
}
