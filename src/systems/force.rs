use crate::components::{Force, Mass, Velocity};
use bevy::prelude::*;

pub fn reset(mut forces: Query<&mut Force>) {
    forces.iter_mut().for_each(|mut f| f.0 = Default::default());
}

pub fn apply_to_masses(time: Res<Time>, mut query: Query<(&Force, &Mass, &mut Velocity)>) {
    for (f, m, mut v) in query.iter_mut() {
        let a = *f / *m;
        *v += a * time.delta;
    }
}
