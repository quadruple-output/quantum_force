#![feature(bool_to_option)]

mod components;
mod entities;
mod plugins;
mod systems;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use systems::*;

fn add_default_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
    let particle = entities::Particle::new().with_mass(1.0);
    particle
        .clone()
        .with_velocity(Vec3::new(0.0, 0.0, 0.3))
        .spawn_at(
            Vec3::new(1.0, 1.0, 0.0),
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    particle.with_velocity(Vec3::new(0.0, 0.0, -0.3)).spawn_at(
        Vec3::new(-1.0, 1.0, 0.0),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::Debug)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(add_default_entities.system())
        .add_system(force::reset.system())
        .add_system(gravity::calculate_forces.system())
        .add_system(force::apply_to_masses.system())
        .add_system(velocity::r#move.system())
        .run();
}
