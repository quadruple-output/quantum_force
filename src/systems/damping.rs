use bevy::utils::Duration;
use bevy::{core::FixedTimesteps, prelude::*};

use crate::{
    components::{Damping, Velocity},
    PausePhysics, PHYSICS_TIMESTEP,
};

pub fn adjust_velocity(
    pause: Res<PausePhysics>,
    commands: &mut Commands,
    ts: Res<FixedTimesteps>,
    mut query: Query<(Entity, &Damping, &mut Velocity)>,
) {
    if !pause.0 {
        let dt = Duration::from_secs_f64(ts.get(PHYSICS_TIMESTEP).unwrap().step());
        for (entity, &damping, mut v) in query.iter_mut() {
            *v *= 1.0 - damping * dt;
            if *v < 0.01 {
                *v = Velocity::default();
                commands.remove_one::<Damping>(entity);
            }
        }
    }
}
