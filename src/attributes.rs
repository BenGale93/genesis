//! Attributes are values that are read from Chromosomes comprised of 0 or 1.
//!
//! Adult age and death age are found on chromosome 0.
//! Mutation probability is found on chromosome 1.
//! Max speed and max rotation rate are found on chromosome 2.
//! Translation and rotation costs are linear interpolations of max speed
//! and max rotation.
//! Eye range and angle are on chromosome 3 and can either be exactly correlated
//! inversely correlated.
//! Internal timer, lay egg and eating boundaries can be found on chromosome 4.
//! Cost of thought, growth rate, and cost of eating can be found on chromosome 5.
//! Size related attributes are on chromosome 10.
//! Offspring energy and hatch age are on chromosome 10.
use bevy::prelude::{Bundle, Component};
use derive_more::Deref;
use genesis_genome::Genome;
use genesis_util::{maths::linear_interpolate, Probability};

use crate::config;

#[derive(Debug)]
struct AttributeConfig {
    lower: f32,
    upper: f32,
    chromosome: usize,
    start: usize,
    length: usize,
}

impl AttributeConfig {
    fn new(lower: f32, upper: f32, chromosome: usize, start: usize, length: usize) -> Self {
        Self {
            lower,
            upper,
            chromosome,
            start,
            length,
        }
    }

    fn read_genome(&self, genome: &Genome) -> f32 {
        genome
            .read_float(
                self.lower,
                self.upper,
                self.chromosome,
                self.start,
                self.length,
            )
            .unwrap_or_else(|_| {
                panic!(
                    "Expected to be able to read from chromosome {} at {} for {}",
                    self.chromosome, self.start, self.length
                )
            })
    }
}

macro_rules! impl_from_genome {
    ($name:ident) => {
        impl $name {
            pub fn from_genome(genome: &Genome) -> Self {
                let attribute_config = Self::default_config();
                let value = attribute_config.read_genome(genome);
                $name(value.into())
            }
        }
    };
}

#[derive(Component, Debug, Deref)]
pub struct AdultAge(f32);

impl AdultAge {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.adult_age;
        AttributeConfig::new(min, max, 0, 0, length)
    }
}

impl_from_genome!(AdultAge);

#[derive(Component, Debug, Deref)]
pub struct DeathAge(f32);

impl DeathAge {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.death_age;
        AttributeConfig::new(min, max, 0, 4, length)
    }
}

impl_from_genome!(DeathAge);

#[derive(Component, Debug, Deref)]
pub struct MutationProbability(Probability);

impl MutationProbability {
    pub fn from_genome(genome: &Genome) -> Self {
        let attribute_config = Self::default_config();
        let value = Probability::new(attribute_config.read_genome(genome) as f64)
            .expect("Expected to be between 0.0 and 1.0.");
        Self(value)
    }

    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global()
            .attributes
            .mutation_probability;
        AttributeConfig::new(min, max, 1, 0, length)
    }
}

#[derive(Component, Debug)]
pub struct MaxSpeed {
    value: f32,
    cost: f32,
}

impl MaxSpeed {
    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn cost(&self) -> f32 {
        self.cost
    }

    pub fn from_genome(genome: &Genome) -> Self {
        let attribute_config = Self::default_config();
        let value = attribute_config.read_genome(genome);
        let cost = Self::compute_cost(value, &attribute_config);
        Self { value, cost }
    }

    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.max_speed;
        AttributeConfig::new(min, max, 2, 0, length)
    }

    fn compute_cost(value: f32, config: &AttributeConfig) -> f32 {
        let cost_bounds = config::WorldConfig::global().translation_cost;
        linear_interpolate(
            value,
            config.lower,
            config.upper,
            cost_bounds.0,
            cost_bounds.1,
        )
    }
}

#[derive(Component, Debug)]
pub struct MaxRotationRate {
    value: f32,
    cost: f32,
}

impl MaxRotationRate {
    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn cost(&self) -> f32 {
        self.cost
    }

    pub fn from_genome(genome: &Genome) -> Self {
        let attribute_config = Self::default_config();
        let value = attribute_config.read_genome(genome);
        let cost = Self::compute_cost(value, &attribute_config);
        Self { value, cost }
    }
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.max_rotation;
        AttributeConfig::new(min, max, 2, 20, length)
    }

    fn compute_cost(value: f32, config: &AttributeConfig) -> f32 {
        let cost_bounds = config::WorldConfig::global().rotation_cost;
        linear_interpolate(
            value,
            config.lower,
            config.upper,
            cost_bounds.0,
            cost_bounds.1,
        )
    }
}

#[derive(Component, Debug, Deref)]
pub struct EyeRange(f32);

impl EyeRange {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.eye_range;
        AttributeConfig::new(min, max, 3, 0, length)
    }
}

impl_from_genome!(EyeRange);

#[derive(Component, Debug, Deref)]
pub struct EyeAngle(f32);

impl EyeAngle {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.eye_angle;
        AttributeConfig::new(f32::to_radians(min), f32::to_radians(max), 3, 0, length)
    }
}

impl_from_genome!(EyeAngle);

#[derive(Component, Debug, Deref)]
pub struct InternalTimerBoundary(f64);

impl InternalTimerBoundary {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global()
            .attributes
            .internal_timer_boundary;
        AttributeConfig::new(min, max, 4, 0, length)
    }
}

impl_from_genome!(InternalTimerBoundary);

#[derive(Component, Debug, Deref)]
pub struct LayEggBoundary(f64);

impl LayEggBoundary {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.lay_egg_boundary;
        AttributeConfig::new(min, max, 4, 30, length)
    }
}

impl_from_genome!(LayEggBoundary);

#[derive(Component, Debug, Deref)]
pub struct WantToGrowBoundary(f64);

