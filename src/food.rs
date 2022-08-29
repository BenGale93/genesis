use bevy::prelude::*;

use crate::ecosystem;

#[derive(Component, Debug)]
pub struct Plant {
    energy: ecosystem::Energy,
}

impl Plant {
    pub fn new(energy: ecosystem::Energy) -> Self {
        Self { energy }
    }
}
