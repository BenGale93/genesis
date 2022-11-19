use bevy::{
    prelude::{Query, Res, ResMut, Resource},
    time::Time,
};
use derive_getters::Getters;
use serde_derive::Serialize;

use crate::{behaviour::lifecycle, ecosystem};

fn last_element<T>(vector: &[T]) -> T
where
    T: Default + Copy,
{
    vector.last().copied().unwrap_or_default()
}

#[derive(Debug, Getters, Serialize)]
pub struct CountStatistics {
    adults: Vec<usize>,
    juveniles: Vec<usize>,
    eggs: Vec<usize>,
}

impl CountStatistics {
    pub fn new() -> Self {
        Self {
            adults: vec![],
            juveniles: vec![],
            eggs: vec![],
        }
    }

    pub fn current_adults(&self) -> usize {
        last_element(&self.adults)
    }

    pub fn current_juveniles(&self) -> usize {
        last_element(&self.juveniles)
    }

    pub fn current_eggs(&self) -> usize {
        last_element(&self.eggs)
    }
}

#[derive(Debug, Getters, Serialize)]
pub struct EnergyStatistics {
    available_energy: Vec<usize>,
    food_energy: Vec<usize>,
}

impl EnergyStatistics {
    pub fn new() -> Self {
        Self {
            available_energy: vec![],
            food_energy: vec![],
        }
    }

    pub fn current_available_energy(&self) -> usize {
        last_element(&self.available_energy)
    }

    pub fn current_food_energy(&self) -> usize {
        last_element(&self.food_energy)
    }
}

#[derive(Debug, Getters, Serialize, Resource)]
pub struct GlobalStatistics {
    time_elapsed: f64,
    max_generation: Vec<usize>,
    count_stats: CountStatistics,
    energy_stats: EnergyStatistics,
}

impl GlobalStatistics {
    pub fn new() -> Self {
        Self {
            time_elapsed: 0.0,
            max_generation: vec![],
            count_stats: CountStatistics::new(),
            energy_stats: EnergyStatistics::new(),
        }
    }

    pub fn current_max_generation(&self) -> usize {
        last_element(&self.max_generation)
    }

    pub fn count_stats_mut(&mut self) -> &mut CountStatistics {
        &mut self.count_stats
    }

    pub fn energy_stats_mut(&mut self) -> &mut EnergyStatistics {
        &mut self.energy_stats
    }
}

pub fn count_system(
    mut global_stats: ResMut<GlobalStatistics>,
    adult_query: Query<&lifecycle::Adult>,
    juvenile_query: Query<&lifecycle::Juvenile>,
    egg_query: Query<&lifecycle::EggEnergy>,
) {
    let stats = global_stats.count_stats_mut();

    let adults = adult_query.into_iter().len();
    let juveniles = juvenile_query.into_iter().len();
    let eggs = egg_query.into_iter().len();

    stats.adults.push(adults);
    stats.juveniles.push(juveniles);
    stats.eggs.push(eggs);
}

pub fn max_generation_system(
    mut stats: ResMut<GlobalStatistics>,
    generation_query: Query<&lifecycle::Generation>,
) {
    let max_generation = generation_query
        .into_iter()
        .max()
        .unwrap_or(&lifecycle::Generation(0));

    stats.max_generation.push(**max_generation)
}

pub fn energy_stats_system(
    mut global_stats: ResMut<GlobalStatistics>,
    ecosystem: Res<ecosystem::Ecosystem>,
    food_energy: Query<&ecosystem::Plant>,
) {
    let stats = global_stats.energy_stats_mut();

    let energy = ecosystem.available_energy();
    let total_food: usize = food_energy.into_iter().map(|x| x.energy().amount()).sum();

    stats.available_energy.push(energy.amount());
    stats.food_energy.push(total_food);
}

pub fn time_elapsed_system(time: Res<Time>, mut global_stats: ResMut<GlobalStatistics>) {
    global_stats.time_elapsed = time.elapsed_seconds_f64();
}
