mod components;
mod systems;

use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(systems::spawn_camera.system())
            .add_system_to_stage(stage::PRE_UPDATE, systems::control_camera.system())
            .add_system_to_stage(stage::UPDATE, systems::slow_down.system())
            .add_system_to_stage(stage::UPDATE, systems::apply_control.system());
    }
}
