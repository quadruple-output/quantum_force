mod components;
mod entities;
mod resources;
mod systems;
mod util;

use bevy::prelude::*;

pub use components::Spin;
pub use entities::Particle;
pub use resources::ParticleAssets;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<resources::ParticleAssets>()
            .add_system_to_stage(stage::UPDATE, systems::animate_spin.system())
            .add_system_to_stage(stage::UPDATE, systems::adjust_shadow.system());
    }
}
