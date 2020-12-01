mod components;
mod entities;
mod systems;

use bevy::prelude::*;

use components::Position;
use entities::Sphere;

fn add_default_positions(mut commands: Commands) {
    commands
        .spawn(Sphere::bundle(Position::origin()))
        .spawn((Sphere, Position::origin()))
        .spawn((Sphere, Position::at(1.0, 2.0, 3.0)));
}

fn main() {
    App::build()
        .add_startup_system(add_default_positions.system())
        .add_system(systems::show_positions.system())
        .run();
}
