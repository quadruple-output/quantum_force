#![feature(bool_to_option)]

mod components;
mod entities;
mod plugins;
mod systems;

use bevy::{
    core::FixedTimestep,
    //diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
};
use components::{Damping, Spin, Velocity};
use entities::{
    main_camera::MainCamera,
    particle::{Particle, ParticleAssets},
};
use systems::*;

const PHYSICS_TIMESTEP: &str = "PHYSICS_TIMESTEP";

pub struct PausePhysics(bool);
impl FromResources for PausePhysics {
    fn from_resources(_resources: &Resources) -> Self {
        Self(true)
    }
}

fn add_default_entities(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    particle_assets: Res<ParticleAssets>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.0, 0.5, 0.5).into()),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(MainCamera)
        .with(Velocity(Vec3::unit_x() * 5.0))
        .with(Damping::half_live(1.0));
    Particle::new()
        .with_mass(2.0)
        .with_spin(Spin::Up)
        .with_velocity(Vec3::new(0.0, 0.0, 0.0))
        //.with_velocity(Vec3::new(-0.1, 0.15, 0.6))
        .spawn_at(Vec3::new(1.5, 1.0, 0.0), commands, &particle_assets);
    Particle::new()
        .with_mass(2.0)
        .with_spin(Spin::Down)
        .with_velocity(Vec3::new(0.0, 0.0, 0.0))
        //.with_velocity(Vec3::new(0.0, -0.1, -0.6))
        .spawn_at(Vec3::new(-1.5, 1.0, 0.0), commands, &particle_assets);
    Particle::new()
        .with_mass(0.5)
        .with_velocity(Vec3::new(0.8, 0.0, 0.0))
        .spawn_at(Vec3::new(-2.0, 3.0, 2.0), commands, &particle_assets);
    Particle::new()
        .with_mass(0.5)
        .with_spin(Spin::Down)
        .with_velocity(Vec3::new(-0.8, 0.0, 0.0))
        .spawn_at(Vec3::new(1.0, 3.0, -2.0), commands, &particle_assets);
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
        .init_resource::<ParticleAssets>()
        .init_resource::<PausePhysics>()
        .add_startup_system(add_default_entities.system())
        .add_system(spin::spin.system())
        .add_system(Particle::animate.system())
        .add_stage_after(
            stage::UPDATE,
            "fixed_step_physics",
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::steps_per_second(200.0).with_label(PHYSICS_TIMESTEP),
                )
                .with_system(force::reset.system())
                .with_system(gravity::calculate_forces.system())
                .with_system(force::accelerate_masses.system())
                .with_system(acceleration::adjust_velocity.system())
                .with_system(damping::adjust_velocity.system())
                .with_system(velocity::adjust_position.system())
                .with_system(MainCamera::aim.system()), // having it in UPDATE stage causes unsteady movements
        )
        .run();
}
