use anyhow::Result;
use bevy_ecs::{prelude::Component, reflect::ReflectComponent};
use bevy_reflect::Reflect;
use bevy_render::color::Color;
use derive_more::{Deref, DerefMut};
use genesis_config as config;
use genesis_ecosystem as ecosystem;

use crate::Size;

#[derive(Component, Debug, Deref, DerefMut, Default, Reflect)]
#[reflect(Component)]
pub struct OriginalColor(pub Color);

#[derive(Debug, Deref, DerefMut, Default, Reflect)]
struct EnergyStore(ecosystem::EnergyReserve);

#[derive(Debug, Deref, DerefMut, Default, Reflect)]
struct Health(ecosystem::EnergyReserve);

#[derive(Debug, PartialEq, Eq, Deref, DerefMut, Default, Reflect)]
pub struct CoreReserve(ecosystem::Energy);

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Vitality {
    energy_store: EnergyStore,
    health: Health,
    core_reserve: CoreReserve,
}

impl Vitality {
    pub fn new(size: &Size, mut total_energy: ecosystem::Energy) -> (Self, ecosystem::Energy) {
        let size_uint = size.as_uint();
        let core_energy = total_energy.take_energy(config::CORE_MULTIPLIER * size_uint);
        let core_reserve = CoreReserve(core_energy);

        let health_energy = total_energy.take_energy(config::HEALTH_MULTIPLIER * size_uint);
        let health = Health(
            ecosystem::EnergyReserve::new(health_energy, config::HEALTH_MULTIPLIER * size_uint)
                .unwrap(),
        );

        let energy_limit = config::EnergyLimitConfig::global().energy_limit(size.as_uint());
        let energy_store = EnergyStore(
            ecosystem::EnergyReserve::new(total_energy.take_energy(energy_limit), energy_limit)
                .unwrap(),
        );

        (
            Self {
                energy_store,
                health,
                core_reserve,
            },
            total_energy,
        )
    }

    pub fn energy_store(&self) -> &ecosystem::EnergyReserve {
        &self.energy_store
    }

    pub fn health(&self) -> &ecosystem::EnergyReserve {
        &self.health
    }

    #[must_use]
    pub fn available_space(&self) -> usize {
        self.health().available_space() + self.energy_store().available_space()
    }

    #[must_use]
    pub fn add_energy(&mut self, energy: ecosystem::Energy) -> ecosystem::Energy {
        let remaining_energy = self.health.add_energy(energy);
        self.energy_store.add_energy(remaining_energy)
    }

    #[must_use]
    pub fn take_energy(&mut self, amount: usize) -> ecosystem::Energy {
        let mut taken_energy = self.energy_store.take_energy(amount);
        let still_needed = amount - taken_energy.amount();
        if still_needed > 0 {
            taken_energy = taken_energy + self.health.take_energy(still_needed);
        }
        taken_energy
    }

    pub fn grow(&mut self, amount: usize, new_size: usize) {
        let core_growing_energy = self
            .energy_store
            .take_energy(amount * config::CORE_MULTIPLIER);
        self.core_reserve.add_energy(core_growing_energy);

        let health_growing_energy = self
            .energy_store
            .take_energy(amount * config::HEALTH_MULTIPLIER);
        let new_health_limit = self.health.energy_limit() + amount * config::HEALTH_MULTIPLIER;
        self.health.set_energy_limit(new_health_limit);

        assert!(
            self.health.add_energy(health_growing_energy) == ecosystem::Energy::new_empty(),
            "Tried to grow and couldn't add all the energy to health."
        );

        self.energy_store
            .set_energy_limit(config::EnergyLimitConfig::global().energy_limit(new_size));
    }

    #[must_use]
    pub fn take_all_energy(&mut self) -> ecosystem::Energy {
        let mut returning_energy = self.energy_store.0.take_all_energy();
        returning_energy = returning_energy + self.health.0.take_all_energy();
        returning_energy = returning_energy + self.core_reserve.0.take_all_energy();
        returning_energy
    }
}

pub struct BodyComponentPlugin;

impl bevy_app::Plugin for BodyComponentPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<OriginalColor>()
            .register_type::<EnergyStore>()
            .register_type::<Health>()
            .register_type::<CoreReserve>()
            .register_type::<Vitality>();
    }
}
