use bevy_ecs::prelude::{Component, Entity};
use bevy_render::color::Color;
use derive_more::{Add, Deref, DerefMut, From};
use genesis_color::rgb_to_hex;
use genesis_config as config;
use genesis_ecosystem::Energy;
use genesis_maths::cantor_pairing;
use genesis_newtype::Weight;
use serde_derive::Serialize;

pub mod body;
pub mod eat;
pub mod grow;
pub mod lay;
pub mod mind;
pub mod see;
pub mod time;

#[derive(Component, Debug, PartialEq, Eq, Deref, DerefMut, From, Add)]
pub struct BurntEnergy(Energy);

impl BurntEnergy {
    pub const fn new() -> Self {
        Self(Energy::new_empty())
    }
}

impl BurntEnergy {
    pub fn return_energy(&mut self) -> Energy {
        let amount = self.amount();
        self.take_energy(amount)
    }
}

#[derive(Component, Debug)]
pub struct MovementSum {
    translation_sum: f32,
    rotation_sum: f32,
}

impl MovementSum {
    pub const fn new() -> Self {
        Self {
            translation_sum: 0.0,
            rotation_sum: 0.0,
        }
    }

    pub fn uint_portion(&mut self) -> usize {
        let tran_floor = self.translation_sum.floor();
        self.translation_sum -= tran_floor;

        let rot_floor = self.rotation_sum.floor();
        self.rotation_sum -= rot_floor;

        (tran_floor + rot_floor) as usize
    }

    pub fn add_translation(&mut self, translation: f32, translation_cost: f32) {
        self.translation_sum += translation.abs() * translation_cost;
    }
    pub fn add_rotation(&mut self, rotation: f32, rotation_cost: f32) {
        self.rotation_sum += rotation.abs() * rotation_cost;
    }
}

#[derive(Component, Debug)]
pub struct ThinkingSum(f32);

impl ThinkingSum {
    pub const fn new() -> Self {
        Self(0.0)
    }

    pub fn add_thought(&mut self, synapses: usize, cost: f32) {
        self.0 += synapses as f32 * cost;
    }

    pub fn uint_portion(&mut self) -> usize {
        let thought_floor = self.0.floor();
        self.0 -= thought_floor;

        thought_floor as usize
    }
}

#[derive(Component, Debug)]
pub struct Egg;

#[derive(Component, Debug)]
pub struct Hatching;

#[derive(Component, Debug)]
pub struct Juvenile;

#[derive(Component, Debug)]
pub struct Adult;

#[derive(
    Component, Debug, Deref, DerefMut, Clone, Copy, From, Add, Ord, PartialOrd, Eq, PartialEq,
)]
pub struct Generation(pub usize);

#[derive(Debug, Component, Serialize, Clone)]
pub struct Relations {
    entity: (u32, String),
    parent: Option<u32>,
    children: Vec<u32>,
}

impl Relations {
    pub fn new(entity: (Entity, Color), parent: Option<Entity>) -> Self {
        let parent = parent.map(|e| cantor_pairing(e.generation(), e.index()));
        Self {
            entity: Self::convert(entity),
            parent,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Entity) {
        self.children
            .push(cantor_pairing(child.generation(), child.index()))
    }

    pub fn is_interesting(&self) -> bool {
        !(self.parent.is_none() && self.children.is_empty())
    }

    fn convert(input: (Entity, Color)) -> (u32, String) {
        let (e, c) = input;
        (
            cantor_pairing(e.generation(), e.index()),
            rgb_to_hex(c.r(), c.g(), c.b()),
        )
    }
}

#[derive(Component, Debug, Deref)]
pub struct SizeMultiplier(Weight);

impl SizeMultiplier {
    pub fn new(size: f32) -> Self {
        Self(Self::compute_multiplier(size))
    }

    pub fn update(&mut self, size: f32) {
        self.0 = Self::compute_multiplier(size);
    }

    fn compute_multiplier(size: f32) -> Weight {
        let world_config = config::WorldConfig::global();
        let min_size = world_config.dependent_attributes.hatch_size_bounds.0;
        let max_size = world_config.attributes.max_size.1;
        let range = max_size - min_size;
        Weight::new(((max_size - size) / range).powf(1.4))
            .expect("Expected size multiplier to be a valid weight.")
    }

    pub fn as_float(&self) -> f32 {
        self.0.as_float()
    }
}
