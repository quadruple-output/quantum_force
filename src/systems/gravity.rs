use crate::components::{Force, Mass};
use bevy::prelude::*;

/*
pub fn apply(
    entity: Entity,
    mass: &Mass,
    transform: &Transform,
    force: &mut Force,
    query: Query<(Entity, &Mass, &Transform)>,
) {
    for (other_entity, other_mass, other_transform) in query.iter() {
        if other_entity != entity {
            let vec_to_other = other_transform.translation - transform.translation;
            let distance = vec_to_other.length();
            let direction = vec_to_other.normalize();
            force.0 += direction * (mass + other_mass) / (distance * distance);
        }
    }
} */

pub fn calculate_forces(
    mut query: Query<(Entity, &Mass, &Transform, &mut Force)>,
    query_others: Query<(Entity, &Mass, &Transform)>,
) {
    query
        .iter_mut()
        .for_each(|(entity, &mass, transform, mut force)| {
            query_others
                .iter()
                .for_each(|(other_entity, &other_mass, other_transform)| {
                    if other_entity != entity {
                        let vec_to_other = other_transform.translation - transform.translation;
                        let distance = vec_to_other.length();
                        let direction = vec_to_other.normalize();
                        *force +=
                            Force(direction * (mass.0 * other_mass.0) / (distance * distance));
                    }
                })
        })
}
