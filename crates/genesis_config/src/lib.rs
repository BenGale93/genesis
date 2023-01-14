#![warn(clippy::all, clippy::nursery)]
#![feature(duration_consts_float)]
use std::time::Duration;
mod attr_config;
mod validators;

use bevy_core_pipeline::clear_color::ClearColor;
use bevy_render::color::Color;
use derive_getters::Getters;
use once_cell::sync::OnceCell;
use serde_derive::{Deserialize, Serialize};
extern crate serde;

pub const BACKGROUND: ClearColor = ClearColor(Color::rgb(0.004, 0.09, 0.15));

// Camera
pub const PAN_SPEED: f32 = 1000.0;
pub const ZOOM_SPEED: f32 = 0.1;

// Bugs
pub const INPUT_NEURONS: usize = 15;
pub const OUTPUT_NEURONS: usize = 8;
pub const EATING_MULTIPLIER: f32 = 20.0;
pub const CORE_MULTIPLIER: usize = 100;
pub const HEALTH_MULTIPLIER: usize = 200;
pub const GRAB_SIZE_THRESHOLD: f32 = 10.0;
pub const BEHAVIOUR_TICK_LENGTH: f32 = 0.05;
pub const BEHAVIOUR_TICK: Duration = Duration::from_secs_f32(BEHAVIOUR_TICK_LENGTH);
pub const SLOW_BEHAVIOUR_TICK_LENGTH: f32 = 0.1;
pub const SLOW_BEHAVIOUR_TICK: Duration = Duration::from_secs_f32(SLOW_BEHAVIOUR_TICK_LENGTH);
pub const VERY_SLOW_BEHAVIOUR_TICK_LENGTH: f32 = 1.0;
pub const VERY_SLOW_BEHAVIOUR_TICK: Duration =
    Duration::from_secs_f32(VERY_SLOW_BEHAVIOUR_TICK_LENGTH);

// Outputs
pub const MOVEMENT_INDEX: usize = 0;
pub const ROTATE_INDEX: usize = 1;
pub const REPRODUCE_INDEX: usize = 2;
pub const EAT_INDEX: usize = 3;
pub const RESET_TIMER_INDEX: usize = 4;
pub const WANT_TO_GROWN_INDEX: usize = 5;
pub const WANT_TO_GRAB_INDEX: usize = 6;
pub const DIGEST_FOOD_INDEX: usize = 7;

// Inputs
pub const CONSTANT_INDEX: usize = 0;
pub const PREV_MOVEMENT_INDEX: usize = 1;
pub const PREV_ROTATE_INDEX: usize = 2;
pub const ENERGY_INDEX: usize = 3;
pub const HEALTH_INDEX: usize = 4;
pub const AGE_INDEX: usize = 5;
pub const VISIBLE_BUGS_INDEX: usize = 6;
pub const BUG_ANGLE_SCORE_INDEX: usize = 7;
pub const BUG_DIST_SCORE_INDEX: usize = 8;
pub const VISIBLE_FOOD_INDEX: usize = 9;
pub const FOOD_ANGLE_SCORE_INDEX: usize = 10;
pub const FOOD_DIST_SCORE_INDEX: usize = 11;
pub const HEARTBEAT_INDEX: usize = 12;
pub const INTERNAL_TIMER_INDEX: usize = 13;
pub const FULLNESS_INDEX: usize = 14;

// Other
pub const GENERATION_SWITCH: usize = 5;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistributionConfig {
    pub name: String,
    pub a: f32,
    pub b: f32,
}

