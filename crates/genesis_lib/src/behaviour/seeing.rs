use std::f32::consts::PI;

use bevy::prelude::{Query, Transform, With};
use genesis_attributes::{EyeAngle, EyeRange};
use genesis_components::{mind::Mind, see::Vision, time::AgeEfficiency, Meat, Plant};
use genesis_maths::{angle_between, Cone};

fn dist_angle_score(
    transform: &Transform,
    target_transform: &Transform,
    eye_range: f32,
) -> (f32, f32) {
    let dist = target_transform.translation - transform.translation;
    let dist_score = dist.length() / eye_range;
    let angle = angle_between(&transform.rotation, dist);
    let angle_score = angle / PI;
    (dist_score, angle_score)
}

pub fn process_sight_system(
    mut eye_query: Query<(
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
    for (eye_range, eye_angle, transform, mind, mut vision, age_efficiency) in eye_query.iter_mut()
    {
        let range = **eye_range * **age_efficiency;
        let cone = Cone::new(
            transform.translation,
            transform.rotation,
            **eye_angle,
            range,
        )
        .unwrap();

        vision.reset();

        for (bug_transform, bug_mind) in bug_query.iter() {
            if transform.translation == bug_transform.translation {
                continue;
            }
            if cone.is_within_cone(bug_transform.translation) {
                vision.increment_bugs();

                let bug_score = dist_angle_score(transform, bug_transform, range);
                if vision.bug_dist_score > bug_score.0 {
                    vision.bug_dist_score = bug_score.0;
                    vision.bug_angle_score = bug_score.1;
                    vision.bug_species = mind.compare(bug_mind);
                }
            }
        }

        for plant_transform in plant_query.iter() {
            if cone.is_within_cone(plant_transform.translation) {
                vision.increment_plant();

                let scores = dist_angle_score(transform, plant_transform, range);
                vision.set_plant_score(scores);
            }
        }

        for meat_transform in meat_query.iter() {
            if cone.is_within_cone(meat_transform.translation) {
                vision.increment_meat();

                let scores = dist_angle_score(transform, meat_transform, range);
                vision.set_meat_score(scores);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use bevy::prelude::{App, Quat, Transform};
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

    #[test]
    fn angle_score_straight_ahead() {
        let bug = Transform::from_xyz(0.0, 0.0, 0.0);
        let target = Transform::from_xyz(0.0, 1.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert_eq!(angle_score, 0.0);
    }

    #[test]
    fn angle_score_straight_behind() {
        let bug = Transform::from_xyz(0.0, 0.0, 0.0);
        let target = Transform::from_xyz(0.0, -1.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert!((-1.0 - angle_score).abs() < 0.0001);
    }

    #[test]
    fn angle_score_to_the_left() {
        let bug = Transform::from_xyz(0.0, 0.0, 0.0);
        let target = Transform::from_xyz(-1.0, 0.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert!((0.5 - angle_score).abs() < 0.0001);
    }

    #[test]
    fn angle_score_to_the_right() {
        let bug = Transform::from_xyz(0.0, 0.0, 0.0);
        let target = Transform::from_xyz(1.0, 0.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert!(((-0.5) - angle_score).abs() < 0.0001);
    }

    #[test]
    fn angle_score_offset_bug() {
        let bug = Transform::from_rotation(Quat::from_rotation_z(f32::to_radians(-45.0)));
        let target = Transform::from_xyz(-1.0, 1.0, 0.0);

        let (_, angle_score) = dist_angle_score(&bug, &target, 20.0);

        assert!(((0.5) - angle_score).abs() < 0.0001);
    }
}
