use bevy::prelude::{Query, ResMut};
use genesis_components::{body, BurntEnergy};
use genesis_ecosystem as ecosystem;
use genesis_traits::BehaviourTracker;

pub fn energy_return_system(
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    mut query: Query<(
        &mut body::Vitality,
        &mut dyn BehaviourTracker,
        &mut BurntEnergy,
    )>,
) {
    for (mut vitality, mut trackers, mut burnt_energy) in query.iter_mut() {
        for mut tracker in trackers.iter_mut() {
            let cost = tracker.uint_portion();
            if cost >= 1 {
                burnt_energy.add_energy(vitality.take_energy(cost));
            }
        }

        ecosystem.return_energy(burnt_energy.return_energy());
    }
}

pub fn update_health_efficiency_system(
    mut bug_query: Query<(&mut body::HealthEfficiency, &body::Vitality)>,
) {
    for (mut health_efficiency, vitality) in bug_query.iter_mut() {
        health_efficiency.update(vitality.health())
    }
}
