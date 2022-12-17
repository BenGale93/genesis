use std::f32::consts::PI;

use anyhow::anyhow;
use bevy::prelude::{Resource, Vec3};
use derive_more::{Deref, DerefMut};
use genesis_config::{DistributionConfig, SpawnerConfig};
use genesis_maths::polars_to_cart;
use rand::{rngs::ThreadRng, Rng};
use rand_distr::{Distribution, Gamma, InverseGaussian, LogNormal, Normal, Uniform};

pub enum DistributionKind {
    Gamma(Gamma<f32>),
    Normal(Normal<f32>),
    Uniform(Uniform<f32>),
    LogNormal(LogNormal<f32>),
    InverseGaussian(InverseGaussian<f32>),
}

impl DistributionKind {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
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

    fn random_position(&self, rng: &mut ThreadRng) -> Vec3 {
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
