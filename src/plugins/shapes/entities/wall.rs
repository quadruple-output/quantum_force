use super::super::components::NcollideShape;
use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use ncollide3d::{math::Vector, procedural::IndexBuffer, shape::Cuboid, transformation::ToTriMesh};
use std::marker::PhantomData;

pub trait BuilderOption {}
pub trait BuilderOptionDefined: BuilderOption {}
pub trait BuilderOptionUndefined: BuilderOption {}
#[derive(Copy, Clone, Default)]
pub struct Defined;
#[derive(Copy, Clone, Default)]
pub struct Undefined;

impl BuilderOption for Defined {}
impl BuilderOption for Undefined {}
impl BuilderOptionDefined for Defined {}
impl BuilderOptionUndefined for Undefined {}

pub fn build_wall() -> WallBuilder<Undefined, Undefined> {
    WallBuilder {
        facing: Vec3::unit_z(),
        up: Vec3::unit_y(),
        ..Default::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct WallBuilder<DIMENSIONS, MATERIAL>
where
    DIMENSIONS: BuilderOption,
{
    center: Vec3,
    ncollide_shape: Option<Cuboid<f32>>,
    mesh: Option<Handle<Mesh>>,
    dimensions_defined: PhantomData<DIMENSIONS>,
    facing: Vec3,
    up: Vec3,
    material: Option<Handle<StandardMaterial>>,
    material_defined: PhantomData<MATERIAL>,
}

impl<DIMENSIONS, MATERIAL> WallBuilder<DIMENSIONS, MATERIAL>
where
    DIMENSIONS: BuilderOptionUndefined,
    MATERIAL: BuilderOption,
{
    pub fn with_dimensions(
        self,
        x: f32,
        y: f32,
        z: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> WallBuilder<Defined, MATERIAL> {
        let ncollide_shape = Cuboid::new(Vector::new(x / 2.0, y / 2.0, z / 2.0));
        let mut ncollide_mesh = ncollide_shape.to_trimesh(());
        ncollide_mesh.unify_index_buffer();

        let bevy_positions = ncollide_mesh
            .coords
            .iter()
            .map(|coord| [coord[0], coord[1], coord[2]])
            .collect::<Vec<_>>();
        let bevy_normals = ncollide_mesh
            .normals
            .unwrap()
            .iter()
            .map(|normal| [normal[0], normal[1], normal[2]])
            .collect::<Vec<_>>();
        let bevy_uvs = ncollide_mesh
            .uvs
            .unwrap()
            .iter()
            .map(|uvs| [uvs[0], uvs[1]])
            .collect::<Vec<_>>();
        let bevy_indices = match ncollide_mesh.indices {
            IndexBuffer::Unified(points) => Indices::U32(
                points
                    .iter()
                    .map(|point| vec![point[0], point[1], point[2]])
                    .flatten()
                    .collect(),
            ),
            _ => panic!(),
        };

        let mut bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        bevy_mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, bevy_positions);
        bevy_mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, bevy_normals);
        bevy_mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, bevy_uvs);
        bevy_mesh.set_indices(Some(bevy_indices));

        WallBuilder {
            ncollide_shape: Some(ncollide_shape),
            mesh: Some((*meshes).add(bevy_mesh)),
            dimensions_defined: PhantomData::<Defined>,
            center: self.center,
            facing: self.facing,
            up: self.up,
            material: self.material,
            material_defined: self.material_defined,
        }
    }
}

impl<DIMENSIONS, MATERIAL> WallBuilder<DIMENSIONS, MATERIAL>
where
    DIMENSIONS: BuilderOption,
    MATERIAL: BuilderOptionUndefined,
{
    pub fn with_material(
        self,
        material: Handle<StandardMaterial>,
    ) -> WallBuilder<DIMENSIONS, Defined> {
        WallBuilder {
            ncollide_shape: self.ncollide_shape,
            mesh: self.mesh,
            dimensions_defined: self.dimensions_defined,
            center: self.center,
            facing: self.facing,
            up: self.up,
            material: Some(material),
            material_defined: PhantomData::<Defined>,
        }
    }
}

impl<DIMENSIONS, MATERIAL> WallBuilder<DIMENSIONS, MATERIAL>
where
    DIMENSIONS: BuilderOption,
    MATERIAL: BuilderOption,
{
    pub fn center_at(mut self, center: Vec3) -> Self {
        self.center = center;
        self
    }

    pub fn facing(mut self, facing: Vec3, up: Vec3) -> Self {
        self.facing = facing;
        self.up = up;
        self
    }
}

impl<DIMENSIONS, MATERIAL> WallBuilder<DIMENSIONS, MATERIAL>
where
    DIMENSIONS: BuilderOptionDefined,
    MATERIAL: BuilderOptionDefined,
{
    pub fn spawn(self, commands: &mut Commands) {
        commands
            .spawn(PbrBundle {
                transform: Transform::from_translation(self.center)
                    .looking_at(self.center - self.facing, self.up),
                mesh: self.mesh.unwrap(),
                material: self.material.unwrap(),
                ..Default::default()
            })
            .with(NcollideShape(Box::new(self.ncollide_shape.unwrap())));
    }
}
