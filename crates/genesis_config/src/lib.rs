#![warn(clippy::all, clippy::nursery)]
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
pub const INPUT_NEURONS: usize = 14;
pub const OUTPUT_NEURONS: usize = 7;
pub const EATING_RATIO: usize = 5;
pub const CORE_MULTIPLIER: usize = 2;
pub const HEALTH_MULTIPLIER: usize = 3;

// Outputs
pub const MOVEMENT_INDEX: usize = 0;
pub const ROTATE_INDEX: usize = 1;
pub const REPRODUCE_INDEX: usize = 2;
pub const EAT_INDEX: usize = 3;
pub const RESET_TIMER_INDEX: usize = 4;
pub const WANT_TO_GROWN_INDEX: usize = 5;
pub const WANT_TO_GRAB_INDEX: usize = 6;

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

// Other
pub const GENERATION_SWITCH: usize = 5;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
    pub plant_energy_per_unit: usize,
    pub plant_size_range: (f32, f32),
    pub plant_density: f32,
    pub mutation_probability: f32,
    pub cost_of_thought: f32,
    pub cost_of_grab: f32,
    pub cost_of_lay: f32,
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
            validators::min_value(1, self.plant_energy_per_unit, "plant_energy_per_unit"),
            validators::between(self.mutation_probability, 0.0, 1.0, "mutation_probability"),
            validators::between(self.max_rotation, 5.0, 40.0, "max_rotation"),
            validators::between(self.rotation_cost, 0.0, 1.0, "rotation_cost"),
            validators::between(self.max_translation, 100.0, 1000.0, "max_translation"),
            validators::between(self.translation_cost, 0.0, 1.0, "translation_cost"),
            validators::between(self.cost_of_thought, 0.0, 0.1, "cost_of_thought"),
            validators::between(self.cost_of_grab, 0.0, 0.1, "cost_of_grab"),
            validators::between(self.cost_of_lay, 0.0, 0.1, "cost_of_lay"),
            validators::between(self.plant_density, 1.0, 100.0, "plant_density"),
            validators::low_high(
                self.minimum_number,
                self.start_num,
                "minimum_number",
                "start_num",
            ),
        ];
        messages.push(validators::low_high_tuple(
            self.plant_size_range,
            "plant_size_range",
        ));
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
            energy_floor: 2000,
            initial_synapse_count: 3,
            mutations: 3,
            start_energy: 300,
            lowest_energy_limit: 600,
            max_rotation: 15.0,
            rotation_cost: 0.015,
            max_translation: 400.0,
            translation_cost: 0.015,
            unit_size_cost: 0.02,
            world_energy: 10000,
            plant_energy_per_unit: 2,
            plant_size_range: (10.0, 30.0),
            plant_density: 10.0,
            mutation_probability: 0.1,
            cost_of_thought: 0.008,
            cost_of_grab: 0.02,
            cost_of_lay: 0.05,
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

pub fn initialize_configs() {
    let config = match WorldConfig::from_config() {
        Ok(c) => c,
        Err(e) => panic!("Config validation failed. Issues are: {e:?}"),
    };
    let energy_limit_config = EnergyLimitConfig::new(&config);
    _ = WORLD_CONFIG_INSTANCE.set(config);
    _ = ENERGY_LIMIT_INSTANCE.set(energy_limit_config);
}

#[derive(Debug, Serialize, Deserialize, Getters)]
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