impl DistributionConfig {
    pub const fn new(name: String, a: f32, b: f32) -> Self {
        Self { name, a, b }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpawnerConfig {
    pub centre: (f32, f32),
    pub radius: f32,
    pub dist: DistributionConfig,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldConfig {
    pub start_num: usize,
    pub minimum_number: usize,
    pub energy_floor: usize,
    pub initial_synapse_count: usize,
    pub mutations: usize,
    pub start_energy: usize,
    pub lowest_energy_limit: usize,
    pub max_rotation: f32,
    pub rotation_cost: f32,
    pub max_translation: f32,
    pub translation_cost: f32,
    pub unit_size_cost: f32,
    pub world_energy: usize,
    pub mutation_probability: f32,
    pub cost_of_thought: f32,
    pub cost_of_grab: f32,
    pub cost_of_lay: f32,
    pub plant: PlantConfig,
    pub meat: MeatConfig,
    pub spawners: Vec<SpawnerConfig>,
    pub attributes: attr_config::AttributeConfig,
    pub dependent_attributes: attr_config::DependentAttributeConfig,
    pub brain_mutations: BrainMutationConfig,
}

impl WorldConfig {
    pub fn global() -> &'static Self {
        WORLD_CONFIG_INSTANCE
            .get()
            .expect("World config is not initialized")
    }

    pub fn from_config() -> Result<Self, Vec<String>> {
        let world_config: Self = confy::load_path("./config/genesis.toml").unwrap();
        world_config.validate()?;
        Ok(world_config)
    }

    fn validate(&self) -> Result<(), Vec<String>> {
        let mut messages = vec![
            validators::min_value(0.0, self.unit_size_cost, "unit_size_cost"),
            validators::between(self.max_rotation, 5.0, 40.0, "max_rotation"),
            validators::between(self.max_translation, 100.0, 1000.0, "max_translation"),
            validators::between(self.rotation_cost, 0.0, 10.0, "rotation_cost"),
            validators::between(self.translation_cost, 0.0, 10.0, "translation_cost"),
            validators::between(self.cost_of_thought, 0.0, 10.0, "cost_of_thought"),
            validators::between(self.cost_of_grab, 0.0, 10.0, "cost_of_grab"),
            validators::between(self.cost_of_lay, 0.0, 10.0, "cost_of_lay"),
            validators::low_high(
                self.minimum_number,
                self.start_num,
                "minimum_number",
                "start_num",
            ),
        ];
        messages.extend(self.plant.validate());
        messages.extend(self.meat.validate());
        messages.extend(self.attributes.validate());
        messages.extend(self.dependent_attributes.validate());
        messages.extend(self.brain_mutations.validate());

        let failures: Vec<String> = messages.into_iter().flatten().collect();
        if !failures.is_empty() {
            return Err(failures);
        }
        Ok(())
    }
}

impl Default for WorldConfig {
    fn default() -> Self {
        let dist = DistributionConfig::new("normal".to_string(), 0.0, 1.0);
        let spawner = SpawnerConfig::new((0.0, 0.0), 500.0, dist);
        Self {
            start_num: 0,
            minimum_number: 0,
            energy_floor: 100000,
            initial_synapse_count: 3,
            mutations: 3,
            start_energy: 10000,
            lowest_energy_limit: 20000,
            max_rotation: 15.0,
            rotation_cost: 1.5,
            max_translation: 400.0,
            translation_cost: 1.5,
            unit_size_cost: 2.00,
            world_energy: 1000000,
            mutation_probability: 0.1,
            cost_of_thought: 0.8,
            cost_of_grab: 2.0,
            cost_of_lay: 5.0,
            plant: PlantConfig::default(),
            meat: MeatConfig::default(),
            spawners: vec![spawner],
            attributes: attr_config::AttributeConfig::default(),
            dependent_attributes: attr_config::DependentAttributeConfig::default(),
            brain_mutations: BrainMutationConfig::default(),
        }
    }
}

pub static WORLD_CONFIG_INSTANCE: OnceCell<WorldConfig> = OnceCell::new();

pub struct EnergyLimitConfig {
    a: f32,
    b: f32,
}

impl EnergyLimitConfig {
    pub fn new(config: &WorldConfig) -> Self {
        let e = config.lowest_energy_limit as f32;
        let h = config.dependent_attributes.hatch_size_bounds.0;
        let m = config.attributes.max_size.1;

        let a = (e / h) * (10.0 * (m - h) / m);
        let b = 5.0f32.mul_add(m, -10.0 * h) / (h * m);

        Self { a, b }
    }
    pub fn energy_limit(&self, size: usize) -> usize {
        let size_f = size as f32;

        ((self.a * size_f) / self.b.mul_add(size_f, 5.0)) as usize
    }

    pub fn global() -> &'static Self {
        ENERGY_LIMIT_INSTANCE
            .get()
            .expect("Energy limit config is not initialized")
    }
}

pub static ENERGY_LIMIT_INSTANCE: OnceCell<EnergyLimitConfig> = OnceCell::new();

pub fn initialize_configs(input_config: Option<WorldConfig>) {
    let config = input_config.map_or_else(
        || match WorldConfig::from_config() {
            Ok(c) => c,
            Err(e) => panic!("Config validation failed. Issues are: {e:?}"),
        },
        |c| c,
    );
    let energy_limit_config = EnergyLimitConfig::new(&config);
    _ = WORLD_CONFIG_INSTANCE.set(config);
    _ = ENERGY_LIMIT_INSTANCE.set(energy_limit_config);
}

#[derive(Debug, Serialize, Deserialize, Getters, Clone)]
pub struct BrainMutationConfig {
    deactivate_neuron: f32,
    add_neuron: f32,
    neuron_bias: f32,
    activation_func: f32,
    synapse_weight: f32,
    deactivate_synapse: f32,
    add_synapse: f32,
}

impl Default for BrainMutationConfig {
    fn default() -> Self {
        Self {
            deactivate_neuron: 0.1,
            add_neuron: 0.1,
            neuron_bias: 0.05,
            activation_func: 0.05,
            synapse_weight: 0.5,
            deactivate_synapse: 0.1,
            add_synapse: 0.1,
        }
    }
}

impl BrainMutationConfig {
    pub fn validate(&self) -> Vec<Option<String>> {
        let mut messages = vec![];
        macro_rules! probabilities {
            ($attr:ident) => {
                messages.push(validators::between(
                    self.$attr,
                    0.0,
                    1.0,
                    stringify!($attr),
                ))
            };
            ($attr:ident, $($attrs:ident), +) => {
                probabilities!($attr);
                probabilities!($($attrs), +)
            }
        }
        probabilities!(
            deactivate_neuron,
            add_neuron,
            neuron_bias,
            activation_func,
            synapse_weight,
            deactivate_synapse,
            add_synapse
        );
        let total_probability = self.deactivate_neuron
            + self.add_neuron
            + self.neuron_bias
            + self.activation_func
            + self.synapse_weight
            + self.deactivate_synapse
            + self.add_synapse;

        messages.push(validators::between(
            total_probability,
            1.0,
            1.0,
            "Total Brain Probabilities",
        ));

        messages
    }
}

#[derive(Debug, Serialize, Deserialize, Getters, Clone)]
pub struct PlantConfig {
    pub energy_density: usize,
    pub toughness: f32,
    pub size_range: (f32, f32),
    pub density: f32,
}

impl Default for PlantConfig {
    fn default() -> Self {
        Self {
            energy_density: 400,
            toughness: 5.0,
            size_range: (10.0, 30.0),
            density: 10.0,
        }
    }
}

impl PlantConfig {
    pub fn validate(&self) -> Vec<Option<String>> {
        let mut messages = vec![
            validators::min_value(1, self.energy_density, "plant.energy_density"),
            validators::between(self.toughness, 1.0, 100.0, "plant.toughness"),
            validators::between(self.density, 1.0, 100.0, "plant.density"),
        ];
        messages.push(validators::low_high_tuple(
            self.size_range,
            "plant.size_range",
        ));
        messages
    }
}

#[derive(Debug, Serialize, Deserialize, Getters, Clone)]
pub struct MeatConfig {
    pub energy_density: usize,
    pub toughness: f32,
    pub density: f32,
    pub rot_rate: usize,
}

impl Default for MeatConfig {
    fn default() -> Self {
        Self {
            energy_density: 700,
            toughness: 2.0,
            density: 5.0,
            rot_rate: 20,
        }
    }
}

impl MeatConfig {
    pub fn validate(&self) -> Vec<Option<String>> {
        vec![
            validators::min_value(1, self.energy_density, "meat.energy_density"),
            validators::between(self.toughness, 1.0, 100.0, "meat.toughness"),
            validators::between(self.density, 1.0, 100.0, "meat.density"),
            validators::min_value(1, self.rot_rate, "meat.rot_rate"),
        ]
    }
}
