use bevy::prelude::{Query, ResMut};
use genesis_components::{
    body, eat::EatingSum, grab::GrabbingSum, grow::SizeSum, lay::LayingSum, BurntEnergy,
    MovementSum, ThinkingSum,
};
use genesis_ecosystem as ecosystem;
use genesis_traits::BehaviourTracker;

pub fn energy_return_system(
    mut ecosystem: ResMut<ecosystem::Ecosystem>,
    mut query: Query<(
        &mut body::Vitality,
        &mut ThinkingSum,
        &mut EatingSum,
        &mut LayingSum,
        &mut GrabbingSum,
        &mut MovementSum,
        &mut SizeSum,
        &mut BurntEnergy,
    )>,
) {
    for (
        mut vitality,
        mut thinking_sum,
        mut eating_sum,
        mut laying_sum,
        mut grabbing_sum,
        mut movement_sum,
        mut size_sum,
        mut burnt_energy,
    ) in query.iter_mut()
    {
        macro_rules! shift_energy {
            ($energy_sum:ident) => {
                let cost = $energy_sum.uint_portion();
                if cost >= 1 {
                    burnt_energy.add_energy(vitality.take_energy(cost));
                }
            };
        }
        shift_energy!(thinking_sum);
        shift_energy!(eating_sum);
        shift_energy!(laying_sum);
        shift_energy!(grabbing_sum);
        shift_energy!(movement_sum);
        shift_energy!(size_sum);

        ecosystem.return_energy(burnt_energy.return_energy());
    }
}
