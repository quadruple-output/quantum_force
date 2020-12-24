use bevy::prelude::*;

pub struct ParticleAssets {
    pub quant: QuantAssets,
    pub sphere: MeshMat,
    pub fake_shadow: MeshMat,
}

pub struct MeshMat {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

pub struct QuantAssets {
    pub mesh: Handle<Mesh>,
    pub material_weight: Handle<StandardMaterial>,
    pub material_inertia: Handle<StandardMaterial>,
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
