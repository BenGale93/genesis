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
//! Cost of thought, growth rate, cost of eating, and mouth width can be found on chromosome 5.
//! Size related attributes are on chromosome 10.
//! Offspring energy and hatch age are on chromosome 10.
use bevy::prelude::{Bundle, Component};
use derive_more::Deref;
use genesis_genome::Genome;
use genesis_maths::linear_interpolate;
use genesis_util::Probability;

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
    const fn new(lower: f32, upper: f32, chromosome: usize, start: usize, length: usize) -> Self {
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
    () => {
        pub fn from_genome(genome: &Genome) -> Self {
            let attribute_config = Self::default_config();
            let value = attribute_config.read_genome(genome);
            Self(value.into())
        }
    };
}

macro_rules! impl_default_config {
    ($attr:ident, $chromosome:literal, $start:literal) => {
        fn default_config() -> AttributeConfig {
            let (min, max, length) = config::WorldConfig::global().attributes.$attr;
            AttributeConfig::new(min, max, $chromosome, $start, length)
        }
    };
}

#[derive(Component, Debug, Deref)]
pub struct AdultAge(f32);

impl AdultAge {
    impl_default_config!(adult_age, 0, 0);
    impl_from_genome!();
}

#[derive(Component, Debug, Deref)]
pub struct DeathAge(f32);

impl DeathAge {
    impl_default_config!(death_age, 0, 4);
    impl_from_genome!();
}

#[derive(Component, Debug, Deref)]
pub struct MutationProbability(Probability);

impl MutationProbability {
    pub fn from_genome(genome: &Genome) -> Self {
        let attribute_config = Self::default_config();
        let value = Probability::new(attribute_config.read_genome(genome))
            .expect("Expected to be between 0.0 and 1.0.");
        Self(value)
    }
    impl_default_config!(mutation_probability, 1, 0);
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

    pub fn from_genome(genome: &Genome) -> Self {
        let attribute_config = Self::default_config();
        let value = attribute_config.read_genome(genome);
        let cost = Self::compute_cost(value, &attribute_config);
        Self { value, cost }
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

    impl_default_config!(max_speed, 2, 0);
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

    pub fn from_genome(genome: &Genome) -> Self {
        let attribute_config = Self::default_config();
        let value = attribute_config.read_genome(genome);
        let cost = Self::compute_cost(value, &attribute_config);
        Self { value, cost }
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

    impl_default_config!(max_rotation, 2, 20);
}

#[derive(Component, Debug, Deref)]
pub struct EyeRange(f32);

impl EyeRange {
    impl_from_genome!();
    impl_default_config!(eye_range, 3, 0);
}

#[derive(Component, Debug, Deref)]
pub struct EyeAngle(f32);

impl EyeAngle {
    fn default_config() -> AttributeConfig {
        let (min, max, length) = config::WorldConfig::global().attributes.eye_angle;
        AttributeConfig::new(f32::to_radians(min), f32::to_radians(max), 3, 0, length)
    }
    impl_from_genome!();
}

#[derive(Component, Debug, Deref)]
pub struct InternalTimerBoundary(f32);

impl InternalTimerBoundary {
    impl_from_genome!();
    impl_default_config!(internal_timer_boundary, 4, 0);
}

#[derive(Component, Debug, Deref)]
pub struct LayEggBoundary(f32);

impl LayEggBoundary {
    impl_from_genome!();
    impl_default_config!(lay_egg_boundary, 4, 30);
}

#[derive(Component, Debug, Deref)]
pub struct WantToGrowBoundary(f32);

impl WantToGrowBoundary {
    impl_from_genome!();
    impl_default_config!(want_to_grow_boundary, 4, 60);
}

#[derive(Component, Debug, Deref)]
pub struct EatingBoundary(f32);

impl EatingBoundary {
    impl_from_genome!();
    impl_default_config!(eating_boundary, 4, 50);
}

#[derive(Component, Debug, Deref)]
pub struct CostOfThought(f32);

impl CostOfThought {
    impl_from_genome!();
    impl_default_config!(cost_of_thought, 5, 0);
}

#[derive(Component, Debug, Deref)]
pub struct CostOfEating(f32);

impl CostOfEating {
    impl_from_genome!();
    impl_default_config!(cost_of_eating, 5, 5);
}

#[derive(Component, Debug, Deref)]
pub struct MouthWidth(f32);

impl MouthWidth {
    impl_from_genome!();
    impl_default_config!(mouth_width, 5, 5);
}

#[derive(Component, Debug, Deref)]
pub struct OffspringEnergy(f32);

impl OffspringEnergy {
    impl_from_genome!();
    impl_default_config!(offspring_energy, 10, 0);
}

#[derive(Component, Debug, Deref)]
pub struct HatchSize(f32);

impl HatchSize {
    impl_from_genome!();
    impl_default_config!(hatch_size, 10, 3);
}

#[derive(Component, Debug, Deref)]
pub struct MaxSize(f32);

impl MaxSize {
    impl_from_genome!();
    impl_default_config!(max_size, 10, 8);
}

#[derive(Component, Debug, Deref)]
pub struct GrowthRate(f32);

impl GrowthRate {
    impl_from_genome!();
    impl_default_config!(growth_rate, 0, 5);
}

#[derive(Component, Debug, Deref)]
pub struct HatchAge(f32);

impl HatchAge {
    impl_from_genome!();
    impl_default_config!(hatch_age, 10, 5);
}

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
    pub mouth_width: MouthWidth,
    pub hatch_size: HatchSize,
    pub max_size: MaxSize,
    pub growth_rate: GrowthRate,
    pub hatch_age: HatchAge,
}

impl AttributeBundle {
    pub fn new(genome: &Genome) -> Self {
        Self {
            adult_age: AdultAge::from_genome(genome),
            death_age: DeathAge::from_genome(genome),
            mutation_probability: MutationProbability::from_genome(genome),
            translation_speed: MaxSpeed::from_genome(genome),
            rotation_speed: MaxRotationRate::from_genome(genome),
            eye_range: EyeRange::from_genome(genome),
            eye_angle: EyeAngle::from_genome(genome),
            internal_timer_boundary: InternalTimerBoundary::from_genome(genome),
            lay_egg_boundary: LayEggBoundary::from_genome(genome),
            want_to_grow_boundary: WantToGrowBoundary::from_genome(genome),
            eating_boundary: EatingBoundary::from_genome(genome),
            cost_of_thought: CostOfThought::from_genome(genome),
            cost_of_eating: CostOfEating::from_genome(genome),
            mouth_width: MouthWidth::from_genome(genome),
            offspring_energy: OffspringEnergy::from_genome(genome),
            hatch_size: HatchSize::from_genome(genome),
            max_size: MaxSize::from_genome(genome),
            growth_rate: GrowthRate::from_genome(genome),
            hatch_age: HatchAge::from_genome(genome),
        }
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
    &'a HatchSize,
);

pub type BugAttributesPart2<'a> = (&'a MaxSize, &'a GrowthRate, &'a MouthWidth, &'a HatchAge);
