use bevy::{
    prelude::{Entity, Query, Res, Transform, With},
    utils::HashMap,
};
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};
use genesis_attributes::{EyeAngle, EyeRange};
use genesis_components::{mind::Mind, see::Vision, time::AgeEfficiency, Meat, Plant};
use genesis_maths::{
    angle_difference, average_angle, cast_angles, point_from_angle, quat_to_angle,
};

#[derive(Default)]
struct RayData {
    toi: Vec<f32>,
    angles: Vec<f32>,
}

impl RayData {
    fn push_data(&mut self, data: (f32, f32)) {
        self.toi.push(data.0);
        self.angles.push(data.1);
    }
    fn score(&self, range: f32) -> (f32, f32) {
        let dist_score = self.average_toi() / range;
        (dist_score, self.average_angle())
    }

    fn average_toi(&self) -> f32 {
        self.toi.iter().sum::<f32>() / self.toi.len() as f32
    }

    fn average_angle(&self) -> f32 {
        average_angle(&self.angles)
    }
}

#[derive(Default)]
struct RayHits {
    hits: HashMap<Entity, RayData>,
}

impl RayHits {
    fn add_entity(&mut self, entity: Entity, data: (f32, f32)) {
        match self.hits.get_mut(&entity) {
            Some(x) => x.push_data(data),
            None => {
                self.hits.insert(
                    entity,
                    RayData {
                        toi: vec![data.0],
                        angles: vec![data.1],
                    },
                );
            }
        }
    }
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
    bug_query: Query<&Mind>,
    plant_query: Query<Entity, With<Plant>>,
    meat_query: Query<Entity, With<Meat>>,
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

        let mut cast_hits = RayHits::default();
        for angle in angles {
            let ray_dir = point_from_angle(angle);
            if let Some(hit) = rapier_context.cast_ray(ray_pos, ray_dir, range, SOLID, filter) {
                let new_angle = angle_difference(eye_angle_relative_to_y, angle);
                cast_hits.add_entity(hit.0, (hit.1, new_angle))
            }
        }

        vision.reset();

        let mut visible_bugs = vec![];
        let mut visible_plants = vec![];
        let mut visible_meat = vec![];
        for (entity, ray_data) in cast_hits.hits.iter() {
            if let Ok(bug_mind) = bug_query.get(*entity) {
                vision.increment_bugs();
                visible_bugs.push((ray_data, bug_mind));
                continue;
            };
            if plant_query.get(*entity).is_ok() {
                vision.increment_plant();
                visible_plants.push(ray_data);
                continue;
            };
            if meat_query.get(*entity).is_ok() {
                vision.increment_meat();
                visible_meat.push(ray_data);
            };
        }
        let mut bug_index = usize::MAX;
        for (i, (result, _)) in visible_bugs.iter().enumerate() {
            let score = result.score(range);
            if vision.bug_dist_score > score.0 {
                vision.bug_dist_score = score.0;
                vision.bug_angle_score = score.1;
                bug_index = i;
            }
        }
        if let Some((_, bug_mind)) = visible_bugs.get(bug_index) {
            vision.bug_species = mind.compare(bug_mind);
        }
        for result in &visible_plants {
            vision.set_plant_score(result.score(range))
        }
        for result in &visible_meat {
            vision.set_meat_score(result.score(range))
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
