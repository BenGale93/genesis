use bevy::prelude::{Component, Entity, Resource};
use serde_derive::Serialize;

#[derive(Debug, Component, Serialize, Clone)]
pub struct Relations {
    entity: Entity,
    parent: Option<Entity>,
    children: Vec<Entity>,
}

impl Relations {
    pub const fn new(entity: Entity, parent: Option<Entity>) -> Self {
        Self {
            entity,
            parent,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Entity) {
        self.children.push(child)
    }
}

#[derive(Resource, Serialize, Debug, Default)]
pub struct FamilyTree {
    pub relations: Vec<Relations>,
}
