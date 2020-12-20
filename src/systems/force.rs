use crate::{
    components::{Force, Mass, Velocity},
    PHYSICS_TIMESTEP,
};
use bevy::utils::Duration;
use bevy::{core::FixedTimesteps, prelude::*};

pub fn reset(mut forces: Query<&mut Force>) {
    forces.iter_mut().for_each(|mut f| f.0 = Default::default());
}

pub fn apply_to_masses(
    ts: Res<FixedTimesteps>,
    mut query: Query<(&Force, &Mass, &mut Velocity, &mut Transform)>,
) {
    let dt = Duration::from_secs_f64(ts.get(PHYSICS_TIMESTEP).unwrap().step());

    for (&f, &m, mut v, mut pos) in query.iter_mut() {
        let a = f / m;
        *v += a * dt;
        pos.translation += *v * dt;
    }
}
