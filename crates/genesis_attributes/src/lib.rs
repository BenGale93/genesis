#![warn(clippy::all, clippy::nursery)]
use std::fmt;

use bevy_app::Plugin;
use bevy_ecs::{
    prelude::{Bundle, Component, Resource},
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use bevy_trait_query::RegisterExt;
use derive_more::Deref;
use genesis_config as config;
use genesis_derive::AttributeDisplay;
use genesis_newtype::Probability;
use genesis_traits::AttributeDisplay;
use ndarray::Array;
use rand::{seq::IteratorRandom, Rng, RngCore};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Deref)]
pub struct Chromosome(Vec<f32>);

impl Chromosome {
    pub fn new(lower: f32, upper: f32, steps: usize) -> Self {
        let array = Array::linspace(lower, upper, steps);
        Self(array.to_vec())
    }

    pub fn random(&self, rng: &mut dyn RngCore) -> f32 {
        self.iter().copied().choose(rng).unwrap()
    }

    pub fn mutate(
        &self,
        current_value: f32,
        rng: &mut dyn RngCore,
        probability: &Probability,
    ) -> f32 {
        if probability.as_float() >= rng.gen_range(0.0..=1.0) {
            let max = self.len() - 1;
            let position = self.iter().position(|&x| x == current_value).unwrap();
            let new_position = if rng.gen_bool(0.5) {
                position.saturating_sub(1)
            } else {
                (position + 1).clamp(0, max)
            };
            self[new_position]
        } else {
            current_value
        }
    }

    pub fn range(&self) -> f32 {
        (self.highest() - self.lowest()).abs()
    }

    pub fn lowest(&self) -> f32 {
        *self.first().unwrap()
    }

    pub fn highest(&self) -> f32 {
        *self.last().unwrap()
    }

    pub fn normalise(&self, value: f32) -> f32 {
        (value - self.lowest()) / self.range()
    }

    pub fn valid_value(&self, value: f32) -> bool {
        self.iter().any(|&x| x == value)
    }
}

impl fmt::Display for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug, Component, Clone, Resource)]
pub struct Genome {
    pub hatch_age: Chromosome,
    pub eye_range: Chromosome,
    pub cost_of_eating: Chromosome,
    pub offspring_energy: Chromosome,
    pub max_size: Chromosome,
    pub growth_rate: Chromosome,
    pub grab_angle: Chromosome,
}

impl Genome {
    pub fn new() -> Self {
        let attributes = &config::WorldConfig::global().attributes;
        macro_rules! get_value {
            ($attr:ident) => {
                let (min, max, steps) = attributes.$attr;
                let $attr = Chromosome::new(min, max, steps);
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
            growth_rate,
            grab_angle
        );
        Self {
            hatch_age,
            eye_range,
            cost_of_eating,
            offspring_energy,
            max_size,
            growth_rate,
            grab_angle,
        }
    }

    pub fn mutate(
        &self,
        current_dna: Dna,
        rng: &mut dyn RngCore,
        probability: &Probability,
    ) -> Dna {
        let mut output_dna = current_dna;
        macro_rules! mutate_value {
            ($attr:ident) => {
                output_dna.$attr = self.$attr.mutate(current_dna.$attr, rng, probability);
            };
            ($attr:ident, $($attrs:ident), +) => {
                mutate_value!($attr);
                mutate_value!($($attrs), +)
            }
        }
        mutate_value!(
            hatch_age,
            eye_range,
            cost_of_eating,
            offspring_energy,
            max_size,
            growth_rate,
            grab_angle
        );
        output_dna
    }
}

impl Default for Genome {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum DnaValidationError {
    #[error("Invalid value '{0}' for attribute '{1}'. Choose from: '{2}'.")]
    InvalidValue(f32, String, Chromosome),
}

#[derive(Debug, Clone, Copy, Component, Serialize, Deserialize, Reflect, Default)]
#[reflect(Component)]
pub struct Dna {
    pub hatch_age: f32,
    pub eye_range: f32,
    pub cost_of_eating: f32,
    pub offspring_energy: f32,
    pub max_size: f32,
    pub growth_rate: f32,
    pub grab_angle: f32,
}

impl Dna {
    pub fn new(genome: &Genome, rng: &mut dyn RngCore) -> Self {
        Self {
            hatch_age: genome.hatch_age.random(rng),
            eye_range: genome.eye_range.random(rng),
            cost_of_eating: genome.cost_of_eating.random(rng),
            offspring_energy: genome.offspring_energy.random(rng),
            max_size: genome.max_size.random(rng),
            growth_rate: genome.growth_rate.random(rng),
            grab_angle: genome.grab_angle.random(rng),
        }
    }

