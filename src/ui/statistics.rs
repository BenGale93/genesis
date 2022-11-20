use bevy::{
    prelude::{Query, Res, ResMut, Resource},
    time::Time,
};
use derive_getters::Getters;
use serde_derive::Serialize;

use crate::{
    behaviour::{eating, lifecycle},
    ecosystem,
};

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

    pub fn current_organisms(&self) -> usize {
        self.current_adults() + self.current_juveniles() + self.current_eggs()
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
pub struct BugPerformanceStatistics {
    highest_energy_consumed: Vec<usize>,
    most_eggs_laid: Vec<usize>,
    max_generation: Vec<usize>,
}

impl BugPerformanceStatistics {
    pub fn new() -> Self {
        Self {
            highest_energy_consumed: vec![],
            most_eggs_laid: vec![],
            max_generation: vec![],
        }
    }

    pub fn current_highest_energy_consumed(&self) -> usize {
        last_element(&self.highest_energy_consumed)
    }

    pub fn current_most_eggs_laid(&self) -> usize {
        last_element(&self.most_eggs_laid)
    }

    pub fn current_max_generation(&self) -> usize {
        last_element(&self.max_generation)
    }
}

#[derive(Debug, Getters, Serialize, Resource)]
pub struct GlobalStatistics {
    time_elapsed: f64,
    count_stats: CountStatistics,
    energy_stats: EnergyStatistics,
    performance_stats: BugPerformanceStatistics,
}

impl GlobalStatistics {
    pub fn new() -> Self {
        Self {
            time_elapsed: 0.0,
            count_stats: CountStatistics::new(),
            energy_stats: EnergyStatistics::new(),
            performance_stats: BugPerformanceStatistics::new(),
        }
    }

    pub fn count_stats_mut(&mut self) -> &mut CountStatistics {
        &mut self.count_stats
    }

    pub fn energy_stats_mut(&mut self) -> &mut EnergyStatistics {
        &mut self.energy_stats
    }

    pub fn performance_stats_mut(&mut self) -> &mut BugPerformanceStatistics {
        &mut self.performance_stats
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

pub fn performance_stats_system(
    mut global_stats: ResMut<GlobalStatistics>,
    performance_query: Query<(
        &eating::EnergyConsumed,
        &lifecycle::EggsLaid,
        &lifecycle::Generation,
    )>,
) {
    let stats = global_stats.performance_stats_mut();

    let mut max_consumption = eating::EnergyConsumed(0);
    let mut max_eggs = lifecycle::EggsLaid(0);
    let mut max_generation = lifecycle::Generation(0);

    for (energy_consumed, eggs_laid, generation) in performance_query.into_iter() {
        max_consumption = max_consumption.max(*energy_consumed);
        max_eggs = max_eggs.max(*eggs_laid);
        max_generation = max_generation.max(*generation);
    }

    stats.highest_energy_consumed.push(*max_consumption);
    stats.most_eggs_laid.push(*max_eggs);
    stats.max_generation.push(*max_generation);
}

pub fn time_elapsed_system(time: Res<Time>, mut global_stats: ResMut<GlobalStatistics>) {
    global_stats.time_elapsed = time.elapsed_seconds_f64();
}
