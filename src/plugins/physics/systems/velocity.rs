use super::super::{components::Velocity, PHYSICS_TIMESTEP};
use bevy::{core::FixedTimesteps, prelude::*, utils::Duration};

pub fn adjust_position(ts: Res<FixedTimesteps>, mut query: Query<(&Velocity, &mut Transform)>) {
    let dt = Duration::from_secs_f64(ts.get(PHYSICS_TIMESTEP).unwrap().step());
    for (&v, mut pos) in query.iter_mut() {
        pos.translation += v * dt;
    }
}
