mod systems;

use self::systems::{react_on_input, tmp_start_physics};
use bevy::{input::system::exit_on_esc_system, prelude::*};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(
            stage::EVENT,
            "qf_input",
            SystemStage::serial()
                .with_system(exit_on_esc_system.system())
                .with_system(react_on_input.system())
                .with_system(tmp_start_physics.system()),
        );
    }
}
