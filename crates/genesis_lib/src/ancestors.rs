use bevy::prelude::{Color, Component, Entity, Query, ResMut, Resource};
use genesis_maths::cantor_pairing;
use genesis_util::color;
use serde_derive::Serialize;

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
            color::rgb_to_hex(c.r(), c.g(), c.b()),
        )
    }
}

#[derive(Resource, Serialize, Debug, Default)]
pub struct FamilyTree {
    pub dead_relations: Vec<Relations>,
    pub active_relations: Vec<Relations>,
}

pub fn family_tree_update(mut family_tree: ResMut<FamilyTree>, relations_query: Query<&Relations>) {
    let interesting_relations = relations_query
        .into_iter()
        .cloned()
        .filter(|x| x.is_interesting())
        .collect();

    family_tree.active_relations = interesting_relations;
}
