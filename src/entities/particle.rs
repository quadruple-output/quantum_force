use crate::components::{Acceleration, Force, Mass, Spin, Velocity};
use bevy::prelude::*;

pub struct ParticleAssets {
    quant: QuantAssets,
    sphere: MeshMat,
    fake_shadow: MeshMat,
}

struct MeshMat {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

struct QuantAssets {
    mesh: Handle<Mesh>,
    material_weight: Handle<StandardMaterial>,
    material_inertia: Handle<StandardMaterial>,
}
pub struct LeadingParticle(Entity);

#[derive(Copy, Clone)]
pub struct Particle {
    spin: Spin,
    mass: Option<f32>,
    velocity: Vec3,
}

impl FromResources for ParticleAssets {
    fn from_resources(resources: &Resources) -> Self {
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();

        Self {
            quant: QuantAssets {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 1.0, // scaled by consumer
                    subdivisions: 1,
                })),
                material_weight: materials.add(Color::RED.into()),
                material_inertia: materials.add(Color::PINK.into()),
            },
            sphere: MeshMat {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 1.0, // scaled by consumer
                    subdivisions: 4,
                })),
                material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.3).into()),
            },
            fake_shadow: MeshMat {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 1.0, // scaled by consumer
                    subdivisions: 2,
                })),
                material: materials.add(Color::rgba(0.3, 0.5, 0.3, 1.0).into()),
            },
        }
    }
}

impl Particle {
    pub fn new() -> Self {
        Self {
            spin: Spin::Up,
            mass: None,
            velocity: Vec3::default(),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn animate(
        mut qs: QuerySet<(
            Query<&LeadingParticle>,
            Query<&Transform>,
            Query<(&LeadingParticle, &mut Transform)>,
        )>,
    ) {
        // cannot read and change Transforms at the same time.  So we build a map of
        // the Transforms we need per Entity, and adjust the Entity's position in a
        // second step:
        let map_of_particle_positions = qs
            .q0()
            .iter()
            .map(|leading_particle| {
                let particle_position = qs.q1().get(leading_particle.0).unwrap().translation;
                (leading_particle.0, particle_position)
            })
            .collect::<std::collections::BTreeMap<_, _>>();
        qs.q2_mut()
            .iter_mut()
            .for_each(|(leading_particle, mut shadow_transform)| {
                let &particle_translation =
                    map_of_particle_positions.get(&leading_particle.0).unwrap();
                *shadow_transform = Self::shadow_transform(particle_translation);
            });
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
            .spawn(PbrBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .with(Force::default())
            .with(Acceleration::default())
            .with(Velocity(self.velocity));
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
                transform: Self::shadow_transform(position),
                ..Default::default()
            })
            .with(LeadingParticle(particle));
    }

    fn shadow_transform(particle_translation: Vec3) -> Transform {
        let xz_position = Vec3::new(particle_translation.x, 0.01, particle_translation.z);
        Transform::from_translation(xz_position) * Transform::from_scale(Vec3::new(0.2, 0.0, 0.2))
    }
}
