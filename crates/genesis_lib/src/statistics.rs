use bevy::prelude::{Query, Res, ResMut, Resource};
use components::{eat, lay, time};
use derive_getters::Getters;
use genesis_attributes as attributes;
use genesis_components as components;
use genesis_ecosystem as ecosystem;
use genesis_maths::mean;
use serde_derive::Serialize;

fn last_element<T>(vector: &[T]) -> T
where
    T: Default + Copy,
{
    vector.last().copied().unwrap_or_default()
}

#[derive(Debug, Getters, Serialize, Default, Resource)]
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

#[derive(Debug, Getters, Serialize, Default, Resource)]
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

#[derive(Debug, Getters, Serialize, Default, Resource)]
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

#[derive(Debug, Getters, Serialize, Default, Resource)]
pub struct AverageAttributes {
    pub hatch_age: Vec<f32>,
    pub adult_age: Vec<f32>,
    pub death_age: Vec<f32>,
    pub eye_range: Vec<f32>,
    pub eye_angle: Vec<f32>,
    pub internal_timer_boundary: Vec<f32>,
    pub lay_egg_boundary: Vec<f32>,
    pub want_to_grow_boundary: Vec<f32>,
    pub eating_boundary: Vec<f32>,
    pub cost_of_eating: Vec<f32>,
    pub offspring_energy: Vec<f32>,
    pub mouth_width: Vec<f32>,
    pub hatch_size: Vec<f32>,
    pub max_size: Vec<f32>,
    pub growth_rate: Vec<f32>,
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
    food_energy: Query<&ecosystem::Plant>,
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

pub fn attribute_stats_system(
    mut stats: ResMut<AverageAttributes>,
    attribute_query: Query<attributes::BugAttributes>,
) {
    macro_rules! attr_vecs {
        ($attr:ident) => {
            let mut $attr = vec![];
        };
        ($attr:ident, $($attrs:ident), +) => {
            attr_vecs!($attr);
            attr_vecs!($($attrs), +)
        }
    }
    attr_vecs!(
        hatch_age,
        adult_age,
        death_age,
        eye_range,
        eye_angle,
        cost_of_eating,
        offspring_energy,
        mouth_width,
        hatch_size,
        max_size,
        growth_rate
    );

    for (ha, aa, da, er, ea, coe, oe, mw, hs, ms, gr) in attribute_query.iter() {
        hatch_age.push(**ha);
        adult_age.push(**aa);
        death_age.push(**da);
        eye_range.push(**er);
        eye_angle.push(**ea);
        cost_of_eating.push(**coe);
        offspring_energy.push(**oe);
        mouth_width.push(**mw);
        hatch_size.push(**hs);
        max_size.push(**ms);
        growth_rate.push(**gr);
    }

    macro_rules! push_attr {
        ($attr:ident) => {
            stats.$attr.push(mean($attr))
        };
        ($attr:ident, $($attrs:ident), +) => {
            push_attr!($attr);
            push_attr!($($attrs), +)
        }
    }
    push_attr!(
        hatch_age,
        adult_age,
        death_age,
        eye_range,
        eye_angle,
        cost_of_eating,
        offspring_energy,
        mouth_width,
        hatch_size,
        max_size,
        growth_rate
    );
}

#[derive(Resource, Serialize, Debug, Default)]
pub struct FamilyTree {
    pub dead_relations: Vec<components::Relations>,
    pub active_relations: Vec<components::Relations>,
}

pub fn family_tree_update(
    mut family_tree: ResMut<FamilyTree>,
    relations_query: Query<&components::Relations>,
) {
    let interesting_relations = relations_query
        .into_iter()
        .cloned()
        .filter(|x| x.is_interesting())
        .collect();

    family_tree.active_relations = interesting_relations;
}
