use super::{
    components::{LeadingParticle, Spin},
    resources::ParticleAssets,
    util,
};
use crate::{
    common::components::Velocity,
    plugins::physics::{
        self,
        components::{Acceleration, Force, Mass},
    },
};
use bevy::prelude::*;

pub struct Particle;

impl Particle {
    pub fn builder() -> ParticleBuilder {
        ParticleBuilder {
            spin: Spin::Up,
            mass: None,
            velocity: Vec3::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ParticleBuilder {
    spin: Spin,
    mass: Option<f32>,
    velocity: Vec3,
}

impl ParticleBuilder {
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
            .spawn(PbrBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .with(Force::default())
            .with(Acceleration::default())
            .with(Velocity::<physics::Plugin>::from(self.velocity));
        let mut quant_scale = 1.0;
        if let Some(mass) = self.mass {
            commands.with(Mass(mass));
            quant_scale = mass;
        };
        commands.with_children(|children| {
            children
                // inner quants:
                .spawn(PbrBundle {
                    transform: Transform::from_scale(Vec3::one() * 0.05 * quant_scale),
                    ..Default::default()
                })
                .with(self.spin)
                .with_children(|inner_quants| {
                    inner_quants
                        .spawn(PbrBundle {
                            mesh: my_assets.quant.mesh.clone(),
                            material: my_assets.quant.material_weight.clone(),
                            transform: Transform::from_translation(Vec3::unit_x() * 2.0),
                            ..Default::default()
                        })
                        .spawn(PbrBundle {
                            mesh: my_assets.quant.mesh.clone(),
                            material: my_assets.quant.material_inertia.clone(),
                            transform: Transform::from_translation(Vec3::unit_x() * -2.0),
                            ..Default::default()
                        });
                });
            // children
            //     // semi transparent sphere:
            //     .spawn(PbrBundle {
            //         mesh: my_assets.sphere.mesh.clone(),
            //         material: my_assets.sphere.material.clone(),
            //         transform: Transform::from_scale(Vec3::one() * 0.2),
            //         ..Default::default()
            //     })
            //     .spawn(PbrBundle {
            //         mesh: my_assets.sphere.mesh.clone(),
            //         material: my_assets.sphere.material.clone(),
            //         transform: Transform::from_scale(Vec3::one() * 0.4),
            //         ..Default::default()
            //     })
            //     .spawn(PbrBundle {
            //         mesh: my_assets.sphere.mesh.clone(),
            //         material: my_assets.sphere.material.clone(),
            //         transform: Transform::from_scale(Vec3::one() * 0.6),
            //         ..Default::default()
            //     });
        });
        let particle = commands.current_entity().unwrap();
        commands
            .spawn(PbrBundle {
                mesh: my_assets.fake_shadow.mesh.clone(),
                material: my_assets.fake_shadow.material.clone(),
                transform: util::shadow_transform(position),
                ..Default::default()
            })
            .with(LeadingParticle(particle));
    }
}
