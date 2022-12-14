use bevy::prelude::{Component, Entity, Resource};
use genesis_util::maths;
use serde_derive::Serialize;

#[derive(Debug, Component, Serialize, Clone)]
pub struct Relations {
    entity: u32,
    parent: Option<u32>,
    children: Vec<u32>,
}

impl Relations {
    pub fn new(entity: Entity, parent: Option<Entity>) -> Self {
        let parent = parent.map(|e| maths::cantor_pairing(e.generation(), e.index()));
        Self {
            entity: maths::cantor_pairing(entity.generation(), entity.index()),
            parent,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Entity) {
        self.children
            .push(maths::cantor_pairing(child.generation(), child.index()))
    }

    pub fn is_interesting(&self) -> bool {
        !(self.parent.is_none() && self.children.is_empty())
    }
}

#[derive(Resource, Serialize, Debug, Default)]
pub struct FamilyTree {
    pub relations: Vec<Relations>,
}
