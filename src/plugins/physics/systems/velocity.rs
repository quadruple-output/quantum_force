use super::super::{components::Velocity, resources::TimeStepState};
use bevy::prelude::*;

pub fn adjust_position(ts: Res<TimeStepState>, mut query: Query<(&Velocity, &mut Transform)>) {
    let dt = ts.get_step_duration();
    for (&v, mut pos) in query.iter_mut() {
        pos.translation += v * dt;
    }
}
