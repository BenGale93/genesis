#![warn(clippy::all, clippy::nursery)]
use bevy_ecs::prelude::{Bundle, Component};
use bevy_reflect::{Reflect, Struct};
use derive_more::Deref;
use genesis_config as config;
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

    pub fn range(&self) -> f32 {
        (self.highest() - self.lowest()).abs()
    }

    pub fn lowest(&self) -> f32 {
        *self.array.first().unwrap()
    }

    pub fn highest(&self) -> f32 {
        *self.array.last().unwrap()
    }

    pub fn normalise(&self) -> f32 {
        (self.value - self.lowest()) / self.range()
    }
}

#[derive(Debug, Component, Reflect, Clone)]
pub struct Genome {
    pub hatch_age: Chromosome,
    pub eye_range: Chromosome,
    pub cost_of_eating: Chromosome,
    pub offspring_energy: Chromosome,
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
        get_value!(
            hatch_age,
            eye_range,
            cost_of_eating,
            offspring_energy,
            max_size,
            growth_rate
        );
        Self {
            hatch_age,
            eye_range,
            cost_of_eating,
            offspring_energy,
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
    pub fn new(hatch_age: &Chromosome) -> Self {
        let (aa_min, aa_max) = config::WorldConfig::global()
            .dependent_attributes
            .adult_age_bounds;
        let aa_range = aa_max - aa_min;
        let value = hatch_age.normalise().mul_add(-aa_range, aa_max);
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct DeathAge(f32);

impl DeathAge {
    pub fn new(max_size: &Chromosome) -> Self {
        let (da_min, da_max) = config::WorldConfig::global()
            .dependent_attributes
            .death_age_bounds;
        let da_range = da_max - da_min;
        let value = max_size.normalise().mul_add(da_range, da_min);
        Self(value)
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
    pub fn new(eye_range: &Chromosome) -> Self {
        let (ea_min, ea_max) = config::WorldConfig::global()
            .dependent_attributes
            .eye_angle_bounds;
        let ea_range = ea_max - ea_min;
        let value = f32::to_radians(eye_range.normalise().mul_add(-ea_range, ea_max));
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
    pub fn new(cost_of_eating: &Chromosome) -> Self {
        let (mw_min, mw_max) = config::WorldConfig::global()
            .dependent_attributes
            .mouth_width_bounds;
        let mw_range = mw_max - mw_min;
        let value = f32::to_radians(cost_of_eating.normalise().mul_add(mw_range, mw_min));
        Self(value)
    }
}

#[derive(Component, Debug, Deref)]
pub struct HatchSize(f32);

impl HatchSize {
    pub fn new(hatch_age: &Chromosome) -> Self {
        let (hs_min, hs_max) = config::WorldConfig::global()
            .dependent_attributes
            .hatch_size_bounds;
        let hs_range = hs_max - hs_min;
        let value = hatch_age.normalise().mul_add(hs_range, hs_min);
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
            adult_age: AdultAge::new(&values.hatch_age),
            death_age: DeathAge::new(&values.max_size),
            eye_range: EyeRange::new(values.eye_range.value),
            eye_angle: EyeAngle::new(&values.eye_range),
            cost_of_eating: CostOfEating::new(values.cost_of_eating.value),
            offspring_energy: OffspringEnergy::new(values.offspring_energy.value),
            mouth_width: MouthWidth::new(&values.cost_of_eating),
            hatch_size: HatchSize::new(&values.hatch_age),
            max_size: MaxSize::new(values.max_size.value),
            growth_rate: GrowthRate::new(values.growth_rate.value),
        }
    }
}

pub type BugAttributes<'a> = (
    &'a HatchAge,
    &'a AdultAge,
    &'a DeathAge,
    &'a EyeRange,
    &'a EyeAngle,
    &'a CostOfEating,
    &'a OffspringEnergy,
    &'a MouthWidth,
    &'a HatchSize,
    &'a MaxSize,
    &'a GrowthRate,
);
