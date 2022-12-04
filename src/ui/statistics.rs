use bevy::prelude::{Query, Res, ResMut, Resource};
use derive_getters::Getters;
use genesis_util::maths::mean;
use serde_derive::Serialize;

use crate::{
    attributes,
    behaviour::{eating, lifecycle, timers},
    ecosystem,
};

fn last_element<T>(vector: &[T]) -> T
where
    T: Default + Copy,
{
    vector.last().copied().unwrap_or_default()
}

#[derive(Debug, Getters, Serialize, Default, Resource)]
pub struct CountStatistics {
    adults: Vec<usize>,
    juveniles: Vec<usize>,
    eggs: Vec<usize>,
}

impl CountStatistics {
    pub fn current_adults(&self) -> usize {
        last_element(&self.adults)
    }

    pub fn current_juveniles(&self) -> usize {
        last_element(&self.juveniles)
    }

    pub fn current_eggs(&self) -> usize {
        last_element(&self.eggs)
    }

    pub fn current_organisms(&self) -> usize {
        self.current_adults() + self.current_juveniles() + self.current_eggs()
    }
}

#[derive(Debug, Getters, Serialize, Default, Resource)]
pub struct EnergyStatistics {
    available_energy: Vec<usize>,
    food_energy: Vec<usize>,
}

impl EnergyStatistics {
    pub fn current_available_energy(&self) -> usize {
        last_element(&self.available_energy)
    }

    pub fn current_food_energy(&self) -> usize {
        last_element(&self.food_energy)
    }
}

#[derive(Debug, Getters, Serialize, Default, Resource)]
pub struct BugPerformanceStatistics {
    highest_energy_consumed: Vec<usize>,
    most_eggs_laid: Vec<usize>,
    max_generation: Vec<usize>,
    oldest_bug: Vec<f32>,
}

impl BugPerformanceStatistics {
    pub fn current_highest_energy_consumed(&self) -> usize {
        last_element(&self.highest_energy_consumed)
    }

    pub fn current_most_eggs_laid(&self) -> usize {
        last_element(&self.most_eggs_laid)
    }

    pub fn current_max_generation(&self) -> usize {
        last_element(&self.max_generation)
    }

    pub fn current_oldest_bug(&self) -> f32 {
        last_element(&self.oldest_bug)
    }
}

#[derive(Debug, Getters, Serialize, Default, Resource)]
pub struct AverageAttributeStatistics {
    pub adult_age: Vec<f32>,
    pub death_age: Vec<f32>,
    pub mutation_probability: Vec<f32>,
    pub translation_speed: Vec<f32>,
    pub rotation_speed: Vec<f32>,
    pub eye_range: Vec<f32>,
    pub eye_angle: Vec<f32>,
    pub internal_timer_boundary: Vec<f32>,
    pub lay_egg_boundary: Vec<f32>,
    pub want_to_grow_boundary: Vec<f32>,
    pub eating_boundary: Vec<f32>,
    pub cost_of_thought: Vec<f32>,
    pub cost_of_eating: Vec<f32>,
    pub offspring_energy: Vec<f32>,
    pub hatch_size: Vec<f32>,
    pub max_size: Vec<f32>,
    pub growth_rate: Vec<f32>,
}

pub fn count_system(
    mut stats: ResMut<CountStatistics>,
    adult_query: Query<&lifecycle::Adult>,
    juvenile_query: Query<&lifecycle::Juvenile>,
    egg_query: Query<&lifecycle::EggEnergy>,
) {
    let adults = adult_query.into_iter().len();
    let juveniles = juvenile_query.into_iter().len();
    let eggs = egg_query.into_iter().len();

    stats.adults.push(adults);
    stats.juveniles.push(juveniles);
    stats.eggs.push(eggs);
}

pub fn energy_stats_system(
    mut stats: ResMut<EnergyStatistics>,
    ecosystem: Res<ecosystem::Ecosystem>,
    food_energy: Query<&ecosystem::Plant>,
) {
    let energy = ecosystem.available_energy();
    let total_food: usize = food_energy.into_iter().map(|x| x.energy().amount()).sum();

    stats.available_energy.push(energy.amount());
    stats.food_energy.push(total_food);
}

