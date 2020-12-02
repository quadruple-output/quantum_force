#![feature(bool_to_option)]

mod components;
mod plugins;
mod systems;

use bevy::prelude::*;

fn add_default_entities(mut commands: Commands) {
    use components::{BasicShape, Position};
    commands
        .spawn((BasicShape::sphere(5.0), Position::origin()))
        .spawn((BasicShape::sphere(2.0), Position::at(1.0, 2.0, 3.0)));
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::Debug)
        .add_startup_system(add_default_entities.system())
        .run();
}
