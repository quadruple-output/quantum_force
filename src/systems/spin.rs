use std::f32::consts::PI;

use bevy::prelude::*;

use crate::components::Spin;

pub fn spin(time: Res<Time>, mut query: Query<(&Spin, &mut Transform)>) {
    query
        .iter_mut()
        .for_each(|(spin, mut transform)| match spin {
            Spin::Up => {
                transform.rotate(Quat::from_rotation_y(time.delta_seconds as f32 * 4.0 * PI))
            }
            Spin::Down => {
                transform.rotate(Quat::from_rotation_y(time.delta_seconds as f32 * 4.0 * -PI))
            }
        })
}
