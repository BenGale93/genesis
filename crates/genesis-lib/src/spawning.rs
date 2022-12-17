use bevy::{
    prelude::{
        default, AssetServer, Color, Commands, DespawnRecursiveExt, Entity, Query, Res, ResMut,
        Resource, Transform, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, Damping, RigidBody, Velocity};
use rand_distr::{Distribution, Uniform};

use crate::{
    behaviour::{eating::Eaten, lifecycle::Generation},
    body, config, ecosystem,
};

pub fn nearest_spawner_system(
    mut spawners: ResMut<spawners::Spawners>,
    organisms: Query<&Transform, With<Generation>>,
    plants: Query<(&Transform, &ecosystem::Plant)>,
) {
    let mut organism_counts = vec![0; spawners.len()];
    for position in organisms.iter() {
        let distances: Vec<f32> = spawners
            .iter()
            .map(|s| s.distance(&position.translation))
            .collect();
        let index = distances
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        organism_counts[index] += 1;
    }
    for (i, spawner) in spawners.iter_mut().enumerate() {
        spawner.set_nearby_organisms(organism_counts[i]);
    }
    let mut food_counts = vec![0; spawners.len()];
    for (transform, plant) in plants.iter() {
        let distances: Vec<f32> = spawners
            .iter()
            .map(|s| s.distance(&transform.translation))
            .collect();
        let index = distances
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        food_counts[index] += plant.energy().amount();
    }
    for (i, spawner) in spawners.iter_mut().enumerate() {
        spawner.set_nearby_food(food_counts[i]);
    }
}

fn spawn_plant(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    energy: ecosystem::Energy,
    location: Vec3,
) {
    let original_color = body::OriginalColor(Color::GREEN);
    let plant = ecosystem::Plant::new(energy);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("food.png"),
            sprite: Sprite {
                custom_size: plant.sprite_size(),
                color: original_color.0,
                ..default()
            },
            ..default()
        })
        .insert(original_color)
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(TransformBundle::from(Transform::from_translation(location)))
        .insert(plant.collider())
        .insert(Velocity::zero())
        .insert(plant);
}

#[derive(Resource)]
pub struct PlantSizeRandomiser(Uniform<f32>);

impl PlantSizeRandomiser {
    pub fn new(bounds: (f32, f32)) -> Self {
        Self(Uniform::new(bounds.0, bounds.1))
    }
    pub fn random_size(&self, rng: &mut rand::rngs::ThreadRng) -> f32 {
        self.0.sample(rng)
    }
}

pub fn spawn_plant_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    spawners: Res<spawners::Spawners>,
    plant_size_randomiser: Res<PlantSizeRandomiser>,
) {
    let config_instance = config::WorldConfig::global();
    let available_energy = ecosystem.available_energy().amount();

    if available_energy > (config_instance.start_num * config_instance.start_energy) {
        let mut rng = rand::thread_rng();
        let size = plant_size_randomiser.random_size(&mut rng);
        let Some(energy) =
            ecosystem.request_energy(size as usize * config_instance.plant_energy_per_unit) else {return};
        let location = spawners.random_food_position(&mut rng);
        spawn_plant(&mut commands, asset_server, energy, location);
    }
}

pub fn update_plant_size(mut plant_query: Query<(&mut Sprite, &mut Collider, &ecosystem::Plant)>) {
    // Might be able to improve this using bevy events.
    // Basically listen for changes to plants and only then update.
    for (mut sprite, mut collider, plant) in plant_query.iter_mut() {
        sprite.custom_size = plant.sprite_size();
        *collider = plant.collider();
    }
}

pub fn despawn_plants_system(mut commands: Commands, plant_query: Query<Entity, With<Eaten>>) {
    for entity in plant_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
