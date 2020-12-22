use crate::PHYSICS_TIMESTEP;
use crate::{components::Velocity, PausePhysics};
use bevy::core::FixedTimesteps;
use bevy::prelude::*;
use bevy::utils::Duration;

pub fn adjust_position(
    pause: Res<PausePhysics>,
    ts: Res<FixedTimesteps>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    if !pause.0 {
        let dt = Duration::from_secs_f64(ts.get(PHYSICS_TIMESTEP).unwrap().step());
        for (&v, mut pos) in query.iter_mut() {
            pos.translation += v * dt;
        }
    }
}
