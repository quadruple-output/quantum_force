use crate::components::Velocity;
use bevy::prelude::*;

pub fn r#move(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    dbg!(time.delta_seconds_f64());
    dbg!(time.seconds_since_startup());
    query
        .iter_mut()
        .for_each(|(&v, mut t)| t.translation += v * time.delta())
}
