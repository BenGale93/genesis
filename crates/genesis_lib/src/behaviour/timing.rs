use bevy::{
    prelude::{Query, Res, ResMut},
    time::Time,
};
use genesis_attributes::DeathAge;
use genesis_components::{mind::MindOutput, time::*};
use genesis_config as config;

pub fn progress_age_system(
    time: Res<Time>,
    mut query: Query<(&mut Age, &mut AgeEfficiency, &DeathAge)>,
) {
    for (mut age, mut age_efficiency, death_age) in query.iter_mut() {
        age.tick(time.delta());
        age_efficiency.update(&age, death_age);
    }
}

pub fn progress_timers_system(time: Res<Time>, mut query: Query<(&mut Heart, &mut InternalTimer)>) {
    for (mut heart, mut internal_timer) in query.iter_mut() {
        heart.tick(time.delta());
        internal_timer.tick(time.delta());
    }
}

pub fn progress_simulation_timer(time: Res<Time>, mut simulation_timer: ResMut<SimulationTime>) {
    simulation_timer.tick(time.delta());
}

pub fn reset_internal_timer_system(mut query: Query<(&mut InternalTimer, &MindOutput)>) {
    for (mut internal_timer, mind_out) in query.iter_mut() {
        if mind_out[config::RESET_TIMER_INDEX] >= 0.0 {
            internal_timer.reset();
        }
    }
}
