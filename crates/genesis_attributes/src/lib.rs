#![warn(clippy::all, clippy::nursery)]
use bevy_ecs::prelude::{Bundle, Component};
use bevy_reflect::{Reflect, Struct};
use derive_more::Deref;
use genesis_config as config;
use genesis_maths::linear_interpolate;
use genesis_newtype::Probability;
use ndarray::Array;
use rand::{seq::IteratorRandom, Rng, RngCore};

#[derive(Debug, Reflect, Clone)]
pub struct Chromosome {
    array: Vec<f32>,
    value: f32,
}

impl Chromosome {
    pub fn new(lower: f32, upper: f32, steps: usize, rng: &mut dyn RngCore) -> Self {
        let array = Array::linspace(lower, upper, steps);
        let value = array.iter().copied().choose(rng).unwrap();
        Self {
            array: array.to_vec(),
            value,
        }
    }

    pub fn mutate(&mut self, rng: &mut dyn RngCore) {
        let max = self.array.len() - 1;
        let position = self.array.iter().position(|&x| x == self.value).unwrap();
        let new_position = if rng.gen_bool(0.5) {
            position.saturating_sub(1)
        } else {
            (position + 1).clamp(0, max)
        };
        self.value = self.array[new_position];
    }
}

#[derive(Debug, Component, Reflect, Clone)]
pub struct Genome {
    pub hatch_age: Chromosome,
    pub adult_age: Chromosome,
    pub death_age: Chromosome,
    pub max_speed: Chromosome,
    pub max_rotation: Chromosome,
    pub eye_range: Chromosome,
    pub eye_angle: Chromosome,
    pub cost_of_eating: Chromosome,
    pub offspring_energy: Chromosome,
    pub mouth_width: Chromosome,
    pub hatch_size: Chromosome,
    pub max_size: Chromosome,
    pub growth_rate: Chromosome,
}

impl Genome {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        let attributes = &config::WorldConfig::global().attributes;
        macro_rules! get_value {
            ($attr:ident) => {
                let (min, max, steps) = attributes.$attr;
                let $attr = Chromosome::new(min, max, steps, rng);
            };
            ($attr:ident, $($attrs:ident), +) => {
                get_value!($attr);
               get_value!($($attrs), +)
            }
        }
        let (min, max, steps) = attributes.eye_angle;
        let eye_angle = Chromosome::new(f32::to_radians(min), f32::to_radians(max), steps, rng);
        get_value!(
            hatch_age,
            adult_age,
            death_age,
            max_speed,
            max_rotation,
            eye_range,
            cost_of_eating,
            offspring_energy,
            mouth_width,
            hatch_size,
            max_size,
            growth_rate
        );
        Self {
            hatch_age,
            adult_age,
            death_age,
            max_speed,
            max_rotation,
            eye_range,
            eye_angle,
            cost_of_eating,
            offspring_energy,
            mouth_width,
            hatch_size,
            max_size,
            growth_rate,
        }
    }

    pub fn mutate(&self, rng: &mut dyn RngCore, probability: &Probability) -> Self {
        let mut output = self.clone();
        for (i, _) in self.iter_fields().enumerate() {
            if probability.as_float() >= rng.gen_range(0.0..=1.0) {
                let attribute = output
                    .field_at_mut(i)
                    .unwrap()
                    .downcast_mut::<Chromosome>()
                    .unwrap();
                attribute.mutate(rng);
            }
        }
        output
    }
}

#[derive(Component, Debug, Deref)]
pub struct HatchAge(f32);

impl HatchAge {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct AdultAge(f32);

impl AdultAge {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct DeathAge(f32);

impl DeathAge {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug)]
pub struct MaxSpeed {
    value: f32,
    cost: f32,
}

impl MaxSpeed {
    pub const fn value(&self) -> f32 {
        self.value
    }

    pub const fn cost(&self) -> f32 {
        self.cost
    }

    pub fn new(value: f32) -> Self {
        let cost = Self::compute_cost(value);
        Self { value, cost }
    }

    fn compute_cost(value: f32) -> f32 {
        let config = config::WorldConfig::global();
        let cost_bounds = config.translation_cost;
        let (lower, upper, _) = config.attributes.max_speed;

        linear_interpolate(value, lower, upper, cost_bounds.0, cost_bounds.1)
    }
}

#[derive(Component, Debug)]
pub struct MaxRotationRate {
    value: f32,
    cost: f32,
}

impl MaxRotationRate {
    pub const fn value(&self) -> f32 {
        self.value
    }

    pub const fn cost(&self) -> f32 {
        self.cost
    }

    pub fn new(value: f32) -> Self {
        let cost = Self::compute_cost(value);
        Self { value, cost }
    }

    fn compute_cost(value: f32) -> f32 {
        let config = config::WorldConfig::global();
        let cost_bounds = config.rotation_cost;
        let (lower, upper, _) = config.attributes.max_rotation;

        linear_interpolate(value, lower, upper, cost_bounds.0, cost_bounds.1)
    }
}

#[derive(Component, Debug, Deref)]
pub struct EyeRange(f32);

impl EyeRange {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct EyeAngle(f32);

impl EyeAngle {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct CostOfEating(f32);

impl CostOfEating {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct OffspringEnergy(f32);

impl OffspringEnergy {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct MouthWidth(f32);

impl MouthWidth {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct HatchSize(f32);

impl HatchSize {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct MaxSize(f32);

impl MaxSize {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct GrowthRate(f32);

impl GrowthRate {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Bundle, Debug)]
pub struct AttributeBundle {
    pub hatch_age: HatchAge,
    pub adult_age: AdultAge,
    pub death_age: DeathAge,
    pub translation_speed: MaxSpeed,
    pub rotation_speed: MaxRotationRate,
    pub eye_range: EyeRange,
    pub eye_angle: EyeAngle,
    pub cost_of_eating: CostOfEating,
    pub offspring_energy: OffspringEnergy,
    pub mouth_width: MouthWidth,
    pub hatch_size: HatchSize,
    pub max_size: MaxSize,
    pub growth_rate: GrowthRate,
}

impl AttributeBundle {
    pub fn new(values: &Genome) -> Self {
        Self {
            hatch_age: HatchAge::new(values.hatch_age.value),
            adult_age: AdultAge::new(values.adult_age.value),
            death_age: DeathAge::new(values.death_age.value),
            translation_speed: MaxSpeed::new(values.max_speed.value),
            rotation_speed: MaxRotationRate::new(values.max_rotation.value),
            eye_range: EyeRange::new(values.eye_range.value),
            eye_angle: EyeAngle::new(values.eye_angle.value),
            cost_of_eating: CostOfEating::new(values.cost_of_eating.value),
            offspring_energy: OffspringEnergy::new(values.offspring_energy.value),
            mouth_width: MouthWidth::new(values.mouth_width.value),
            hatch_size: HatchSize::new(values.hatch_size.value),
            max_size: MaxSize::new(values.max_size.value),
            growth_rate: GrowthRate::new(values.growth_rate.value),
        }
    }
}

pub type BugAttributes<'a> = (
    &'a HatchAge,
    &'a AdultAge,
    &'a DeathAge,
    &'a MaxSpeed,
    &'a MaxRotationRate,
    &'a EyeRange,
    &'a EyeAngle,
    &'a CostOfEating,
    &'a OffspringEnergy,
    &'a MouthWidth,
    &'a HatchSize,
    &'a MaxSize,
    &'a GrowthRate,
);
