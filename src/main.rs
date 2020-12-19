#![feature(bool_to_option)]

mod components;
mod entities;
mod plugins;
mod systems;

//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use components::Spin;
use entities::particle::{Particle, ParticleAssets};
use systems::*;

fn add_default_entities(
    commands: &mut Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    particle_assets: Res<ParticleAssets>,
) {
    commands
        // .spawn(PbrBundle {
        //     mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        //     ..Default::default()
        // })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                // transform: Transform::from_translation(Vec3::new(-6.0, 10.0, 16.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
    Particle::new()
        .with_mass(2.0)
        .with_spin(Spin::Up)
        .with_velocity(Vec3::new(-0.1, 0.15, 0.6))
        .spawn_at(Vec3::new(1.5, 1.0, 0.0), commands, &particle_assets);
    Particle::new()
        .with_mass(2.0)
        .with_spin(Spin::Down)
        .with_velocity(Vec3::new(0.0, -0.1, -0.6))
        .spawn_at(Vec3::new(-1.5, 1.0, 0.0), commands, &particle_assets);
    Particle::new()
        .with_mass(0.5)
        .with_velocity(Vec3::new(-0.2, 0.0, 0.0))
        .spawn_at(Vec3::new(-3.0, 3.0, 3.0), commands, &particle_assets)
}

fn main() {
    App::build()
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "#!> Quantum  Force <!#".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(PrintDiagnosticsPlugin::default())
        //.add_plugin(plugins::Debug)
        .init_resource::<ParticleAssets>()
        .add_startup_system(add_default_entities.system())
        .add_system(force::reset.system())
        .add_system(gravity::calculate_forces.system())
        .add_system(force::apply_to_masses.system())
        .add_system(velocity::r#move.system())
        .add_system(spin::spin.system())
        .add_system(Particle::animate.system())
        .run();
}
