use super::super::{
    components::{Acceleration, Velocity},
    PHYSICS_TIMESTEP,
};
use bevy::{core::FixedTimesteps, prelude::*, utils::Duration};

pub fn adjust_velocity(ts: Res<FixedTimesteps>, mut query: Query<(&Acceleration, &mut Velocity)>) {
    let dt = Duration::from_secs_f64(ts.get(PHYSICS_TIMESTEP).unwrap().step());
    query.iter_mut().for_each(|(&a, mut v)| {
        *v += a * dt;
    });
}