impl WantToGrowBoundary {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global()
            .attributes
            .want_to_grow_boundary;
        AttributeConfig::new(min, max, 4, 60, length)
    }
}

impl_from_genome!(WantToGrowBoundary);

#[derive(Component, Debug, Deref)]
pub struct EatingBoundary(f64);

impl EatingBoundary {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.eating_boundary;
        AttributeConfig::new(min, max, 4, 50, length)
    }
}

impl_from_genome!(EatingBoundary);

#[derive(Component, Debug, Deref)]
pub struct CostOfThought(f32);

impl CostOfThought {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.cost_of_thought;
        AttributeConfig::new(min, max, 5, 0, length)
    }
}

impl_from_genome!(CostOfThought);

#[derive(Component, Debug, Deref)]
pub struct CostOfEating(f32);

impl CostOfEating {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.cost_of_eating;
        AttributeConfig::new(min, max, 5, 5, length)
    }
}

impl_from_genome!(CostOfEating);

#[derive(Component, Debug, Deref)]
pub struct OffspringEnergy(usize);

impl OffspringEnergy {
    pub fn from_genome(genome: &Genome) -> Self {
        let attribute_config = Self::default_config();
        let value = attribute_config.read_genome(genome) as usize;
        Self(value)
    }

    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.offspring_energy;
        AttributeConfig::new(min, max, 10, 0, length)
    }
}

#[derive(Component, Debug, Deref)]
pub struct HatchSize(f32);

impl HatchSize {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.hatch_size;
        AttributeConfig::new(min, max, 10, 3, length)
    }
}

impl_from_genome!(HatchSize);

#[derive(Component, Debug, Deref)]
pub struct MaxSize(f32);

impl MaxSize {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.max_size;
        AttributeConfig::new(min, max, 10, 8, length)
    }
}

impl_from_genome!(MaxSize);

#[derive(Component, Debug, Deref)]
pub struct GrowthRate(f32);

impl GrowthRate {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.growth_rate;
        AttributeConfig::new(min, max, 0, 5, length)
    }
}

impl_from_genome!(GrowthRate);
#[derive(Bundle, Debug)]
pub struct AttributeBundle {
    pub adult_age: AdultAge,
    pub death_age: DeathAge,
    pub mutation_probability: MutationProbability,
    pub translation_speed: MaxSpeed,
    pub rotation_speed: MaxRotationRate,
    pub eye_range: EyeRange,
    pub eye_angle: EyeAngle,
    pub internal_timer_boundary: InternalTimerBoundary,
    pub lay_egg_boundary: LayEggBoundary,
    pub want_to_grow_boundary: WantToGrowBoundary,
    pub eating_boundary: EatingBoundary,
    pub cost_of_thought: CostOfThought,
    pub cost_of_eating: CostOfEating,
    pub offspring_energy: OffspringEnergy,
    pub hatch_size: HatchSize,
    pub max_size: MaxSize,
    pub growth_rate: GrowthRate,
}

impl AttributeBundle {
    pub fn new(genome: &Genome) -> Self {
        let adult_age = AdultAge::from_genome(genome);
        let death_age = DeathAge::from_genome(genome);
        let mutation_probability = MutationProbability::from_genome(genome);
        let translation_speed = MaxSpeed::from_genome(genome);
        let rotation_speed = MaxRotationRate::from_genome(genome);
        let eye_range = EyeRange::from_genome(genome);
        let eye_angle = EyeAngle::from_genome(genome);
        let internal_timer_boundary = InternalTimerBoundary::from_genome(genome);
        let lay_egg_boundary = LayEggBoundary::from_genome(genome);
        let want_to_grow_boundary = WantToGrowBoundary::from_genome(genome);
        let eating_boundary = EatingBoundary::from_genome(genome);
        let cost_of_thought = CostOfThought::from_genome(genome);
        let cost_of_eating = CostOfEating::from_genome(genome);
        let offspring_energy = OffspringEnergy::from_genome(genome);
        let hatch_size = HatchSize::from_genome(genome);
        let max_size = MaxSize::from_genome(genome);
        let growth_rate = GrowthRate::from_genome(genome);

        Self {
            adult_age,
            death_age,
            mutation_probability,
            translation_speed,
            rotation_speed,
            eye_range,
            eye_angle,
            internal_timer_boundary,
            lay_egg_boundary,
            want_to_grow_boundary,
            eating_boundary,
            cost_of_thought,
            cost_of_eating,
            offspring_energy,
            hatch_size,
            max_size,
            growth_rate,
        }
    }
}

#[derive(Component, Debug, Deref)]
pub struct HatchAge(f32);

impl HatchAge {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.hatch_age;
        AttributeConfig::new(min, max, 10, 5, length)
    }
}

impl_from_genome!(HatchAge);

#[derive(Bundle, Debug)]
pub struct EggAttributeBundle {
    pub hatch_age: HatchAge,
}

impl EggAttributeBundle {
    pub fn new(genome: &Genome) -> Self {
        let hatch_age = HatchAge::from_genome(genome);

        Self { hatch_age }
    }
}

pub type BugAttributesPart1<'a> = (
    &'a AdultAge,
    &'a DeathAge,
    &'a EyeAngle,
    &'a EyeRange,
    &'a MaxRotationRate,
    &'a MaxSpeed,
    &'a MutationProbability,
    &'a OffspringEnergy,
    &'a LayEggBoundary,
    &'a InternalTimerBoundary,
    &'a WantToGrowBoundary,
    &'a EatingBoundary,
    &'a CostOfThought,
    &'a CostOfEating,
    &'a MaxSize,
);

pub type BugAttributesPart2<'a> = (&'a GrowthRate,);
