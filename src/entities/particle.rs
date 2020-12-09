use crate::components::{Force, Mass, Velocity};
use bevy::prelude::*;

#[derive(Clone, Default)]
pub struct Particle {
    mass: Option<f32>,
    velocity: Vec3,
}

impl Particle {
    pub fn new() -> Self {
        Self {
            mass: None,
            velocity: Vec3::default(),
        }
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
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        const SUBDIVISIONS: usize = 5;
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
                .spawn(PbrComponents {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.2,
                        subdivisions: SUBDIVISIONS,
                    })),
                    material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.3).into()),
                    ..Default::default()
                })
                .spawn(PbrComponents {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.4,
                        subdivisions: SUBDIVISIONS,
                    })),
                    material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.3).into()),
                    ..Default::default()
                })
                .spawn(PbrComponents {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.6,
                        subdivisions: SUBDIVISIONS,
                    })),
                    material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.3).into()),
                    ..Default::default()
                });
        });
    }
}
