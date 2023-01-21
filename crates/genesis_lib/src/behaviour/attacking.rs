use bevy::prelude::{info, Mut, Query, Res, ResMut, Transform};
use bevy_rapier2d::prelude::RapierContext;
use genesis_components::{
    body::{HealthEfficiency, Vitality},
    mind::MindOutput,
    time::AgeEfficiency,
};
use genesis_config::ATTACK_INDEX;
use genesis_ecosystem::Ecosystem;
use genesis_maths::angle_between;

pub type AttackingBug<'a> = (
    &'a Transform,
    &'a MindOutput,
    &'a AgeEfficiency,
    &'a HealthEfficiency,
);

fn attack_bug(
    bug: &AttackingBug,
    other: &mut (&Transform, Mut<Vitality>),
    ecosystem: &mut ResMut<Ecosystem>,
) {
    let (bug_transform, mind_out, age_efficiency, health_efficiency) = bug;
    let (other_transform, vitality) = other;
    let translation_between = other_transform.translation - bug_transform.translation;
    let angle_to_other = angle_between(&bug_transform.rotation, translation_between);
    if angle_to_other.abs() < 0.2 {
        let attack = mind_out[ATTACK_INDEX];
        let attack_strength = (100.0 * attack * ***age_efficiency * ***health_efficiency) as usize;
        info!(attack_strength);
        let lost_energy = vitality.health_mut().take_energy(attack_strength);
        ecosystem.return_energy(lost_energy);
    }
}

pub fn attacking_system(
    rapier_context: Res<RapierContext>,
    mut ecosystem: ResMut<Ecosystem>,
    bug_query: Query<AttackingBug>,
    mut other_query: Query<(&Transform, &mut Vitality)>,
) {
    for contact_pair in rapier_context.contact_pairs() {
        if let (Ok(bug), Ok(mut other)) = (
            bug_query.get(contact_pair.collider1()),
            other_query.get_mut(contact_pair.collider2()),
        ) {
            attack_bug(&bug, &mut other, &mut ecosystem);
        }
        if let (Ok(bug), Ok(mut other)) = (
            bug_query.get(contact_pair.collider2()),
            other_query.get_mut(contact_pair.collider1()),
        ) {
            attack_bug(&bug, &mut other, &mut ecosystem);
        }
    }
}
