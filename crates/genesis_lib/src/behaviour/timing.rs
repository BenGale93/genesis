use bevy::{
    prelude::{Query, Res, ResMut},
    time::Time,
};

use crate::{
    attributes,
    components::{mind::MindOutput, time::*},
    config,
};

pub fn progress_age_system(time: Res<Time>, mut query: Query<&mut Age>) {
    for mut age in query.iter_mut() {
        age.tick(time.delta());
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

pub fn reset_internal_timer_system(
    mut query: Query<(
        &mut InternalTimer,
        &MindOutput,
        &attributes::InternalTimerBoundary,
    )>,
) {
    for (mut internal_timer, mind_out, boundary) in query.iter_mut() {
        if mind_out[config::RESET_TIMER_INDEX] > **boundary {
            internal_timer.reset();
        }
    }
}
