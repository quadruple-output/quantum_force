use super::super::{
    components::{Acceleration, Velocity},
    resources::TimeStepState,
};
use bevy::prelude::*;

pub fn adjust_velocity(ts: Res<TimeStepState>, mut query: Query<(&Acceleration, &mut Velocity)>) {
    let dt = ts.get_step_duration();
    query.iter_mut().for_each(|(&a, mut v)| {
        *v += a * dt;
    });
}
