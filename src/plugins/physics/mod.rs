pub mod components;
mod systems;

use self::systems::{acceleration, force, gravity, velocity, RunCriteria};
use bevy::{core::FixedTimestep, prelude::*};

const PHYSICS_TIMESTEP: &str = "PHYSICS_TIMESTEP";

#[derive(Clone, Copy)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_stage_before(
            stage::UPDATE,
            "fixed_step_physics",
            SystemStage::parallel()
                .with_run_criteria(RunCriteria::from(
                    FixedTimestep::steps_per_second(200.0).with_label(PHYSICS_TIMESTEP),
                ))
                .with_system(force::reset.system())
                .with_system(gravity::calculate_forces.system())
                .with_system(force::accelerate_masses.system())
                .with_system(acceleration::adjust_velocity.system())
                .with_system(velocity::adjust_position.system()),
        );
    }
}
