use super::super::components::{Acceleration, Force, Mass};
use bevy::prelude::*;

pub fn reset(mut forces: Query<&mut Force>) {
    forces.iter_mut().for_each(|mut f| *f = Default::default());
}

pub fn accelerate_masses(mut query: Query<(&Force, &Mass, &mut Acceleration)>) {
    for (&f, &m, mut a) in query.iter_mut() {
        *a = f / m;
    }
}
