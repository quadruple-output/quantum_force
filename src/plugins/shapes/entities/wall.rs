use super::super::components::NcollideShape;
use crate::{common::types::builder_option::*, plugins::shapes::tools::convert_ncollide_mesh};
use bevy::prelude::*;
use ncollide3d::{math::Vector, shape::Cuboid, transformation::ToTriMesh};
use std::marker::PhantomData;

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
    MATERIAL: BuilderOption,
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
        let ncollide_shape = Cuboid::new(Vector::new(0.5 * x, 0.5 * y, 0.5 * z));
        WallBuilder {
            ncollide_shape: Some(ncollide_shape),
            mesh: Some((*meshes).add(convert_ncollide_mesh(ncollide_shape.to_trimesh(())))),
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
