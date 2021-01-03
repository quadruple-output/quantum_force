use super::super::components::{Force, Repulsor};
use crate::plugins::shapes::{
    tools::{transform_to_isometry, vec3_from_point, vec3_to_point},
    NcollideShape,
};
use bevy::prelude::*;

pub fn calculate_forces(
    mut query_particles: Query<(Entity, &mut Force, &Transform)>,
    query_repulsors: Query<(Entity, &NcollideShape, &Repulsor, &Transform)>,
) {
    query_particles
        .iter_mut()
        .for_each(|(particle_entity, mut force, particle_transform)| {
            query_repulsors
                .iter()
                .for_each(|(shape_entity, shape, repulsor, shape_transform)| {
                    if shape_entity != particle_entity {
                        let projection = shape.0.as_point_query().unwrap().project_point(
                            &transform_to_isometry(shape_transform),
                            &vec3_to_point(particle_transform.translation),
                            false,
                        );
                        if projection.is_inside {
                            todo!();
                        } else {
                            let v_shape_to_particle: Vec3 =
                                particle_transform.translation - vec3_from_point(projection.point);
                            let distance = v_shape_to_particle.length();
                            let abs_force = 1. / ((distance / repulsor.range).exp() - 1.);
                            // if abs_force > 1. {
                            //     dbg!(distance);
                            //     dbg!(abs_force);
                            //     //dbg!(projection.point);
                            // }
                            *force += Force(v_shape_to_particle.normalize() * abs_force);
                        }
                    }
                });
        });
}
