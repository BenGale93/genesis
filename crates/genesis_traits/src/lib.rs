#[bevy_trait_query::queryable]
pub trait BehaviourTracker {
    fn new() -> Self
    where
        Self: Sized;

    fn add_time(&mut self, time: f32, cost: f32);

    fn uint_portion(&mut self) -> usize;
}

#[bevy_trait_query::queryable]
pub trait AttributeDisplay {
    fn value(&self) -> f32;

    fn display(&self) -> String;
}
