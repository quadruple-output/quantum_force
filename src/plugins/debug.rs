use crate::systems::debug as sys_debug;
use bevy::prelude::*;

pub struct Debug;

impl Plugin for Debug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(sys_debug::show_geometries_in_world.system())
            .add_resource(sys_debug::UpdateIntervalTimer(Timer::from_seconds(
                2.0, true,
            )));
    }
}
