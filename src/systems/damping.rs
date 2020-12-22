use bevy::utils::Duration;
use bevy::{core::FixedTimesteps, prelude::*};

use crate::{
    components::{Damping, Velocity},
    PausePhysics, PHYSICS_TIMESTEP,
};

pub fn adjust_velocity(
    pause: Res<PausePhysics>,
    ts: Res<FixedTimesteps>,
    mut query: Query<(&Damping, &mut Velocity)>,
) {
    const UNNOTICEABLE_VELOCITY: f32 = 0.01;
    if !pause.0 {
        let dt = Duration::from_secs_f64(ts.get(PHYSICS_TIMESTEP).unwrap().step());
        for (&damping, mut v) in query.iter_mut() {
            if *v >= UNNOTICEABLE_VELOCITY {
                assert!(damping >= 0.0); // don't want to accelerate
                *v *= 1.0 - damping * dt;
            } else if *v != Velocity::default() {
                *v = Velocity::default();
            }
        }
    }
}