pub fn performance_stats_system(
    mut stats: ResMut<BugPerformanceStatistics>,
    performance_query: Query<(
        &eating::EnergyConsumed,
        &lifecycle::EggsLaid,
        &lifecycle::Generation,
        &timers::Age,
    )>,
) {
    let mut max_consumption = eating::EnergyConsumed(0);
    let mut max_eggs = lifecycle::EggsLaid(0);
    let mut max_generation = lifecycle::Generation(0);
    let mut oldest_bug: f32 = 0.0;

    for (energy_consumed, eggs_laid, generation, age) in performance_query.into_iter() {
        max_consumption = max_consumption.max(*energy_consumed);
        max_eggs = max_eggs.max(*eggs_laid);
        max_generation = max_generation.max(*generation);
        oldest_bug = oldest_bug.max(age.elapsed_secs());
    }

    stats.highest_energy_consumed.push(*max_consumption);
    stats.most_eggs_laid.push(*max_eggs);
    stats.max_generation.push(*max_generation);
    stats.oldest_bug.push(oldest_bug);
}

pub fn attribute_stats_system(
    mut stats: ResMut<AverageAttributeStatistics>,
    attribute_query_1: Query<attributes::BugAttributesPart1>,
    attribute_query_2: Query<attributes::BugAttributesPart2>,
    egg_attribute_query: Query<&attributes::HatchAge>,
) {
    let mut adult_ages = vec![];
    let mut death_ages = vec![];
    let mut eye_angles = vec![];
    let mut eye_ranges = vec![];
    let mut max_rotation_rates = vec![];
    let mut max_speeds = vec![];
    let mut mutation_probabilities = vec![];
    let mut offspring_energies = vec![];
    let mut lay_eggs = vec![];
    let mut internal_timers = vec![];
    let mut want_to_grows = vec![];
    let mut eatings = vec![];
    let mut costs_of_thought = vec![];
    let mut costs_of_eating = vec![];
    let mut hatch_ages = vec![];
    let mut max_sizes = vec![];
    let mut growth_rates = vec![];

    for (aa, da, ea, er, mrr, ms, mp, oe, le, it, wtg, e, cot, coe, msz) in attribute_query_1.iter()
    {
        adult_ages.push(**aa);
        death_ages.push(**da);
        eye_angles.push(**ea);
        eye_ranges.push(**er);
        max_rotation_rates.push(mrr.value());
        max_speeds.push(ms.value());
        mutation_probabilities.push(mp.as_float() as f32);
        offspring_energies.push(**oe);
        lay_eggs.push(**le as f32);
        internal_timers.push(**it as f32);
        want_to_grows.push(**wtg as f32);
        eatings.push(**e as f32);
        costs_of_thought.push(**cot);
        costs_of_eating.push(**coe);
        max_sizes.push(**msz);
    }
    for (gr,) in attribute_query_2.iter() {
        growth_rates.push(**gr);
    }
    for ha in egg_attribute_query.iter() {
        hatch_ages.push(**ha);
    }

    stats.adult_age.push(mean(adult_ages));
    stats.death_age.push(mean(death_ages));
    stats.eye_angle.push(mean(eye_angles));
    stats.eye_range.push(mean(eye_ranges));
    stats.rotation_speed.push(mean(max_rotation_rates));
    stats.translation_speed.push(mean(max_speeds));
    stats
        .mutation_probability
        .push(mean(mutation_probabilities));
    stats.offspring_energy.push(mean(offspring_energies));
    stats.lay_egg_boundary.push(mean(lay_eggs));
    stats.internal_timer_boundary.push(mean(internal_timers));
    stats.want_to_grow_boundary.push(mean(want_to_grows));
    stats.eating_boundary.push(mean(eatings));
    stats.cost_of_thought.push(mean(costs_of_thought));
    stats.cost_of_eating.push(mean(costs_of_eating));
    stats.hatch_size.push(mean(hatch_ages));
    stats.max_size.push(mean(max_sizes));
    stats.growth_rate.push(mean(growth_rates));
}
