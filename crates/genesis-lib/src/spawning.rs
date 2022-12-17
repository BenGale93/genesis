use std::f32::consts::PI;

use anyhow::anyhow;
use bevy::{
    prelude::{
        default, AssetServer, Color, Commands, Deref, DerefMut, DespawnRecursiveExt, Entity, Query,
        Res, ResMut, Resource, Transform, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, Damping, RigidBody, Velocity};
use genesis_util::maths::polars_to_cart;
use rand::{self, rngs::ThreadRng, Rng};
use rand_distr::{Distribution, Gamma, InverseGaussian, LogNormal, Normal, Uniform};
use serde_derive::{Deserialize, Serialize};

use crate::{
    behaviour::{eating::Eaten, lifecycle::Generation},
    body, config, ecosystem,
};

pub enum DistributionKind {
    Gamma(Gamma<f32>),
    Normal(Normal<f32>),
    Uniform(Uniform<f32>),
    LogNormal(LogNormal<f32>),
    InverseGaussian(InverseGaussian<f32>),
}

impl DistributionKind {
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
        match self {
            Self::Gamma(d) => d.sample(rng),
            Self::Normal(d) => d.sample(rng),
            Self::Uniform(d) => d.sample(rng),
            Self::LogNormal(d) => d.sample(rng),
            Self::InverseGaussian(d) => d.sample(rng),
        }
    }

    pub fn from_config(config: &DistributionConfig) -> anyhow::Result<Self> {
        let dist = match config.name.as_str() {
            "gamma" => Self::Gamma(Gamma::new(config.a, config.b)?),
            "normal" => Self::Normal(Normal::new(config.a, config.b)?),
            "uniform" => Self::Uniform(Uniform::new(config.a, config.b)),
            "lognormal" => Self::LogNormal(LogNormal::new(config.a, config.b)?),
            "inversegaussian" => Self::InverseGaussian(InverseGaussian::new(config.a, config.b)?),
            _ => return Err(anyhow!("Unknown distribution.")),
        };
        Ok(dist)
    }
}

pub struct Spawner {
    centre: Vec3,
    radius: f32,
    dist: DistributionKind,
    nearby_organisms: usize,
    nearby_food: usize,
}

impl Spawner {
    pub const fn new(centre: Vec3, radius: f32, dist: DistributionKind) -> Self {
        Self {
            centre,
            radius,
            dist,
            nearby_organisms: 0,
            nearby_food: 0,
        }
    }

    pub fn random_position(&self, rng: &mut ThreadRng) -> Vec3 {
        let r = self.dist.sample(rng) * self.radius;
        let theta = rng.gen_range(-PI..PI);
        let (x, y) = polars_to_cart(r, theta);
        Vec3::new(x, y, 0.0) + self.centre
    }

    pub fn from_config(config: &SpawnerConfig) -> anyhow::Result<Self> {
        let (x, y) = config.centre;
        let centre = Vec3::new(x, y, 0.0);
        let dist = DistributionKind::from_config(&config.dist)?;

        Ok(Self::new(centre, config.radius, dist))
    }

    pub const fn nearby_organisms(&self) -> usize {
        self.nearby_organisms
    }

    pub fn set_nearby_organisms(&mut self, nearby_bugs: usize) {
        self.nearby_organisms = nearby_bugs;
    }

    pub const fn nearby_food(&self) -> usize {
        self.nearby_food
    }

    pub fn set_nearby_food(&mut self, nearby_food: usize) {
        self.nearby_food = nearby_food;
    }

    pub fn distance(&self, position: &Vec3) -> f32 {
        self.centre.distance(*position)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct Spawners(Vec<Spawner>);

impl Spawners {
    pub fn random_organism_position(&self, rng: &mut ThreadRng) -> Vec3 {
        let index = self
            .nearby_organisms()
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        let spawner = &self.0[index];
        spawner.random_position(rng)
    }

    pub fn random_food_position(&self, rng: &mut ThreadRng) -> Vec3 {
        let index = self
            .nearby_food()
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        let spawner = &self.0[index];
        spawner.random_position(rng)
    }

    pub fn from_configs(configs: &[SpawnerConfig]) -> anyhow::Result<Self> {
        let mut result = vec![];
        for config in configs {
            result.push(Spawner::from_config(config)?);
        }
        Ok(Self(result))
    }

    pub fn nearby_organisms(&self) -> Vec<usize> {
        self.0.iter().map(Spawner::nearby_organisms).collect()
    }

    pub fn nearby_food(&self) -> Vec<usize> {
        self.0.iter().map(Spawner::nearby_food).collect()
    }

    pub fn space_for_organisms(&self, min_number: usize) -> bool {
        self.iter().any(|s| s.nearby_organisms() < min_number)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionConfig {
    name: String,
    a: f32,
    b: f32,
}

impl DistributionConfig {
    pub const fn new(name: String, a: f32, b: f32) -> Self {
        Self { name, a, b }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnerConfig {
    centre: (f32, f32),
    radius: f32,
    dist: DistributionConfig,
}

impl SpawnerConfig {
    pub const fn new(centre: (f32, f32), radius: f32, dist: DistributionConfig) -> Self {
        Self {
            centre,
            radius,
            dist,
        }
    }
}

pub fn nearest_spawner_system(
    mut spawners: ResMut<Spawners>,
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
    let mut food_counts = vec![0; spawners.0.len()];
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
    spawners: Res<Spawners>,
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
