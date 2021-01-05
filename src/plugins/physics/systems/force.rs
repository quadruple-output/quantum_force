use super::super::{
    components::{Acceleration, Force, Mass, Velocity},
    resources::TimeStepState,
};
use bevy::prelude::*;
use std::time::Duration;

pub fn reset(mut forces: Query<&mut Force>) {
    forces.iter_mut().for_each(|mut f| *f = Default::default());
}

pub fn accelerate_masses(
    mut ts: ResMut<TimeStepState>,
    mut query: Query<(&Force, &Mass, &mut Acceleration, &Velocity)>,
) {
    for (&f, &m, mut a, v) in query.iter_mut() {
        *a = f / m;
        let a_abs = a.0.length();
        let v_abs = v.0.length();
        // |a|*dt should be less than 0.01:
        // a_abs*dt = 0.01;
        // dt = 0.01/a_abs;
        if a_abs > 1000. * f32::EPSILON {
            ts.limit_step_duration(Duration::from_secs_f32(0.01 / a_abs));
        }
        // same for v*dt:
        if v_abs > 1000. * f32::EPSILON {
            ts.limit_step_duration(Duration::from_secs_f32(0.05 / v_abs));
        }
    }
}
