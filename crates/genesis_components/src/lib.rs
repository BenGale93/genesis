use bevy_app::Plugin;
use bevy_ecs::prelude::{Component, Entity};
use bevy_render::color::Color;
use bevy_trait_query::RegisterExt;
use derive_more::{Add, Deref, DerefMut, From};
use genesis_color::rgb_to_hex;
use genesis_config as config;
use genesis_derive::BehaviourTracker;
use genesis_ecosystem::Energy;
use genesis_maths::cantor_pairing;
use genesis_newtype::Weight;
use genesis_traits::BehaviourTracker;
use serde_derive::Serialize;

pub mod body;
pub mod eat;
pub mod grab;
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

#[derive(Component, Debug, BehaviourTracker)]
pub struct TranslationSum(f32);

#[derive(Component, Debug, BehaviourTracker)]
pub struct RotationSum(f32);

#[derive(Component, Debug, BehaviourTracker)]
pub struct ThinkingSum(f32);

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

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        let config_instance = config::WorldConfig::global();

        app.init_resource::<time::SimulationTime>()
            .insert_resource(mind::MindThresholds::new(&config_instance.brain_mutations))
            .add_event::<eat::EatenEvent>()
            .register_component_as::<dyn BehaviourTracker, ThinkingSum>()
            .register_component_as::<dyn BehaviourTracker, TranslationSum>()
            .register_component_as::<dyn BehaviourTracker, RotationSum>()
            .register_component_as::<dyn BehaviourTracker, eat::EatingSum>()
            .register_component_as::<dyn BehaviourTracker, lay::LayingSum>()
            .register_component_as::<dyn BehaviourTracker, grab::GrabbingSum>()
            .register_component_as::<dyn BehaviourTracker, grow::SizeSum>()
            .register_component_as::<dyn BehaviourTracker, grow::GrowingSum>();
    }
}
