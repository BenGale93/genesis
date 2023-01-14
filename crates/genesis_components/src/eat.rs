use bevy_ecs::{
    prelude::{Component, Entity},
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use bevy_time::Stopwatch;
use derive_getters::Getters;
use derive_more::{Deref, DerefMut};
use genesis_attributes::FoodPreference;
use genesis_config as config;
use genesis_derive::BehaviourTracker;
use genesis_ecosystem::{Energy, Food};

use crate::{body::Vitality, meat_as_food, plant_as_food, Size};

#[derive(Component, Debug, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct TryingToEat(pub Stopwatch);

#[derive(Component, Debug, BehaviourTracker, Reflect, Default, Getters)]
#[reflect(Component)]
pub struct EatingSum {
    sum: f32,
    rate: f32,
}

#[derive(
    Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd, Reflect, Default,
)]
#[reflect(Component)]
pub struct EnergyConsumed(pub usize);

#[derive(
    Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd, Reflect, Default,
)]
#[reflect(Component)]
pub struct EnergyDigested(pub usize);

#[derive(
    Component, Copy, Clone, Debug, Deref, Ord, PartialEq, Eq, PartialOrd, Reflect, Default,
)]
#[reflect(Component)]
pub struct DigestionCost(pub usize);

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Eaten;

#[derive(Debug)]
pub struct EatenEvent(pub Entity);

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Stomach {
    intensity: f32,
    capacity: f32,
    plant_matter: Food,
    meat_matter: Food,
}

impl Stomach {
    pub fn new(size: f32) -> Self {
        let plant_matter = plant_as_food(Energy::new_empty());
        let meat_matter = meat_as_food(Energy::new_empty());
        let mut stomach = Self {
            intensity: 0.0,
            capacity: 0.0,
            plant_matter,
            meat_matter,
        };
        stomach.update_capacity(size);
        stomach
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn update_capacity(&mut self, size: f32) {
        self.capacity = size
    }

    pub fn fullness(&self) -> f32 {
        (self.plant_matter.size() + self.meat_matter.size()) / self.capacity
    }

    fn plant_limit(&self) -> usize {
        let meat_energy_density = self.meat_matter.energy_density();
        (self.plant_matter.energy_density()
            * (self.capacity as usize * meat_energy_density - self.meat_matter.energy().amount()))
            / meat_energy_density
    }

    fn meat_limit(&self) -> usize {
        let plant_energy_density = self.plant_matter.energy_density();
        (self.meat_matter.energy_density()
            * (self.capacity as usize * plant_energy_density - self.plant_matter.energy().amount()))
            / plant_energy_density
    }

    pub fn available_plant_space(&self) -> usize {
        self.plant_limit() - self.plant_matter.energy().amount()
    }

    pub fn available_meat_space(&self) -> usize {
        self.meat_limit() - self.meat_matter.energy().amount()
    }

    pub fn eat(&mut self, food: &mut Food, size: &Size) {
        let toughness = food.toughness();
        let food_chunk = ((**size / toughness) * config::EATING_MULTIPLIER).ceil();
        if self.plant_matter.toughness() == toughness {
            let requested_energy = self.available_plant_space().min(food_chunk as usize);
            self.plant_matter
                .add_energy(food.take_energy(requested_energy));
        } else {
            let requested_energy = self.available_meat_space().min(food_chunk as usize);
            self.meat_matter
                .add_energy(food.take_energy(requested_energy));
        }
    }

    pub fn digestion_cost(&self) -> usize {
        ((self.capacity / 5.0) * (1.0 + (self.intensity - self.fullness() - 0.5).clamp(0.0, 0.3)))
            .round() as usize
    }

    pub fn digest(
        &mut self,
        preference: &FoodPreference,
        vitality: &mut Vitality,
    ) -> (Energy, Energy) {
        let mut usable_energy = Energy::new_empty();
        let mut waste_energy = vitality.take_energy(self.digestion_cost());

        let mut energy_extract = self
            .plant_matter
            .take_energy((self.intensity * self.capacity * 10.0) as usize);
        let usable_plant_energy =
            (preference.plant_digestion_efficiency() * energy_extract.amount() as f32) as isize;

        if usable_plant_energy < 0 {
            waste_energy.add_energy(vitality.take_energy(usable_plant_energy.unsigned_abs()));
        } else {
            usable_energy
                .add_energy(energy_extract.take_energy(usable_plant_energy.unsigned_abs()));
        }
        waste_energy.add_energy(energy_extract);

        let mut energy_extract = self
            .meat_matter
            .take_energy((self.intensity * self.capacity * 10.0) as usize);
        let usable_meat_energy =
            (preference.meat_digestion_efficiency() * energy_extract.amount() as f32) as isize;

        if usable_meat_energy < 0 {
            waste_energy.add_energy(vitality.take_energy(usable_meat_energy.unsigned_abs()));
        } else {
            usable_energy.add_energy(energy_extract.take_energy(usable_meat_energy.unsigned_abs()));
        }
        waste_energy.add_energy(energy_extract);

        (usable_energy, waste_energy)
    }
}

pub struct EatComponentPlugin;

impl bevy_app::Plugin for EatComponentPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<TryingToEat>()
            .register_type::<EatingSum>()
            .register_type::<EnergyConsumed>()
            .register_type::<EnergyDigested>()
            .register_type::<DigestionCost>()
            .register_type::<Eaten>()
            .register_type::<Stomach>();
    }
}
