use std::{collections::HashMap, fs, time::Duration};

use bevy::{
    app::AppExit,
    prelude::{CoreStage, EventReader, Plugin, Query, Res, ResMut, Resource, SystemSet},
    time::Time,
};
use bevy_trait_query::ReadTraits;
use components::{eat, lay, time};
use derive_getters::Getters;
use genesis_components as components;
use genesis_config::WorldConfig;
use genesis_ecosystem as ecosystem;
use genesis_traits::AttributeDisplay;
use iyes_loopless::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{conditions, SimState};

fn last_element<T>(vector: &[T]) -> T
where
    T: Default + Copy,
{
    vector.last().copied().unwrap_or_default()
}

#[derive(Debug, Getters, Default, Resource, Serialize, Deserialize, Clone)]
pub struct CountStats {
    adults: Vec<usize>,
    juveniles: Vec<usize>,
    eggs: Vec<usize>,
}

impl CountStats {
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

#[derive(Debug, Getters, Serialize, Deserialize, Default, Resource, Clone)]
pub struct EnergyStats {
    available_energy: Vec<usize>,
    food_energy: Vec<usize>,
}

impl EnergyStats {
    pub fn current_available_energy(&self) -> usize {
        last_element(&self.available_energy)
    }

    pub fn current_food_energy(&self) -> usize {
        last_element(&self.food_energy)
    }
}

#[derive(Debug, Getters, Serialize, Deserialize, Default, Resource, Clone)]
pub struct BugPerformance {
    highest_energy_consumed: Vec<usize>,
    most_eggs_laid: Vec<usize>,
    max_generation: Vec<usize>,
    oldest_bug: Vec<f32>,
}

impl BugPerformance {
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

pub fn count_system(
    mut stats: ResMut<CountStats>,
    adult_query: Query<&components::Adult>,
    juvenile_query: Query<&components::Juvenile>,
    egg_query: Query<&ecosystem::EggEnergy>,
) {
    let adults = adult_query.into_iter().len();
    let juveniles = juvenile_query.into_iter().len();
    let eggs = egg_query.into_iter().len();

    stats.adults.push(adults);
    stats.juveniles.push(juveniles);
    stats.eggs.push(eggs);
}

pub fn energy_stats_system(
    mut stats: ResMut<EnergyStats>,
    ecosystem: Res<ecosystem::Ecosystem>,
    food_energy: Query<&ecosystem::Food>,
) {
    let energy = ecosystem.available_energy();
    let total_food: usize = food_energy.into_iter().map(|x| x.energy().amount()).sum();

    stats.available_energy.push(energy.amount());
    stats.food_energy.push(total_food);
}

pub fn performance_stats_system(
    mut stats: ResMut<BugPerformance>,
    performance_query: Query<(
        &eat::EnergyConsumed,
        &lay::EggsLaid,
        &components::Generation,
        &time::Age,
    )>,
) {
    let mut max_consumption = eat::EnergyConsumed(0);
    let mut max_eggs = lay::EggsLaid(0);
    let mut max_generation = components::Generation(0);
    let mut oldest_bug: f32 = 0.0;

    for (energy_consumed, eggs_laid, generation, age) in &performance_query {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BugData {
    relations: components::Relations,
    attributes: HashMap<String, f32>,
}

impl BugData {
    fn new(relations: components::Relations, attrs: ReadTraits<dyn AttributeDisplay>) -> Self {
        let mut attributes = HashMap::new();
        for attr in attrs.into_iter() {
            attributes.insert(attr.name().to_string(), attr.value());
        }
        Self {
            relations,
            attributes,
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Debug, Default, Clone)]
pub struct FamilyTree {
    dead_relations: Vec<BugData>,
    active_relations: Vec<BugData>,
}

impl FamilyTree {
    pub fn add_active_relation(
        &mut self,
        relations: &components::Relations,
        attrs: ReadTraits<dyn AttributeDisplay>,
    ) {
        if relations.is_interesting() {
            let bug_data = BugData::new(relations.clone(), attrs);
            self.active_relations.push(bug_data);
        }
    }
    pub fn add_dead_relation(
        &mut self,
        relations: &components::Relations,
        attrs: ReadTraits<dyn AttributeDisplay>,
    ) {
        if relations.is_interesting() {
            let bug_data = BugData::new(relations.clone(), attrs);
            self.dead_relations.push(bug_data);
        }
    }
}

pub fn family_tree_update(
    mut family_tree: ResMut<FamilyTree>,
    relations_query: Query<(&components::Relations, &dyn AttributeDisplay)>,
) {
    for (relation, attrs) in relations_query.into_iter() {
        family_tree.add_active_relation(relation, attrs)
    }
}

#[derive(Debug, Serialize)]
struct RunInfo<'a> {
    time_elapsed: &'a f32,
    run_config: &'a WorldConfig,
    count_stats: &'a CountStats,
    energy_stats: &'a EnergyStats,
    performance_stats: &'a BugPerformance,
    family_tree: &'a FamilyTree,
}

impl<'a> RunInfo<'a> {
    const fn new(
        time_elapsed: &'a f32,
        run_config: &'a WorldConfig,
        count_stats: &'a CountStats,
        energy_stats: &'a EnergyStats,
        performance_stats: &'a BugPerformance,
        family_tree: &'a FamilyTree,
    ) -> Self {
        Self {
            time_elapsed,
            run_config,
            count_stats,
            energy_stats,
            performance_stats,
            family_tree,
        }
    }
}

pub struct SaveStatsEvent;

pub fn save_stats(
    exit_event: EventReader<AppExit>,
    save_stats: EventReader<SaveStatsEvent>,
    time: Res<Time>,
    count_stats: Res<CountStats>,
    energy_stats: Res<EnergyStats>,
    performance_stats: Res<BugPerformance>,
    family_tree: Res<FamilyTree>,
) {
    if !exit_event.is_empty() || !save_stats.is_empty() {
        let time = time.elapsed_seconds();
        let run_info = RunInfo::new(
            &time,
            WorldConfig::global(),
            &count_stats,
            &energy_stats,
            &performance_stats,
            &family_tree,
        );
        let j = serde_json::to_string_pretty(&run_info).unwrap();
        fs::write("./run_data.json", j).expect("Unable to write file.");
    }
}

pub fn global_statistics_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .run_if_not(conditions::is_paused)
        .with_system(count_system)
        .with_system(energy_stats_system)
        .with_system(performance_stats_system)
        .into()
}

pub fn save_stats_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(SimState::Simulation)
        .run_if_not(conditions::is_paused)
        .with_system(save_stats)
        .into()
}

pub struct GenesisStatsPlugin;

impl Plugin for GenesisStatsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SaveStatsEvent>()
            .add_fixed_timestep(Duration::from_secs(1), "stats")
            .add_fixed_timestep_system_set("stats", 0, global_statistics_system_set())
            .add_system_set_to_stage(CoreStage::Last, save_stats_system_set());
    }
}
