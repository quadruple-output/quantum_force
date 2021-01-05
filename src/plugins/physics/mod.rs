pub mod components;
mod resources;
mod systems;

use self::systems::{acceleration, debug, force, gravity, repulsion, velocity, RunCriteria};
use bevy::prelude::*;
use systems::adaptive_time_step::AdaptiveTimeStep;

#[derive(Clone, Copy)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<resources::TimeStepState>()
            .add_stage_before(
                stage::UPDATE,
                "fixed_step_physics",
                SystemStage::parallel()
                    .with_run_criteria(RunCriteria::from(Box::new(AdaptiveTimeStep::default())))
                    .with_system(force::reset.system())
                    .with_system(gravity::calculate_forces.system())
                    .with_system(repulsion::calculate_forces.system())
                    .with_system(force::accelerate_masses.system())
                    .with_system(debug::print_stats.system())
                    .with_system(acceleration::adjust_velocity.system())
                    .with_system(velocity::adjust_position.system()),
            );
    }
}
