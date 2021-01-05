use std::time::Duration;

use super::super::{
    components::{Acceleration, Velocity},
    resources::TimeStepState,
};
use bevy::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Stats {
    time_of_last_stat: Duration,
    min_dt: Duration,
    max_dt: Duration,
    max_a: f32,
    max_v: f32,
    sum_dt: Duration,
    num_steps: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            min_dt: Duration::from_secs(999),
            max_dt: Duration::default(),
            max_a: 0.,
            max_v: 0.,
            time_of_last_stat: Duration::default(),
            sum_dt: Duration::default(),
            num_steps: 0,
        }
    }
}

pub fn print_stats(
    mut stats: Local<Stats>,
    time: Res<Time>,
    ts: Res<TimeStepState>,
    query: Query<(&Acceleration, &Velocity)>,
) {
    //let old_stats = (*stats).clone();
    let dt = ts.get_step_duration_limit().unwrap();
    stats.sum_dt += dt;
    stats.num_steps += 1;
    stats.min_dt = Duration::min(dt, stats.min_dt);
    stats.max_dt = Duration::max(dt, stats.max_dt);
    query.iter().for_each(|(a, v)| {
        stats.max_a = f32::max(stats.max_a, a.0.length() * dt.as_secs_f32());
        stats.max_v = f32::max(stats.max_v, v.0.length() * dt.as_secs_f32());
    });
    if time.time_since_startup() - stats.time_of_last_stat > Duration::from_millis(100) {
        println!(
            "max steps/s: {:?}, max a*dt: {:?}, max v*dt: {:?}, avg steps/s: {:?}",
            (1. / stats.min_dt.as_secs_f32()) as usize,
            stats.max_a,
            stats.max_v,
            (1. / (stats.sum_dt.as_secs_f32() / stats.num_steps as f32)) as usize
        );
        stats.time_of_last_stat = time.time_since_startup();
        stats.num_steps = 0;
        stats.sum_dt = Duration::default();
    }
}
