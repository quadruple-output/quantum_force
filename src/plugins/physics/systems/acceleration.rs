use super::super::{components::Acceleration, PHYSICS_TIMESTEP};
use crate::common::{components::Velocity, resources::PausePhysics};
use bevy::{core::FixedTimesteps, prelude::*, utils::Duration};

pub fn adjust_velocity(
    pause: Res<PausePhysics>,
    ts: Res<FixedTimesteps>,
    mut query: Query<(&Acceleration, &mut Velocity)>,
) {
    if !pause.0 {
        let dt = Duration::from_secs_f64(ts.get(PHYSICS_TIMESTEP).unwrap().step());
        query.iter_mut().for_each(|(&a, mut v)| {
            *v += a * dt;
        });
    }
}
