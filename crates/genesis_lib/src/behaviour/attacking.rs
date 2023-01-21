use bevy::prelude::{Mut, Query, Res, ResMut, Transform};
use bevy_rapier2d::prelude::RapierContext;
use genesis_attributes::{BaseAttack, BaseDefence};
use genesis_components::{
    body::{HealthEfficiency, Vitality},
    mind::MindOutput,
    time::AgeEfficiency,
    Size,
};
use genesis_config::ATTACK_INDEX;
use genesis_ecosystem::Ecosystem;
use genesis_maths::angle_between;

pub type AttackingBug<'a> = (
    &'a Transform,
    &'a MindOutput,
    &'a BaseAttack,
    &'a Size,
    &'a AgeEfficiency,
    &'a HealthEfficiency,
);

fn attack_bug(
    bug: &AttackingBug,
    other: &mut (
        &Transform,
        Mut<Vitality>,
        &Size,
        &BaseDefence,
        &AgeEfficiency,
        &HealthEfficiency,
    ),
    ecosystem: &mut ResMut<Ecosystem>,
) {
    let (bug_transform, mind_out, base_attack, size, age_efficiency, health_efficiency) = bug;
    let (other_transform, vitality, other_size, base_defence, other_age, other_health) = other;
    let attack = mind_out[ATTACK_INDEX];
    if attack <= 0.0 {
        return;
    }

    let translation_between = other_transform.translation - bug_transform.translation;
    let angle_to_other = angle_between(&bug_transform.rotation, translation_between);

    if angle_to_other.abs() >= 0.2 {
        return;
    }

    let attack_strength = ***base_attack * attack * ***age_efficiency * ***health_efficiency;
    let defence_absorption = ***base_defence * ***other_age * ***other_health;

    let size_multiplier = if ***size > ***other_size {
        ***other_size / ***size
    } else {
        1.0
    };
    let health_impact = attack_strength * defence_absorption.mul_add(-size_multiplier, 1.0);
    let lost_energy = vitality
        .health_mut()
        .take_energy(health_impact.ceil() as usize);
    ecosystem.return_energy(lost_energy);
}

pub fn attacking_system(
    rapier_context: Res<RapierContext>,
    mut ecosystem: ResMut<Ecosystem>,
    bug_query: Query<AttackingBug>,
    mut other_query: Query<(
        &Transform,
        &mut Vitality,
        &Size,
        &BaseDefence,
        &AgeEfficiency,
        &HealthEfficiency,
    )>,
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
