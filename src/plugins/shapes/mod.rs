mod components;
mod entities;
mod resources;
mod tools;

pub use entities::build_wall;

use resources::NcollideContext;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<NcollideContext>();
    }
}
