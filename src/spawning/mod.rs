use std::f32::consts::PI;

use anyhow::anyhow;
use bevy::prelude::{Resource, Vec3};
use genesis_util::maths::polars_to_cart;
use rand::{self, rngs::ThreadRng, seq::SliceRandom, Rng};
use rand_distr::*;
use serde_derive::{Deserialize, Serialize};

pub enum DistributionKind {
    Gamma(Gamma<f32>),
    Normal(Normal<f32>),
}

impl DistributionKind {
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
        match self {
            DistributionKind::Gamma(g) => g.sample(rng),
            DistributionKind::Normal(g) => g.sample(rng),
        }
    }

    pub fn from_config(config: &DistributionConfig) -> anyhow::Result<Self> {
        let dist = match config.name.as_str() {
            "gamma" => DistributionKind::Gamma(Gamma::new(config.a, config.b)?),
            "normal" => DistributionKind::Normal(Normal::new(config.a, config.b)?),
            _ => return Err(anyhow!("Unknown distribution.")),
        };
        Ok(dist)
    }
}

pub struct Spawner {
    centre: Vec3,
    radius: f32,
    dist: DistributionKind,
}

impl Spawner {
    pub fn new(centre: Vec3, radius: f32, dist: DistributionKind) -> Self {
        Self {
            centre,
            radius,
            dist,
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
}

#[derive(Resource)]
pub struct Spawners(Vec<Spawner>);

impl Spawners {
    pub fn random_position(&self, rng: &mut ThreadRng) -> Vec3 {
        let spawner = self.0.choose(rng).unwrap();
        spawner.random_position(rng)
    }

    pub fn from_configs(configs: &[SpawnerConfig]) -> anyhow::Result<Self> {
        let mut result = vec![];
        for config in configs {
            result.push(Spawner::from_config(config)?);
        }
        Ok(Self(result))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionConfig {
    name: String,
    a: f32,
    b: f32,
}

impl DistributionConfig {
    pub fn new(name: String, a: f32, b: f32) -> Self {
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
    pub fn new(centre: (f32, f32), radius: f32, dist: DistributionConfig) -> Self {
        Self {
            centre,
            radius,
            dist,
        }
    }
}
