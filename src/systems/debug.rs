use crate::components::Velocity;
use bevy::prelude::*;

pub struct UpdateIntervalTimer(pub Timer);

pub fn show_geometries_in_world(
    time: Res<Time>,
    mut timer: ResMut<UpdateIntervalTimer>,
    query: Query<(&Transform, &Velocity)>,
) {
    timer.0.tick(time.delta_seconds);
    timer.0.finished.then(|| {
        query
            .iter()
            .for_each(|(t, v)| println!("{:?} at {:?}", v, t.translation))
    });
}
