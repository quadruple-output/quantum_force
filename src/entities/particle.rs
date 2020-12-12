use crate::components::{Force, Mass, Spin, Velocity};
use bevy::prelude::*;

#[derive(Default)]
pub struct ParticleAssets {
    quant: QuantAssets,
    sphere: AssetHandles,
}

#[derive(Default)]
struct AssetHandles {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Default)]
struct QuantAssets {
    mesh: Handle<Mesh>,
    material_weight: Handle<StandardMaterial>,
    material_inertia: Handle<StandardMaterial>,
}

#[derive(Copy, Clone)]
pub struct Particle {
    spin: Spin,
    mass: Option<f32>,
    velocity: Vec3,
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ParticleAssets::default())
            .add_startup_system(Particle::startup_system.system());
    }
}

impl Particle {
    fn startup_system(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut my_assets: ResMut<ParticleAssets>,
    ) {
        my_assets.quant.mesh = meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.0, // scaled by consumer
            subdivisions: 1,
        }));
        my_assets.quant.material_weight = materials.add(Color::rgb(1.0, 0.0, 0.0).into());
        my_assets.quant.material_inertia = materials.add(Color::rgb(1.0, 1.0, 0.0).into());
        my_assets.sphere.mesh = meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.0, // scaled by consumer
            subdivisions: 4,
        }));
        my_assets.sphere.material = materials.add(Color::rgba(0.0, 0.0, 1.0, 0.3).into());
    }

    pub fn new() -> Self {
        Self {
            spin: Spin::Up,
            mass: None,
            velocity: Vec3::default(),
        }
    }

    pub fn with_spin(mut self, spin: Spin) -> Self {
        self.spin = spin;
        self
    }

    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = Some(mass);
        self
    }

    pub fn with_velocity(mut self, velocity: Vec3) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn spawn_at(
        &self,
        position: Vec3,
        commands: &mut Commands,
        my_assets: &Res<ParticleAssets>,
    ) {
        commands
            .spawn(PbrComponents {
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .with(Velocity(self.velocity))
            .with(Force::default());
        if let Some(mass) = self.mass {
            commands.with(Mass(mass));
        };
        commands.with_children(|children| {
            children
                // inner quants:
                .spawn(PbrComponents {
                    transform: Transform::from_scale(Vec3::one() * 0.05),
                    ..Default::default()
                })
                .with(self.spin)
                .with_children(|inner_quants| {
                    inner_quants
                        .spawn(PbrComponents {
                            mesh: my_assets.quant.mesh.clone(),
                            material: my_assets.quant.material_weight.clone(),
                            transform: Transform::from_translation(Vec3::unit_x() * 2.0),
                            ..Default::default()
                        })
                        .spawn(PbrComponents {
                            mesh: my_assets.quant.mesh.clone(),
                            material: my_assets.quant.material_inertia.clone(),
                            transform: Transform::from_translation(Vec3::unit_x() * -2.0),
                            ..Default::default()
                        });
                });
            children
                // semi transparent sphere:
                .spawn(PbrComponents {
                    mesh: my_assets.sphere.mesh.clone(),
                    material: my_assets.sphere.material.clone(),
                    transform: Transform::from_scale(Vec3::one() * 0.2),
                    ..Default::default()
                })
                .spawn(PbrComponents {
                    mesh: my_assets.sphere.mesh.clone(),
                    material: my_assets.sphere.material.clone(),
                    transform: Transform::from_scale(Vec3::one() * 0.4),
                    ..Default::default()
                })
                .spawn(PbrComponents {
                    mesh: my_assets.sphere.mesh.clone(),
                    material: my_assets.sphere.material.clone(),
                    transform: Transform::from_scale(Vec3::one() * 0.6),
                    ..Default::default()
                });
        });
    }
}
