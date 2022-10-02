use bevy::prelude::*;

use crate::{attributes::AdultAge, body::Age};

#[derive(Component, Debug)]
pub struct Juvenile;

#[derive(Component, Debug)]
pub struct Adult;

pub fn transition_to_adult_system(
    mut commands: Commands,
    bug_query: Query<(Entity, &Age, &AdultAge), With<Juvenile>>,
) {
    for (entity, age, adult_age) in bug_query.iter() {
        if age.elapsed_secs() > adult_age.value() {
            commands.entity(entity).remove::<Juvenile>().insert(Adult);
        }
    }
}
