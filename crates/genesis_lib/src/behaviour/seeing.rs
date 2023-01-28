use bevy::prelude::{Query, Transform, With};
use genesis_attributes::{EyeAngle, EyeRange};
use genesis_components::{mind::Mind, see::Vision, time::AgeEfficiency, Meat, Plant};
use genesis_maths::Cone;

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
            if let Some(bug_score) = cone.vision_scores(bug_transform.translation) {
                vision.increment_bugs();

                if vision.bug_dist_score > bug_score.0 {
                    vision.bug_dist_score = bug_score.0;
                    vision.bug_angle_score = bug_score.1;
                    vision.bug_species = mind.compare(bug_mind);
                }
            }
        }

        for plant_transform in plant_query.iter() {
            if let Some(scores) = cone.vision_scores(plant_transform.translation) {
                vision.increment_plant();
                vision.set_plant_score(scores);
            }
        }

        for meat_transform in meat_query.iter() {
            if let Some(scores) = cone.vision_scores(meat_transform.translation) {
                vision.increment_meat();
                vision.set_meat_score(scores);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use bevy::prelude::{App, Transform};
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
}
