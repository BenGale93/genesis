use bevy::{prelude::*, sprite::collide_aabb::collide};

pub fn check_for_collisions(
    new_transform: &Transform,
    current_transforms: &Query<&Transform>,
) -> bool {
    let new_size = new_transform.scale.truncate();
    for transform in current_transforms.iter() {
        let collision = collide(
            new_transform.translation,
            new_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if collision.is_some() {
            return true;
        }
    }
    false
}
