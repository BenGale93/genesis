use bevy_app::Plugin;
use bevy_ecs::{
    prelude::{Component, Entity},
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use bevy_render::color::Color;
use bevy_trait_query::RegisterExt;
use derive_more::{Add, Deref, DerefMut, From};
use genesis_color::rgb_to_hex;
use genesis_config as config;
use genesis_derive::BehaviourTracker;
use genesis_ecosystem::Energy;
use genesis_maths::cantor_pairing;
use genesis_newtype::{Probability, Weight};
use genesis_traits::BehaviourTracker;
use serde::Deserialize;
use serde_derive::Serialize;

pub mod body;
pub mod eat;
pub mod grab;
pub mod grow;
pub mod lay;
pub mod mind;
pub mod see;
pub mod time;

#[derive(Component, Debug, PartialEq, Eq, Deref, DerefMut, From, Add, Reflect, Default)]
#[reflect(Component)]
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

#[derive(Component, Debug, BehaviourTracker, Reflect, Default)]
#[reflect(Component)]
pub struct TranslationSum(f32);

#[derive(Component, Debug, BehaviourTracker, Reflect, Default)]
#[reflect(Component)]
pub struct RotationSum(f32);

#[derive(Component, Debug, BehaviourTracker, Reflect, Default)]
#[reflect(Component)]
pub struct ThinkingSum(f32);

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Egg;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Hatching;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Juvenile;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Adult;

#[derive(
    Component,
    Debug,
    Deref,
    DerefMut,
    Clone,
    Copy,
    From,
    Add,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Reflect,
    Default,
)]
#[reflect(Component)]
pub struct Generation(pub usize);

#[derive(Debug, Component, Serialize, Deserialize, Clone, Reflect, Default)]
#[reflect(Component)]
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

#[derive(Component, Debug, Deref, Reflect, Default)]
#[reflect(Component)]
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
        app.add_event::<eat::EatenEvent>()
            .add_plugin(eat::EatComponentPlugin)
            .add_plugin(body::BodyComponentPlugin)
            .add_plugin(grab::GrabComponentPlugin)
            .add_plugin(grow::GrowComponentPlugin)
            .add_plugin(lay::LayComponentPlugin)
            .add_plugin(mind::MindComponentPlugin)
            .add_plugin(time::TimeComponentPlugin)
            .register_type::<Weight>()
            .register_type::<Probability>()
            .register_type::<Option<u32>>()
            .register_type::<Vec<f32>>()
            .register_type::<Vec<u32>>()
            .register_type::<(u32, String)>()
            .register_type::<genesis_brain::Brain>()
            .register_type::<genesis_brain::Neuron>()
            .register_type::<Vec<genesis_brain::Neuron>>()
            .register_type::<genesis_brain::NeuronKind>()
            .register_type::<genesis_brain::ActivationFunctionKind>()
            .register_type::<genesis_brain::Synapse>()
            .register_type::<Vec<genesis_brain::Synapse>>()
            .register_type::<see::Vision>()
            .register_type::<BurntEnergy>()
            .register_type::<TranslationSum>()
            .register_type::<RotationSum>()
            .register_type::<ThinkingSum>()
            .register_type::<Egg>()
            .register_type::<Hatching>()
            .register_type::<Juvenile>()
            .register_type::<Adult>()
            .register_type::<Generation>()
            .register_type::<Relations>()
            .register_type::<SizeMultiplier>()
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
