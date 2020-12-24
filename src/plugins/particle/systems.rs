use super::{
    components::{LeadingParticle, Spin},
    util,
};
use bevy::prelude::*;
use core::f32::consts::PI;

#[allow(clippy::type_complexity)]
pub fn adjust_shadow(
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
    assert!(!map_of_particle_positions.is_empty());
    qs.q2_mut()
        .iter_mut()
        .for_each(|(leading_particle, mut shadow_transform)| {
            let &particle_translation = map_of_particle_positions.get(&leading_particle.0).unwrap();
            *shadow_transform = util::shadow_transform(particle_translation);
        });
}

pub fn animate_spin(time: Res<Time>, mut query: Query<(&Spin, &mut Transform)>) {
    query
        .iter_mut()
        .for_each(|(spin, mut transform)| match spin {
            Spin::Up => transform.rotate(Quat::from_rotation_y(
                time.delta_seconds() as f32 * 4.0 * PI,
            )),
            Spin::Down => transform.rotate(Quat::from_rotation_y(
                time.delta_seconds() as f32 * 4.0 * -PI,
            )),
        })
}
