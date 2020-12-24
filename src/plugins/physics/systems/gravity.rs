use super::super::components::{Force, Mass};
use crate::common::resources::PausePhysics;
use bevy::prelude::*;

pub fn calculate_forces(
    pause: Res<PausePhysics>,
    mut query: Query<(Entity, &Mass, &Transform, &mut Force)>,
    query_others: Query<(Entity, &Mass, &Transform)>,
) {
    if !pause.0 {
        query
            .iter_mut()
            .for_each(|(entity, &mass, transform, mut force)| {
                query_others
                    .iter()
                    .for_each(|(other_entity, &other_mass, other_transform)| {
                        if other_entity != entity {
                            let vec_to_other = other_transform.translation - transform.translation;
                            let distance_squared = vec_to_other.length_squared();
                            let direction = vec_to_other.normalize();
                            *force += Force(
                                direction * (mass.0 * other_mass.0)
                                    / (0.05 + // cheating to keep forces relatively small
                                 distance_squared),
                            );
                        }
                    })
            })
    }
}
