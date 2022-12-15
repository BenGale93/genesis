use bevy::prelude::{Color, Component, Entity, Resource};
use genesis_util::{color, maths};
use serde_derive::Serialize;

#[derive(Debug, Component, Serialize, Clone)]
pub struct Relations {
    entity: (u32, String),
    parent: Option<(u32, String)>,
    children: Vec<(u32, String)>,
}

impl Relations {
    pub fn new(entity: (Entity, Color), parent: Option<(Entity, Color)>) -> Self {
        let parent = parent.map(Self::convert);
        Self {
            entity: Self::convert(entity),
            parent,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: (Entity, Color)) {
        self.children.push(Self::convert(child))
    }

    pub fn is_interesting(&self) -> bool {
        !(self.parent.is_none() && self.children.is_empty())
    }

    fn convert(input: (Entity, Color)) -> (u32, String) {
        let (e, c) = input;
        (
            maths::cantor_pairing(e.generation(), e.index()),
            color::rgb_to_hex(c.r(), c.g(), c.b()),
        )
    }
}

#[derive(Resource, Serialize, Debug, Default)]
pub struct FamilyTree {
    pub relations: Vec<Relations>,
}
