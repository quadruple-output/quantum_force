use super::super::{Plugin, PHYSICS_TIMESTEP};
use crate::common::{components::Velocity, resources::PausePhysics};
use bevy::{core::FixedTimesteps, prelude::*, utils::Duration};

pub fn adjust_position(
    pause: Res<PausePhysics>,
    ts: Res<FixedTimesteps>,
    mut query: Query<(&Velocity<Plugin>, &mut Transform)>,
) {
    if !pause.0 {
        let dt = Duration::from_secs_f64(ts.get(PHYSICS_TIMESTEP).unwrap().step());
        for (&v, mut pos) in query.iter_mut() {
            pos.translation += v * dt;
        }
    }
}
