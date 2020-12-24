use super::super::components::{Acceleration, Force, Mass};
use crate::common::resources::PausePhysics;
use bevy::prelude::*;

pub fn reset(
    time: Res<Time>,
    mut pause_physics: ResMut<PausePhysics>,
    mut forces: Query<&mut Force>,
) {
    if time.seconds_since_startup() > 3.0 {
        pause_physics.0 = false;
        forces.iter_mut().for_each(|mut f| *f = Default::default());
    }
}

pub fn accelerate_masses(
    pause: Res<PausePhysics>,
    mut query: Query<(&Force, &Mass, &mut Acceleration)>,
) {
    if !pause.0 {
        for (&f, &m, mut a) in query.iter_mut() {
            *a = f / m;
        }
    }
}
