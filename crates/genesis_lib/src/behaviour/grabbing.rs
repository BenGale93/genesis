use bevy::{
    prelude::{Commands, Entity, Mut, Query, Res, Transform, With, Without},
    time::Stopwatch,
};
use bevy_rapier2d::prelude::{ExternalImpulse, RapierContext};
use genesis_attributes as attributes;
use genesis_components::{body::HealthEfficiency, grab::*, mind, time::AgeEfficiency, Egg, Size};
use genesis_config as config;
use genesis_maths::angle_between;
use genesis_traits::BehaviourTracker;
use iyes_loopless::prelude::FixedTimesteps;

type GrabberTest<'a> = (Entity, &'a mind::MindOutput);

pub fn process_grabbers_system(
    mut commands: Commands,
    not_grabbing_query: Query<GrabberTest, (Without<Egg>, Without<TryingToGrab>)>,
    grabbing_query: Query<GrabberTest, With<TryingToGrab>>,
) {
    for (entity, mind_out) in not_grabbing_query.iter() {
        if mind_out[config::WANT_TO_GRAB_INDEX] >= 0.0 {
            commands
                .entity(entity)
                .insert(TryingToGrab(Stopwatch::new()));
        }
    }

    for (entity, mind_out) in grabbing_query.iter() {
        if mind_out[config::WANT_TO_GRAB_INDEX] < 0.0 {
            commands.entity(entity).remove::<TryingToGrab>();
        }
    }
}

pub fn attempted_to_grab_system(
    timesteps: Res<FixedTimesteps>,
    mut bug_query: Query<(&mut TryingToGrab, &mut GrabbingSum)>,
) {
    let grab_cost = config::WorldConfig::global().cost_of_grab;
    let standard = timesteps.get("standard").unwrap();

    for (mut trying_to_grab, mut grow_sum) in bug_query.iter_mut() {
        trying_to_grab.tick(standard.step);
        let time_spent = trying_to_grab.elapsed().as_secs_f32();
        if time_spent >= 1.0 {
            grow_sum.add_time(time_spent, grab_cost);
            trying_to_grab.reset();
        }
    }
}

pub type GrabbingBug<'a> = (
    &'a Transform,
    &'a attributes::GrabAngle,
    &'a attributes::GrabStrength,
    &'a AgeEfficiency,
    &'a HealthEfficiency,
);

fn apply_grab(bug: &GrabbingBug, other: &mut (&Transform, &Size, Mut<ExternalImpulse>)) {
    let (bug_transform, grab_angle, grab_strength, age_efficiency, health_efficiency) = bug;
    let (other_transform, size, ext_impulse) = other;
    if ***size < config::GRAB_SIZE_THRESHOLD {
        return;
    }
    let translation_between = other_transform.translation - bug_transform.translation;
    let angle_to_other = angle_between(&bug_transform.rotation, translation_between);
    if angle_to_other.abs() < ***grab_angle {
        ext_impulse.impulse = -***grab_strength
            * ***age_efficiency
            * ***health_efficiency
            * translation_between.normalize().truncate();
    }
}

pub fn grabbing_system(
    rapier_context: Res<RapierContext>,
    bug_query: Query<GrabbingBug, With<TryingToGrab>>,
    mut other_query: Query<(&Transform, &Size, &mut ExternalImpulse)>,
) {
    for contact_pair in rapier_context.contact_pairs() {
        if let (Ok(bug), Ok(mut other)) = (
            bug_query.get(contact_pair.collider1()),
            other_query.get_mut(contact_pair.collider2()),
        ) {
            apply_grab(&bug, &mut other);
        }
        if let (Ok(bug), Ok(mut other)) = (
            bug_query.get(contact_pair.collider2()),
            other_query.get_mut(contact_pair.collider1()),
        ) {
            apply_grab(&bug, &mut other);
        }
    }
}