    pub fn validate(&self, genome: &Genome) -> Result<(), DnaValidationError> {
        macro_rules! validate_values {
            ($attr:ident) => {
                if !genome.$attr.valid_value(self.$attr) {
                    return Err(DnaValidationError::InvalidValue(
                        self.$attr,
                        stringify!($attr).to_string(),
                        genome.$attr.clone(),
                    ))
                }
            };
            ($attr:ident, $($attrs:ident), +) => {
                validate_values!($attr);
                validate_values!($($attrs), +)
            }
        }
        validate_values!(
            hatch_age,
            eye_range,
            cost_of_eating,
            offspring_energy,
            max_size,
            growth_rate,
            grab_angle
        );
        Ok(())
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct HatchAge(f32);

impl HatchAge {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct AdultAge(f32);

impl AdultAge {
    pub fn new(hatch_age: f32, ha_chromosome: &Chromosome) -> Self {
        let (aa_min, aa_max) = config::WorldConfig::global()
            .dependent_attributes
            .adult_age_bounds;
        let aa_range = aa_max - aa_min;
        let value = ha_chromosome
            .normalise(hatch_age)
            .mul_add(-aa_range, aa_max);
        Self(value)
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct DeathAge(f32);

impl DeathAge {
    pub fn new(max_size: f32, ms_chromosome: &Chromosome) -> Self {
        let (da_min, da_max) = config::WorldConfig::global()
            .dependent_attributes
            .death_age_bounds;
        let da_range = da_max - da_min;
        let value = ms_chromosome.normalise(max_size).mul_add(da_range, da_min);
        Self(value)
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct EyeRange(f32);

impl EyeRange {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref, Default, Reflect)]
#[reflect(Component)]
pub struct EyeAngle(f32);

impl EyeAngle {
    pub fn new(eye_range: f32, er_chromosome: &Chromosome) -> Self {
        let (ea_min, ea_max) = config::WorldConfig::global()
            .dependent_attributes
            .eye_angle_bounds;
        let ea_range = ea_max - ea_min;
        let value = f32::to_radians(
            er_chromosome
                .normalise(eye_range)
                .mul_add(-ea_range, ea_max),
        );
        Self(value)
    }
}

impl AttributeDisplay for EyeAngle {
    fn name(&self) -> &str {
        "EyeAngle"
    }
    fn value(&self) -> f32 {
        f32::to_degrees(self.0)
    }

    fn display(&self) -> String {
        format!("{}: {:.3}", self.name(), self.value())
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct CostOfEating(f32);

impl CostOfEating {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct OffspringEnergy(f32);

impl OffspringEnergy {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref, Default, Reflect)]
#[reflect(Component)]
pub struct MouthWidth(f32);

impl MouthWidth {
    pub fn new(cost_of_eating: f32, coe_chromosome: &Chromosome) -> Self {
        let (mw_min, mw_max) = config::WorldConfig::global()
            .dependent_attributes
            .mouth_width_bounds;
        let mw_range = mw_max - mw_min;
        let value = f32::to_radians(
            coe_chromosome
                .normalise(cost_of_eating)
                .mul_add(mw_range, mw_min),
        );
        Self(value)
    }
}

impl AttributeDisplay for MouthWidth {
    fn name(&self) -> &str {
        "MouthWidth"
    }

    fn value(&self) -> f32 {
        f32::to_degrees(self.0)
    }

    fn display(&self) -> String {
        format!("{}: {:.3}", self.name(), self.value())
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct HatchSize(f32);

impl HatchSize {
    pub fn new(hatch_age: f32, ha_chromosome: &Chromosome) -> Self {
        let (hs_min, hs_max) = config::WorldConfig::global()
            .dependent_attributes
            .hatch_size_bounds;
        let hs_range = hs_max - hs_min;
        let value = ha_chromosome.normalise(hatch_age).mul_add(hs_range, hs_min);
        Self(value)
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct MaxSize(f32);

impl MaxSize {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct GrowthRate(f32);

impl GrowthRate {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Deref, Default, Reflect)]
#[reflect(Component)]
pub struct GrabAngle(f32);

impl GrabAngle {
    pub fn new(value: f32) -> Self {
        let value = f32::to_radians(value);
        Self(value)
    }
}

impl AttributeDisplay for GrabAngle {
    fn name(&self) -> &str {
        "GrabAngle"
    }

    fn value(&self) -> f32 {
        f32::to_degrees(self.0)
    }

    fn display(&self) -> String {
        format!("{}: {:.3}", self.name(), self.value())
    }
}

#[derive(Component, Debug, Deref, AttributeDisplay, Default, Reflect)]
#[reflect(Component)]
pub struct GrabStrength(f32);

impl GrabStrength {
    pub fn new(grab_angle: f32, ga_chromosome: &Chromosome) -> Self {
        let (gs_min, gs_max) = config::WorldConfig::global()
            .dependent_attributes
            .grab_strength_bounds;
        let gs_range = gs_max - gs_min;
        let value = ga_chromosome
            .normalise(grab_angle)
            .mul_add(-gs_range, gs_max);
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
    pub grab_angle: GrabAngle,
    pub grab_strength: GrabStrength,
}

impl AttributeBundle {
    pub fn new(dna: &Dna, genome: &Genome) -> Self {
        Self {
            hatch_age: HatchAge::new(dna.hatch_age),
            adult_age: AdultAge::new(dna.hatch_age, &genome.hatch_age),
            death_age: DeathAge::new(dna.max_size, &genome.max_size),
            eye_range: EyeRange::new(dna.eye_range),
            eye_angle: EyeAngle::new(dna.eye_range, &genome.eye_range),
            cost_of_eating: CostOfEating::new(dna.cost_of_eating),
            offspring_energy: OffspringEnergy::new(dna.offspring_energy),
            mouth_width: MouthWidth::new(dna.cost_of_eating, &genome.cost_of_eating),
            hatch_size: HatchSize::new(dna.hatch_age, &genome.hatch_age),
            max_size: MaxSize::new(dna.max_size),
            growth_rate: GrowthRate::new(dna.growth_rate),
            grab_angle: GrabAngle::new(dna.grab_angle),
            grab_strength: GrabStrength::new(dna.grab_angle, &genome.grab_angle),
        }
    }
}

pub struct AttributesPlugin;

impl Plugin for AttributesPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<Dna>()
            .register_type::<HatchAge>()
            .register_type::<AdultAge>()
            .register_type::<DeathAge>()
            .register_type::<EyeRange>()
            .register_type::<EyeAngle>()
            .register_type::<CostOfEating>()
            .register_type::<OffspringEnergy>()
            .register_type::<MouthWidth>()
            .register_type::<MaxSize>()
            .register_type::<GrowthRate>()
            .register_type::<GrabAngle>()
            .register_type::<GrabStrength>()
            .register_component_as::<dyn AttributeDisplay, HatchAge>()
            .register_component_as::<dyn AttributeDisplay, AdultAge>()
            .register_component_as::<dyn AttributeDisplay, DeathAge>()
            .register_component_as::<dyn AttributeDisplay, EyeRange>()
            .register_component_as::<dyn AttributeDisplay, EyeAngle>()
            .register_component_as::<dyn AttributeDisplay, CostOfEating>()
            .register_component_as::<dyn AttributeDisplay, OffspringEnergy>()
            .register_component_as::<dyn AttributeDisplay, MouthWidth>()
            .register_component_as::<dyn AttributeDisplay, HatchSize>()
            .register_component_as::<dyn AttributeDisplay, MaxSize>()
            .register_component_as::<dyn AttributeDisplay, GrowthRate>()
            .register_component_as::<dyn AttributeDisplay, GrabAngle>()
            .register_component_as::<dyn AttributeDisplay, GrabStrength>();
    }
}
