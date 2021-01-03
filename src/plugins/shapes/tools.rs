use bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};
use ncollide3d::procedural::{IndexBuffer, TriMesh};

pub fn convert_ncollide_mesh(mut ncollide_mesh: TriMesh<f32>) -> Mesh {
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

    bevy_mesh
}
