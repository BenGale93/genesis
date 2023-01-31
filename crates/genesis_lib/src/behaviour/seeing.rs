use bevy::{
    prelude::{Entity, Query, Res, Transform, With},
    utils::HashSet,
};
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};
use genesis_attributes::{EyeAngle, EyeRange};
use genesis_components::{mind::Mind, see::Vision, time::AgeEfficiency, Meat, Plant};
use genesis_maths::{angle_between, cast_angles, point_from_angle, quat_to_angle};

fn dist_angle_score(
    transform: &Transform,
    target_transform: &Transform,
    eye_range: f32,
) -> (f32, f32) {
    let dist = target_transform.translation - transform.translation;
    let dist_score = dist.length() / eye_range;
    let angle = angle_between(&transform.rotation, dist);
    (dist_score, angle)
}

pub fn process_sight_system(
    rapier_context: Res<RapierContext>,
    mut eye_query: Query<(
        Entity,
        &EyeRange,
        &EyeAngle,
        &Transform,
        &Mind,
        &mut Vision,
        &AgeEfficiency,
    )>,
    bug_query: Query<(&Transform, &Mind)>,
    plant_query: Query<&Transform, With<Plant>>,
    meat_query: Query<&Transform, With<Meat>>,
) {
    const SOLID: bool = false;
    const FREQ: usize = 20;
    for (entity, eye_range, eye_angle, transform, mind, mut vision, age_efficiency) in
        eye_query.iter_mut()
    {
        let range = **eye_range * **age_efficiency;
        let filter = QueryFilter::new().exclude_collider(entity);
        let ray_pos = transform.translation.truncate();

        let eye_angle_relative_to_y = quat_to_angle(&transform.rotation);
        let angles = cast_angles(eye_angle_relative_to_y, **eye_angle, FREQ);

        let mut cast_hits = HashSet::new();
        for angle in angles {
            let ray_dir = point_from_angle(angle);
            if let Some(hit) = rapier_context.cast_ray(ray_pos, ray_dir, range, SOLID, filter) {
                cast_hits.insert(hit.0);
            }
        }

        vision.reset();

        let mut visible_bugs = vec![];
        for entity in cast_hits.iter() {
            if let Ok(plant_transform) = plant_query.get(*entity) {
                vision.increment_plant();
                let scores = dist_angle_score(transform, plant_transform, range);
                vision.set_plant_score(scores);
                continue;
            };
            if let Ok(bug_data) = bug_query.get(*entity) {
                vision.increment_bugs();
                visible_bugs.push(bug_data);
                continue;
            };
            if let Ok(meat_transform) = meat_query.get(*entity) {
                vision.increment_meat();
                let scores = dist_angle_score(transform, meat_transform, range);
                vision.set_meat_score(scores)
            };
        }
        let mut bug_index = usize::MAX;
        for (i, (bug_transform, _)) in visible_bugs.iter().enumerate() {
            let scores = dist_angle_score(transform, bug_transform, range);
            if vision.bug_dist_score > scores.0 {
                vision.bug_dist_score = scores.0;
                vision.bug_angle_score = scores.1;
                bug_index = i;
            }
        }
        if let Some((_, bug_mind)) = visible_bugs.get(bug_index) {
            vision.bug_species = mind.compare(bug_mind);
        }
    }
}

#[cfg(test)]
mod tests {

    use bevy::{
        prelude::{App, Transform},
        time::TimePlugin,
    };
    use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
    use genesis_attributes::Genome;
    use genesis_config::{initialize_configs, BEHAVIOUR_TICK, INPUT_NEURONS, OUTPUT_NEURONS};
    use iyes_loopless::prelude::*;
    use rand::{rngs::StdRng, SeedableRng};
    use rand_distr::{Distribution, Uniform};
    use test::Bencher;

    use super::*;

    #[bench]
    fn bench_sight_system(b: &mut Bencher) {
        initialize_configs(None);
        let mut rng = StdRng::seed_from_u64(2);
        let uniform = Uniform::new(-500.0, 500.0);
        let mut app = App::new();
        app.init_resource::<FixedTimesteps>()
            .add_plugin(TimePlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0))
            .add_fixed_timestep(BEHAVIOUR_TICK, "standard");

        app.add_system(process_sight_system);

        for _ in 0..100 {
            let transform =
                Transform::from_xyz(uniform.sample(&mut rng), uniform.sample(&mut rng), 0.0);
            let plant = (transform, Plant);
            app.world.spawn(plant);
        }

        let genome = Genome::new();
        let mind = Mind::minimal(INPUT_NEURONS, OUTPUT_NEURONS, &[]);

        for _ in 0..100 {
            let transform =
                Transform::from_xyz(uniform.sample(&mut rng), uniform.sample(&mut rng), 0.0);
            let eye_range = EyeRange::new(400.0);
            let eye_angle = EyeAngle::new(*eye_range, &genome.eye_range);
            let bug = (
                transform,
                eye_range,
                eye_angle,
                mind.clone(),
                Vision::new(),
                AgeEfficiency(1.0),
            );
            app.world.spawn(bug);
        }

        b.iter(|| {
            for _ in 0..100 {
                app.update()
            }
        });
    }
}
