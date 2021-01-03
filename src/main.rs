mod common;
mod plugins;

use self::plugins::*;
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugin(setup_window::Plugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(common::Plugin)
        .add_plugin(physics::Plugin)
        .add_plugin(shapes::Plugin)
        .add_plugin(particle::Plugin)
        .add_plugin(input::Plugin)
        .add_plugin(camera::Plugin)
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
        .add_startup_system(add_default_entities.system())
        .run();
}

fn add_default_entities(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    particle_assets: Res<particle::ParticleAssets>,
) {
    use self::plugins::particle::{Particle, Spin};

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.0, 0.5, 0.5).into()),
        ..Default::default()
    });

    // Light Source ----------------------------------------
    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    // Walls ----------------------------------------
    let wall = shapes::build_wall()
        .with_dimensions(10.0, 10.0, 0.5, &mut meshes)
        .with_material(materials.add(Color::GOLD.into()));
    wall.clone()
        .center_at(Vec3::unit_x() * 5.0)
        .facing(-Vec3::unit_x(), Vec3::unit_y())
        .spawn(commands);
    wall.clone()
        .center_at(Vec3::unit_x() * -5.0)
        .facing(Vec3::unit_x(), Vec3::unit_y())
        .spawn(commands);
    wall.clone()
        .center_at(Vec3::unit_z() * -5.0)
        .facing(Vec3::unit_z(), Vec3::unit_y())
        .spawn(commands);
    wall.center_at(Vec3::unit_z() * 5.0)
        .facing(-Vec3::unit_z(), Vec3::unit_y())
        .spawn(commands);

    // Particles ----------------------------------------
    Particle::builder()
        .with_mass(2.0)
        .with_spin(Spin::Up)
        .with_velocity(Vec3::new(0.0, 0.0, -1.0))
        .spawn_at(Vec3::new(1.5, 1.0, 0.0), commands, &particle_assets);
    Particle::builder()
        .with_mass(2.0)
        .with_spin(Spin::Down)
        .with_velocity(Vec3::new(0.0, 0.0, 0.0))
        .spawn_at(Vec3::new(-1.5, 1.0, 0.0), commands, &particle_assets);
    Particle::builder()
        .with_mass(0.5)
        .with_velocity(Vec3::new(0.8, 0.0, 0.0))
        .spawn_at(Vec3::new(-2.0, 3.0, 2.0), commands, &particle_assets);
    Particle::builder()
        .with_mass(0.5)
        .with_spin(Spin::Down)
        .with_velocity(Vec3::new(-0.8, 0.0, 0.0))
        .spawn_at(Vec3::new(1.0, 3.0, -2.0), commands, &particle_assets);
}
