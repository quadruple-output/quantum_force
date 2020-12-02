use std::ops::{Deref, DerefMut};

use crate::components::{BasicShape, Position};
use bevy::prelude::*;

pub struct UpdateIntervalTimer(pub Timer);

impl Deref for UpdateIntervalTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UpdateIntervalTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn show_geometries_in_world(
    time: Res<Time>,
    mut timer: ResMut<UpdateIntervalTimer>,
    query: Query<(&BasicShape, &Position)>,
) {
    timer.tick(time.delta_seconds);
    timer.finished.then(|| {
        query
            .iter()
            .for_each(|(shape, position)| println!("{:?} at {:?}", shape, position))
    });
}
